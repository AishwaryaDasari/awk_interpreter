package parser

import (
	"fmt"
	"strconv"

	"awk-go/lexer"
)

// ── AST node interface ────────────────────────────────────────────────────────

// Node is the common interface for all AST nodes.
type Node interface {
	nodeType() string
}

// ── Top-level program ─────────────────────────────────────────────────────────

// Program is the root node: a list of rules.
// Each rule is a pattern + action pair.
type Program struct {
	Rules []*Rule
}

func (n *Program) nodeType() string { return "program" }

// Rule represents one `pattern { action }` pair.
// Pattern is one of: "always", "begin", "end", "regex".
// For regex patterns, Regex holds the pattern string.
type Rule struct {
	Pattern string // "always" | "begin" | "end" | "regex"
	Regex   string // set when Pattern == "regex"
	Body    []Node // list of statements
}

func (n *Rule) nodeType() string { return "rule" }

// ── Statement nodes ───────────────────────────────────────────────────────────

type PrintNode struct {
	Args []Node
}

func (n *PrintNode) nodeType() string { return "print" }

// ── Expression nodes ──────────────────────────────────────────────────────────

// FieldExprNode represents $(expr) — field number determined at runtime.
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

func (p *Parser) advance() {
	if p.pos < len(p.tokens) {
		p.pos++
	}
}

// skipTerminators skips newlines and semicolons between statements/rules.
func (p *Parser) skipTerminators() {
	for p.current().Type == lexer.NEWLINE || p.current().Type == lexer.SEMICOLON {
		p.advance()
	}
}

// Parse parses the full program and returns a Program node.
// It supports three forms:
//
//	BEGIN { stmts }
//	END   { stmts }
//	{ stmts }          ← action-only, runs on every line
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
	var pattern string
	var regex string

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
		// don't advance — parseBlock will consume the brace
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

// parseBlock parses { stmt; stmt; ... }
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

func (p *Parser) parseStmt() (Node, error) {
	switch p.current().Type {
	case lexer.PRINT:
		return p.parsePrint()
	default:
		return nil, fmt.Errorf("unexpected token %q in statement", p.current().Value)
	}
}

func (p *Parser) parsePrint() (Node, error) {
	p.advance() // consume 'print'

	var args []Node

	// print with no args → print $0
	if p.current().Type == lexer.NEWLINE ||
		p.current().Type == lexer.SEMICOLON ||
		p.current().Type == lexer.RBRACE ||
		p.current().Type == lexer.EOF {
		return &PrintNode{Args: args}, nil
	}

	// optional parens: print($1) or print $1
	parens := p.current().Type == lexer.LPAREN
	if parens {
		p.advance()
	}

	for {
		arg := p.parseExpr()
		args = append(args, arg)
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

	// consume optional statement terminator
	if p.current().Type == lexer.SEMICOLON || p.current().Type == lexer.NEWLINE {
		p.advance()
	}

	return &PrintNode{Args: args}, nil
}

func (p *Parser) parseExpr() Node {
	tok := p.current()

	switch tok.Type {
	case lexer.FIELD:
		p.advance()
		inner := p.parseExpr()
		return &FieldExprNode{Inner: inner}

	case lexer.STRING:
		p.advance()
		return &StrNode{Value: tok.Value}

	case lexer.NUMBER:
		val, _ := strconv.ParseFloat(tok.Value, 64)
		p.advance()
		return &NumNode{Value: val}

	case lexer.IDENT, lexer.NR, lexer.NF:
		p.advance()
		return &IdentNode{Name: tok.Value}

	default:
		p.advance()
		return &StrNode{Value: ""}
	}
}
