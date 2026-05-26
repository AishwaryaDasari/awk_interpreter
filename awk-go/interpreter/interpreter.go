package interpreter

import (
	"bufio"
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

// String returns the display representation.
// Whole numbers print without a decimal point.
func (v Value) String() string {
	if v.IsNum {
		if v.Num == float64(int64(v.Num)) {
			return strconv.FormatInt(int64(v.Num), 10)
		}
		return strconv.FormatFloat(v.Num, 'f', -1, 64)
	}
	return v.Str
}

// Float coerces the value to float64.
func (v Value) Float() float64 {
	if v.IsNum {
		return v.Num
	}
	f, _ := strconv.ParseFloat(strings.TrimSpace(v.Str), 64)
	return f
}

// ── Interpreter ───────────────────────────────────────────────────────────────

// Interpreter executes a parsed AWK program against input text.
type Interpreter struct {
	variables map[string]Value
	fields    []string // [0] = $0, [1] = $1, ...
	nr        int      // current record number (NR)
	output    strings.Builder
}

func New() *Interpreter {
	return &Interpreter{
		variables: make(map[string]Value),
	}
}

// Run executes the program against the full input string and returns all output.
//
// Execution order:
//  1. All BEGIN rules — once, before any input is read
//  2. All "always" rules — once per input line
//  3. All END rules — once, after all input is consumed
func (i *Interpreter) Run(prog *parser.Program, input string) string {
	i.output.Reset()

	// 1. BEGIN rules
	for _, rule := range prog.Rules {
		if rule.Pattern == "begin" {
			i.execStmts(rule.Body)
		}
	}

	// 2. Per-line rules
	scanner := bufio.NewScanner(strings.NewReader(input))
	for scanner.Scan() {
		i.setLine(scanner.Text())
		for _, rule := range prog.Rules {
			switch rule.Pattern {
			case "always":
				i.execStmts(rule.Body)
			case "regex":
				if i.matchesRegex(rule.Regex, i.getField(0).String()) {
					i.execStmts(rule.Body)
				}
			}
		}
	}

	// 3. END rules
	for _, rule := range prog.Rules {
		if rule.Pattern == "end" {
			i.execStmts(rule.Body)
		}
	}

	return i.output.String()
}

// setLine splits an input line into fields and increments NR.
// Strips trailing \r to handle Windows CRLF line endings transparently.
func (i *Interpreter) setLine(line string) {
	i.nr++
	line = strings.TrimRight(line, "\r")
	parts := strings.Fields(line)
	i.fields = make([]string, 0, len(parts)+1)
	i.fields = append(i.fields, line)     // $0
	i.fields = append(i.fields, parts...) // $1, $2, ...
}

func (i *Interpreter) getField(n int) Value {
	if n >= 0 && n < len(i.fields) {
		return Value{Str: i.fields[n]}
	}
	return Value{Str: ""}
}

func (i *Interpreter) getVar(name string) Value {
	switch name {
	case "NR":
		return Value{Num: float64(i.nr), IsNum: true}
	case "NF":
		nf := len(i.fields) - 1
		if nf < 0 {
			nf = 0
		}
		return Value{Num: float64(nf), IsNum: true}
	default:
		if v, ok := i.variables[name]; ok {
			return v
		}
		return Value{Str: ""}
	}
}

// execStmts runs a list of statements.
func (i *Interpreter) execStmts(stmts []parser.Node) {
	for _, stmt := range stmts {
		i.execNode(stmt)
	}
}

func (i *Interpreter) execNode(node parser.Node) {
	switch n := node.(type) {
	case *parser.PrintNode:
		i.execPrint(n)
	}
}

func (i *Interpreter) execPrint(node *parser.PrintNode) {
	// print with no args → print $0
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

func (i *Interpreter) evalExpr(node parser.Node) Value {
	switch n := node.(type) {
	case *parser.FieldExprNode:
		v := i.evalExpr(n.Inner)
		return i.getField(int(v.Float()))

	case *parser.IdentNode:
		return i.getVar(n.Name)

	case *parser.StrNode:
		return Value{Str: n.Value}

	case *parser.NumNode:
		return Value{Num: n.Value, IsNum: true}

	default:
		return Value{Str: ""}
	}
}

// matchesRegex returns true if s matches the given regex pattern.
// Compile errors are treated as no-match.
func (i *Interpreter) matchesRegex(pattern, s string) bool {
	re, err := regexp.Compile(pattern)
	if err != nil {
		return false
	}
	return re.MatchString(s)
}
