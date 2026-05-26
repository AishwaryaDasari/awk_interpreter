package lexer

import (
	"strconv"
	"strings"
	"unicode"
)

type TokenType int

const (
	ILLEGAL TokenType = iota
	EOF
	LBRACE
	RBRACE
	LPAREN
	RPAREN
	FIELD  // $
	COMMA
	SEMICOLON
	PRINT  // keyword
	NR     // keyword
	NF     // keyword
	STRING
	NUMBER
	IDENT
)

type Token struct {
	Type  TokenType
	Value string
}

type Lexer struct {
	input []rune
	pos   int
}

func New(input string) *Lexer {
	return &Lexer{
		input: []rune(input),
		pos:   0,
	}
}

func (l *Lexer) Tokenize() []Token {
	var tokens []Token
	for {
		tok := l.nextToken()
		tokens = append(tokens, tok)
		if tok.Type == EOF {
			break
		}
	}
	return tokens
}

func (l *Lexer) nextToken() Token {
	l.skipWhitespace()

	if l.pos >= len(l.input) {
		return Token{Type: EOF}
	}

	ch := l.input[l.pos]

	switch ch {
	case '{':
		l.pos++
		return Token{Type: LBRACE, Value: "{"}
	case '}':
		l.pos++
		return Token{Type: RBRACE, Value: "}"}
	case '(':
		l.pos++
		return Token{Type: LPAREN, Value: "("}
	case ')':
		l.pos++
		return Token{Type: RPAREN, Value: ")"}
	case '$':
		l.pos++
		return Token{Type: FIELD, Value: "$"}
	case ',':
		l.pos++
		return Token{Type: COMMA, Value: ","}
	case ';':
		l.pos++
		return Token{Type: SEMICOLON, Value: ";"}
	case '"':
		return l.readString()
	default:
		if ch >= '0' && ch <= '9' || ch == '-' {
			return l.readNumber()
		}
		if unicode.IsLetter(ch) || ch == '_' {
			return l.readIdent()
		}
		l.pos++
		return Token{Type: IDENT, Value: string(ch)}
	}
}

func (l *Lexer) skipWhitespace() {
	for l.pos < len(l.input) && unicode.IsSpace(l.input[l.pos]) {
		l.pos++
	}
}

func (l *Lexer) readString() Token {
	l.pos++ // skip opening "
	var buf strings.Builder
	for l.pos < len(l.input) && l.input[l.pos] != '"' {
		buf.WriteRune(l.input[l.pos])
		l.pos++
	}
	if l.pos < len(l.input) {
		l.pos++ // skip closing "
	}
	return Token{Type: STRING, Value: buf.String()}
}

func (l *Lexer) readNumber() Token {
	start := l.pos
	if l.input[l.pos] == '-' {
		l.pos++
	}
	for l.pos < len(l.input) && l.input[l.pos] >= '0' && l.input[l.pos] <= '9' {
		l.pos++
	}
	if l.pos < len(l.input) && l.input[l.pos] == '.' {
		l.pos++
		for l.pos < len(l.input) && l.input[l.pos] >= '0' && l.input[l.pos] <= '9' {
			l.pos++
		}
	}
	raw := string(l.input[start:l.pos])
	_, err := strconv.ParseFloat(raw, 64)
	if err != nil {
		raw = "0"
	}
	return Token{Type: NUMBER, Value: raw}
}

func (l *Lexer) readIdent() Token {
	start := l.pos
	for l.pos < len(l.input) && (unicode.IsLetter(l.input[l.pos]) || unicode.IsDigit(l.input[l.pos]) || l.input[l.pos] == '_') {
		l.pos++
	}
	s := string(l.input[start:l.pos])
	switch s {
	case "print":
		return Token{Type: PRINT, Value: s}
	case "NR":
		return Token{Type: NR, Value: s}
	case "NF":
		return Token{Type: NF, Value: s}
	default:
		return Token{Type: IDENT, Value: s}
	}
}
