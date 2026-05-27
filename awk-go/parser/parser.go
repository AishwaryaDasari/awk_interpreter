package parser

import (
	"fmt"
	"strconv"

	"awk-go/lexer"
)

// ── AST node interface ────────────────────────────────────────────────────────

type Node interface {
	nodeType() string
}

// ── Top-level ─────────────────────────────────────────────────────────────────

type Program struct {
	Rules []*Rule
}

func (n *Program) nodeType() string { return "program" }

type Rule struct {
	Pattern string // "always" | "begin" | "end" | "regex"
	Regex   string
	Body    []Node
}

func (n *Rule) nodeType() string { return "rule" }

// ── Statements ────────────────────────────────────────────────────────────────

type PrintNode struct {
	Args []Node
}

func (n *PrintNode) nodeType() string { return "print" }

// ExprStmt wraps an expression used as a statement (assignment, increment).
type ExprStmt struct {
	Expr Node
}

func (n *ExprStmt) nodeType() string { return "exprstmt" }

// ── Expressions ───────────────────────────────────────────────────────────────

type FieldExprNode struct {
	Inner Node
}

func (n *FieldExprNode) nodeType() string { return "fieldexpr" }

type IdentNode struct {
	Name string
}

func (n *IdentNode) nodeType() string { return "ident" }

type StrNode struct {
	Value string
}

func (n *StrNode) nodeType() string { return "str" }

type NumNode struct {
	Value float64
}

func (n *NumNode) nodeType() string { return "num" }

// BinOpNode: left op right
// op is one of: + - * / % == != < <= > >= && || (concat)
type BinOpNode struct {
	Op    string
	Left  Node
	Right Node
}

func (n *BinOpNode) nodeType() string { return "binop" }

// UnaryNode: op expr  (op is "-" or "!")
type UnaryNode struct {
	Op   string
	Expr Node
}

func (n *UnaryNode) nodeType() string { return "unary" }

// AssignNode: name = expr
type AssignNode struct {
	Name  string
	Value Node
}

func (n *AssignNode) nodeType() string { return "assign" }

// AssignOpNode: name op= expr  (e.g. x += 1)
type AssignOpNode struct {
	Op    string // "+" "-" "*" "/" "%"
	Name  string
	Value Node
}

func (n *AssignOpNode) nodeType() string { return "assignop" }

// IncrNode: x++ / x-- / ++x / --x
type IncrNode struct {
	Name  string
	Delta int  // +1 or -1
	Post  bool // true = postfix (x++), false = prefix (++x)
}

func (n *IncrNode) nodeType() string { return "incr" }

// ── Parser ────────────────────────────────────────────────────────────────────

type Parser struct {
	tokens []lexer.Token
	pos    int
}

func New(tokens []lexer.Token) *Parser {
	return &Parser{tokens: tokens, pos: 0}
}

func (p *Parser) current() lexer.Token {
	if p.pos >= len(p.tokens) {
		return lexer.Token{Type: lexer.EOF}
	}
	return p.tokens[p.pos]
}

func (p *Parser) peek() lexer.Token {
	if p.pos+1 >= len(p.tokens) {
		return lexer.Token{Type: lexer.EOF}
	}
	return p.tokens[p.pos+1]
}

func (p *Parser) advance() lexer.Token {
	tok := p.current()
	if p.pos < len(p.tokens) {
		p.pos++
	}
	return tok
}

func (p *Parser) skipTerminators() {
	for p.current().Type == lexer.NEWLINE || p.current().Type == lexer.SEMICOLON {
		p.advance()
	}
}

// ── Top-level parse ───────────────────────────────────────────────────────────

func (p *Parser) Parse() (*Program, error) {
	prog := &Program{}
	p.skipTerminators()
	for p.current().Type != lexer.EOF {
		rule, err := p.parseRule()
		if err != nil {
			return nil, err
		}
		prog.Rules = append(prog.Rules, rule)
		p.skipTerminators()
	}
	return prog, nil
}

func (p *Parser) parseRule() (*Rule, error) {
	var pattern, regex string

	switch p.current().Type {
	case lexer.BEGIN:
		pattern = "begin"
		p.advance()
	case lexer.END:
		pattern = "end"
		p.advance()
	case lexer.REGEX:
		pattern = "regex"
		regex = p.current().Value
		p.advance()
	case lexer.LBRACE:
		pattern = "always"
	default:
		return nil, fmt.Errorf("unexpected token %q at start of rule", p.current().Value)
	}

	p.skipTerminators()
	body, err := p.parseBlock()
	if err != nil {
		return nil, err
	}
	return &Rule{Pattern: pattern, Regex: regex, Body: body}, nil
}

func (p *Parser) parseBlock() ([]Node, error) {
	if p.current().Type != lexer.LBRACE {
		return nil, fmt.Errorf("expected '{', got %q", p.current().Value)
	}
	p.advance() // consume {
	p.skipTerminators()

	var stmts []Node
	for p.current().Type != lexer.RBRACE && p.current().Type != lexer.EOF {
		stmt, err := p.parseStmt()
		if err != nil {
			return nil, err
		}
		stmts = append(stmts, stmt)
		p.skipTerminators()
	}

	if p.current().Type != lexer.RBRACE {
		return nil, fmt.Errorf("expected '}', got EOF")
	}
	p.advance() // consume }
	return stmts, nil
}

// ── Statements ────────────────────────────────────────────────────────────────

func (p *Parser) parseStmt() (Node, error) {
	switch p.current().Type {
	case lexer.PRINT:
		return p.parsePrint()
	default:
		// Expression statement: assignment, increment, etc.
		expr := p.parseExpr(0)
		if p.current().Type == lexer.SEMICOLON || p.current().Type == lexer.NEWLINE {
			p.advance()
		}
		return &ExprStmt{Expr: expr}, nil
	}
}

func (p *Parser) parsePrint() (Node, error) {
	p.advance() // consume 'print'

	var args []Node
	if p.current().Type == lexer.NEWLINE ||
		p.current().Type == lexer.SEMICOLON ||
		p.current().Type == lexer.RBRACE ||
		p.current().Type == lexer.EOF {
		return &PrintNode{Args: args}, nil
	}

	parens := p.current().Type == lexer.LPAREN
	if parens {
		p.advance()
	}

	for {
		args = append(args, p.parseExpr(0))
		if p.current().Type == lexer.COMMA {
			p.advance()
		} else {
			break
		}
	}

	if parens {
		if p.current().Type != lexer.RPAREN {
			return nil, fmt.Errorf("expected ')' after print args")
		}
		p.advance()
	}

	if p.current().Type == lexer.SEMICOLON || p.current().Type == lexer.NEWLINE {
		p.advance()
	}
	return &PrintNode{Args: args}, nil
}

// ── Expression parser (Pratt / precedence climbing) ───────────────────────────
//
// Precedence levels (higher = tighter binding):
//   1  ||
//   2  &&
//   3  == != < <= > >=
//   4  + -  (and string concat)
//   5  * / %
//   6  unary - !   (handled in parseUnary)
//   7  $ ++ --     (handled in parsePrimary / postfix)

func (p *Parser) parseExpr(minPrec int) Node {
	left := p.parseUnary()

	for {
		op, prec, ok := p.infixOp()
		if !ok || prec < minPrec {
			break
		}
		p.advance() // consume operator
		right := p.parseExpr(prec + 1)
		left = &BinOpNode{Op: op, Left: left, Right: right}
	}

	// Check for assignment after we have the left side
	switch p.current().Type {
	case lexer.ASSIGN:
		p.advance()
		name := identName(left)
		val := p.parseExpr(0)
		return &AssignNode{Name: name, Value: val}
	case lexer.PLUS_ASSIGN:
		p.advance()
		name := identName(left)
		val := p.parseExpr(0)
		return &AssignOpNode{Op: "+", Name: name, Value: val}
	case lexer.MINUS_ASSIGN:
		p.advance()
		name := identName(left)
		val := p.parseExpr(0)
		return &AssignOpNode{Op: "-", Name: name, Value: val}
	case lexer.STAR_ASSIGN:
		p.advance()
		name := identName(left)
		val := p.parseExpr(0)
		return &AssignOpNode{Op: "*", Name: name, Value: val}
	case lexer.SLASH_ASSIGN:
		p.advance()
		name := identName(left)
		val := p.parseExpr(0)
		return &AssignOpNode{Op: "/", Name: name, Value: val}
	case lexer.PERCENT_ASSIGN:
		p.advance()
		name := identName(left)
		val := p.parseExpr(0)
		return &AssignOpNode{Op: "%", Name: name, Value: val}
	case lexer.INCR:
		p.advance()
		name := identName(left)
		return &IncrNode{Name: name, Delta: 1, Post: true}
	case lexer.DECR:
		p.advance()
		name := identName(left)
		return &IncrNode{Name: name, Delta: -1, Post: true}
	}

	return left
}

// infixOp returns the operator string, its precedence, and whether the current
// token is a binary infix operator.
func (p *Parser) infixOp() (string, int, bool) {
	switch p.current().Type {
	case lexer.OR:
		return "||", 1, true
	case lexer.AND:
		return "&&", 2, true
	case lexer.EQ:
		return "==", 3, true
	case lexer.NEQ:
		return "!=", 3, true
	case lexer.LT:
		return "<", 3, true
	case lexer.LE:
		return "<=", 3, true
	case lexer.GT:
		return ">", 3, true
	case lexer.GE:
		return ">=", 3, true
	case lexer.PLUS:
		return "+", 4, true
	case lexer.MINUS:
		return "-", 4, true
	case lexer.STAR:
		return "*", 5, true
	case lexer.SLASH:
		return "/", 5, true
	case lexer.PERCENT:
		return "%", 5, true
	}
	// String concatenation: two adjacent value-producing tokens
	switch p.current().Type {
	case lexer.STRING, lexer.NUMBER, lexer.IDENT, lexer.NR, lexer.NF, lexer.FIELD, lexer.LPAREN:
		return "concat", 4, true
	}
	return "", 0, false
}

func (p *Parser) parseUnary() Node {
	switch p.current().Type {
	case lexer.MINUS:
		p.advance()
		expr := p.parseUnary()
		return &UnaryNode{Op: "-", Expr: expr}
	case lexer.NOT:
		p.advance()
		expr := p.parseUnary()
		return &UnaryNode{Op: "!", Expr: expr}
	case lexer.INCR:
		p.advance()
		inner := p.parsePrimary()
		name := identName(inner)
		return &IncrNode{Name: name, Delta: 1, Post: false}
	case lexer.DECR:
		p.advance()
		inner := p.parsePrimary()
		name := identName(inner)
		return &IncrNode{Name: name, Delta: -1, Post: false}
	case lexer.FIELD:
		p.advance()
		inner := p.parseUnary()
		return &FieldExprNode{Inner: inner}
	}
	return p.parsePrimary()
}

func (p *Parser) parsePrimary() Node {
	tok := p.current()

	switch tok.Type {
	case lexer.NUMBER:
		val, _ := strconv.ParseFloat(tok.Value, 64)
		p.advance()
		return &NumNode{Value: val}

	case lexer.STRING:
		p.advance()
		return &StrNode{Value: tok.Value}

	case lexer.IDENT, lexer.NR, lexer.NF:
		p.advance()
		return &IdentNode{Name: tok.Value}

	case lexer.LPAREN:
		p.advance()
		expr := p.parseExpr(0)
		if p.current().Type == lexer.RPAREN {
			p.advance()
		}
		return expr

	default:
		p.advance()
		return &StrNode{Value: ""}
	}
}

// identName extracts the variable name from an IdentNode.
// Used for the left side of assignments.
func identName(n Node) string {
	if id, ok := n.(*IdentNode); ok {
		return id.Name
	}
	return ""
}
