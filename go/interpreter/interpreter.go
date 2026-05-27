package interpreter

import (
	"bufio"
	"math"
	"regexp"
	"strconv"
	"strings"

	"awk-go/parser"
)

// ── Value ─────────────────────────────────────────────────────────────────────

// Value is an AWK runtime value — simultaneously a string and a number.
type Value struct {
	Str   string
	Num   float64
	IsNum bool
}

func numVal(n float64) Value  { return Value{Num: n, IsNum: true} }
func strVal(s string) Value   { return Value{Str: s} }
func zeroVal() Value          { return Value{Str: ""} }

// String returns the display representation.
func (v Value) String() string {
	if v.IsNum {
		if v.Num == math.Trunc(v.Num) && !math.IsInf(v.Num, 0) {
			return strconv.FormatInt(int64(v.Num), 10)
		}
		return strconv.FormatFloat(v.Num, 'g', 6, 64)
	}
	return v.Str
}

// Float coerces to float64.
func (v Value) Float() float64 {
	if v.IsNum {
		return v.Num
	}
	f, _ := strconv.ParseFloat(strings.TrimSpace(v.Str), 64)
	return f
}

// Truthy: non-zero number or non-empty string.
func (v Value) Truthy() bool {
	if v.IsNum {
		return v.Num != 0
	}
	return v.Str != ""
}

// ── Interpreter ───────────────────────────────────────────────────────────────

type Interpreter struct {
	variables map[string]Value
	fields    []string
	nr        int
	output    strings.Builder
}

func New() *Interpreter {
	return &Interpreter{variables: make(map[string]Value)}
}

// Run executes the program: BEGIN → per-line → END.
func (i *Interpreter) Run(prog *parser.Program, input string) string {
	i.output.Reset()

	// BEGIN
	for _, rule := range prog.Rules {
		if rule.Pattern == "begin" {
			i.execStmts(rule.Body)
		}
	}

	// Per-line
	scanner := bufio.NewScanner(strings.NewReader(input))
	for scanner.Scan() {
		i.setLine(scanner.Text())
		for _, rule := range prog.Rules {
			switch rule.Pattern {
			case "always":
				i.execStmts(rule.Body)
			case "regex":
				if matchRegex(rule.Regex, i.getField(0).String()) {
					i.execStmts(rule.Body)
				}
			}
		}
	}

	// END
	for _, rule := range prog.Rules {
		if rule.Pattern == "end" {
			i.execStmts(rule.Body)
		}
	}

	return i.output.String()
}

func (i *Interpreter) setLine(line string) {
	i.nr++
	line = strings.TrimRight(line, "\r")
	parts := strings.Fields(line)
	i.fields = make([]string, 0, len(parts)+1)
	i.fields = append(i.fields, line)
	i.fields = append(i.fields, parts...)
}

func (i *Interpreter) getField(n int) Value {
	if n >= 0 && n < len(i.fields) {
		return strVal(i.fields[n])
	}
	return zeroVal()
}

func (i *Interpreter) getVar(name string) Value {
	switch name {
	case "NR":
		return numVal(float64(i.nr))
	case "NF":
		nf := len(i.fields) - 1
		if nf < 0 {
			nf = 0
		}
		return numVal(float64(nf))
	default:
		if v, ok := i.variables[name]; ok {
			return v
		}
		return zeroVal()
	}
}

func (i *Interpreter) setVar(name string, v Value) {
	i.variables[name] = v
}

// ── Statement execution ───────────────────────────────────────────────────────

func (i *Interpreter) execStmts(stmts []parser.Node) {
	for _, stmt := range stmts {
		i.execNode(stmt)
	}
}

func (i *Interpreter) execNode(node parser.Node) {
	switch n := node.(type) {
	case *parser.PrintNode:
		i.execPrint(n)
	case *parser.ExprStmt:
		i.evalExpr(n.Expr) // side effects: assignment, increment
	}
}

func (i *Interpreter) execPrint(node *parser.PrintNode) {
	if len(node.Args) == 0 {
		i.output.WriteString(i.getField(0).String())
		i.output.WriteByte('\n')
		return
	}
	for idx, arg := range node.Args {
		if idx > 0 {
			i.output.WriteByte(' ')
		}
		i.output.WriteString(i.evalExpr(arg).String())
	}
	i.output.WriteByte('\n')
}

// ── Expression evaluation ─────────────────────────────────────────────────────

func (i *Interpreter) evalExpr(node parser.Node) Value {
	switch n := node.(type) {

	case *parser.NumNode:
		return numVal(n.Value)

	case *parser.StrNode:
		return strVal(n.Value)

	case *parser.IdentNode:
		return i.getVar(n.Name)

	case *parser.FieldExprNode:
		v := i.evalExpr(n.Inner)
		return i.getField(int(v.Float()))

	case *parser.BinOpNode:
		return i.evalBinOp(n)

	case *parser.UnaryNode:
		return i.evalUnary(n)

	case *parser.AssignNode:
		val := i.evalExpr(n.Value)
		i.setVar(n.Name, val)
		return val

	case *parser.AssignOpNode:
		cur := i.getVar(n.Name)
		rhs := i.evalExpr(n.Value)
		var result Value
		switch n.Op {
		case "+":
			result = numVal(cur.Float() + rhs.Float())
		case "-":
			result = numVal(cur.Float() - rhs.Float())
		case "*":
			result = numVal(cur.Float() * rhs.Float())
		case "/":
			if rhs.Float() == 0 {
				result = numVal(0)
			} else {
				result = numVal(cur.Float() / rhs.Float())
			}
		case "%":
			if rhs.Float() == 0 {
				result = numVal(0)
			} else {
				result = numVal(math.Mod(cur.Float(), rhs.Float()))
			}
		}
		i.setVar(n.Name, result)
		return result

	case *parser.IncrNode:
		cur := i.getVar(n.Name)
		old := cur.Float()
		newVal := numVal(old + float64(n.Delta))
		i.setVar(n.Name, newVal)
		if n.Post {
			return numVal(old) // postfix: return old value
		}
		return newVal // prefix: return new value

	default:
		return zeroVal()
	}
}

func (i *Interpreter) evalBinOp(n *parser.BinOpNode) Value {
	// Short-circuit logical operators
	if n.Op == "&&" {
		if !i.evalExpr(n.Left).Truthy() {
			return numVal(0)
		}
		if !i.evalExpr(n.Right).Truthy() {
			return numVal(0)
		}
		return numVal(1)
	}
	if n.Op == "||" {
		if i.evalExpr(n.Left).Truthy() {
			return numVal(1)
		}
		if i.evalExpr(n.Right).Truthy() {
			return numVal(1)
		}
		return numVal(0)
	}

	left := i.evalExpr(n.Left)
	right := i.evalExpr(n.Right)

	switch n.Op {
	case "+":
		return numVal(left.Float() + right.Float())
	case "-":
		return numVal(left.Float() - right.Float())
	case "*":
		return numVal(left.Float() * right.Float())
	case "/":
		if right.Float() == 0 {
			return numVal(0)
		}
		return numVal(left.Float() / right.Float())
	case "%":
		if right.Float() == 0 {
			return numVal(0)
		}
		return numVal(math.Mod(left.Float(), right.Float()))
	case "==":
		return boolVal(compareValues(left, right) == 0)
	case "!=":
		return boolVal(compareValues(left, right) != 0)
	case "<":
		return boolVal(compareValues(left, right) < 0)
	case "<=":
		return boolVal(compareValues(left, right) <= 0)
	case ">":
		return boolVal(compareValues(left, right) > 0)
	case ">=":
		return boolVal(compareValues(left, right) >= 0)
	case "concat":
		return strVal(left.String() + right.String())
	}
	return zeroVal()
}

func (i *Interpreter) evalUnary(n *parser.UnaryNode) Value {
	v := i.evalExpr(n.Expr)
	switch n.Op {
	case "-":
		return numVal(-v.Float())
	case "!":
		if v.Truthy() {
			return numVal(0)
		}
		return numVal(1)
	}
	return zeroVal()
}

// ── Helpers ───────────────────────────────────────────────────────────────────

func boolVal(b bool) Value {
	if b {
		return numVal(1)
	}
	return numVal(0)
}

// compareValues compares two AWK values.
// If both look numeric, compare as numbers. Otherwise compare as strings.
func compareValues(a, b Value) int {
	af, aIsNum := tryNum(a)
	bf, bIsNum := tryNum(b)
	if aIsNum && bIsNum {
		if af < bf {
			return -1
		}
		if af > bf {
			return 1
		}
		return 0
	}
	as, bs := a.String(), b.String()
	if as < bs {
		return -1
	}
	if as > bs {
		return 1
	}
	return 0
}

func tryNum(v Value) (float64, bool) {
	if v.IsNum {
		return v.Num, true
	}
	f, err := strconv.ParseFloat(strings.TrimSpace(v.Str), 64)
	if err != nil {
		return 0, false
	}
	return f, true
}

func matchRegex(pattern, s string) bool {
	re, err := regexp.Compile(pattern)
	if err != nil {
		return false
	}
	return re.MatchString(s)
}
