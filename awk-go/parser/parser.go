package parser

import (
	"strconv"

	"awk-go/lexer"
)

// Node is the common interface for all AST nodes.
type Node interface {
	nodeType() string
}

// --- Statement nodes ---

type ProgramNode struct {
	Statements []Node
}

func (n *ProgramNode) nodeType() string { return "program" }

type PrintNode struct {
	Args []Node
}

func (n *PrintNode) nodeType() string { return "print" }

// --- Expression nodes ---

// FieldNode represents a direct numeric field access ($0, $1, ..., $N)
// produced when the parser sees a literal number after $.
type FieldNode struct {
	Index int
}

func (n *FieldNode) nodeType() string { return "field" }

// FieldExprNode represents a computed field access ($expr), where
// expr is evaluated at runtime to determine the field number.
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

// --- Parser ---

type Parser struct {
	tokens []lexer.Token
	pos    int
}

func New(tokens []lexer.Token) *Parser {
	return &Parser{
		tokens: tokens,
		pos:    0,
	}
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

func (p *Parser) Parse() ([]Node, error) {
	var stmts []Node

	if p.current().Type == lexer.LBRACE {
		p.advance()
	}

	for {
		switch p.current().Type {
		case lexer.RBRACE, lexer.EOF:
			return stmts, nil
		case lexer.PRINT:
			stmts = append(stmts, p.parsePrint())
		default:
			p.advance()
		}
	}
}

func (p *Parser) parsePrint() Node {
	p.advance() // skip PRINT

	var args []Node

	if p.current().Type == lexer.COMMA {
		p.advance()
	}

	for {
		switch p.current().Type {
		case lexer.RBRACE, lexer.EOF, lexer.SEMICOLON:
			return &PrintNode{Args: args}
		case lexer.COMMA:
			p.advance()
		default:
			args = append(args, p.parseExpr())
			if p.current().Type == lexer.COMMA {
				p.advance()
			} else {
				return &PrintNode{Args: args}
			}
		}
	}
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
		return &IdentNode{Name: ""}
	}
}
