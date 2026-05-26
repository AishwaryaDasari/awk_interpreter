package interpreter

import (
	"bufio"
	"strconv"
	"strings"

	"awk-go/parser"
)

// Value represents an AWK value that can be either a string or a number.
// IsNum indicates whether the canonical representation is numeric.
type Value struct {
	Str   string
	Num   float64
	IsNum bool
}

// String returns the string representation of the value.
// Whole numbers are formatted without a decimal point.
func (v Value) String() string {
	if v.IsNum {
		if v.Num == float64(int64(v.Num)) {
			return strconv.FormatInt(int64(v.Num), 10)
		}
		return strconv.FormatFloat(v.Num, 'f', -1, 64)
	}
	return v.Str
}

// Float converts the value to float64, parsing strings on demand.
func (v Value) Float() float64 {
	if v.IsNum {
		return v.Num
	}
	f, _ := strconv.ParseFloat(v.Str, 64)
	return f
}

// Interpreter walks the AST and executes AWK statements against input lines.
type Interpreter struct {
	variables map[string]Value // user-defined variables
	fields    []string         // fields[0] = $0 (full line), fields[1] = $1, ...
	output    strings.Builder
	nr        int // current record number
}

func New() *Interpreter {
	return &Interpreter{
		variables: make(map[string]Value),
	}
}

// Run executes the given AST nodes against every line in input.
func (i *Interpreter) Run(nodes []parser.Node, input string) string {
	i.output.Reset()
	scanner := bufio.NewScanner(strings.NewReader(input))
	for scanner.Scan() {
		i.setLine(scanner.Text(), " ")
		for _, node := range nodes {
			i.execNode(node)
		}
	}
	return i.output.String()
}

func (i *Interpreter) setLine(line, sep string) {
	i.nr++
	var parts []string
	if sep == " " {
		parts = strings.Fields(line)
	} else {
		parts = strings.Split(line, sep)
	}
	i.fields = make([]string, 0, len(parts)+1)
	i.fields = append(i.fields, line)  // $0
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
		n := len(i.fields) - 1
		if n < 0 {
			n = 0
		}
		return Value{Num: float64(n), IsNum: true}
	default:
		if v, ok := i.variables[name]; ok {
			return v
		}
		return Value{Num: 0, IsNum: true}
	}
}

func (i *Interpreter) execNode(node parser.Node) Value {
	switch n := node.(type) {
	case *parser.PrintNode:
		return i.execPrint(n)
	case *parser.ProgramNode:
		for _, stmt := range n.Statements {
			i.execNode(stmt)
		}
		return Value{}
	default:
		return i.evalExpr(node)
	}
}

func (i *Interpreter) execPrint(node *parser.PrintNode) Value {
	if len(node.Args) == 0 {
		i.output.WriteString(i.getField(0).String())
		i.output.WriteByte('\n')
		return Value{}
	}

	var parts []string
	for _, arg := range node.Args {
		parts = append(parts, i.evalExpr(arg).String())
	}
	for idx, part := range parts {
		if idx > 0 {
			i.output.WriteByte(' ')
		}
		i.output.WriteString(part)
	}
	i.output.WriteByte('\n')
	return Value{}
}

func (i *Interpreter) evalExpr(node parser.Node) Value {
	switch n := node.(type) {
	case *parser.FieldNode:
		return i.getField(n.Index)

	case *parser.FieldExprNode:
		v := i.evalExpr(n.Inner)
		idx := int(v.Float())
		return i.getField(idx)

	case *parser.IdentNode:
		return i.getVar(n.Name)

	case *parser.StrNode:
		return Value{Str: n.Value}

	case *parser.NumNode:
		return Value{Num: n.Value, IsNum: true}

	default:
		return Value{}
	}
}
