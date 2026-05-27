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
	FIELD // $
	COMMA
	SEMICOLON
	NEWLINE
	// Keywords
	PRINT
	BEGIN
	END
	// Built-in variables
	NR
	NF
	// Arithmetic operators
	PLUS    // +
	MINUS   // -
	STAR    // *
	SLASH   // /
	PERCENT // %
	// Assignment operators
	ASSIGN         // =
	PLUS_ASSIGN    // +=
	MINUS_ASSIGN   // -=
	STAR_ASSIGN    // *=
	SLASH_ASSIGN   // /=
	PERCENT_ASSIGN // %=
	INCR           // ++
	DECR           // --
	// Comparison operators
	EQ  // ==
	NEQ // !=
	LT  // <
	LE  // <=
	GT  // >
	GE  // >=
	// Logical operators
	AND // &&
	OR  // ||
	NOT // !
	// Literals
	STRING
	NUMBER
	IDENT
	REGEX // /pattern/
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

func (l *Lexer) peek() rune {
	if l.pos+1 < len(l.input) {
		return l.input[l.pos+1]
	}
	return 0
}

func (l *Lexer) nextToken() Token {
	l.skipSpaces()

	if l.pos >= len(l.input) {
		return Token{Type: EOF}
	}

	ch := l.input[l.pos]

	switch ch {
	case '\n':
		l.pos++
		return Token{Type: NEWLINE, Value: "\n"}
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
	case '+':
		l.pos++
		if l.pos < len(l.input) && l.input[l.pos] == '+' {
			l.pos++
			return Token{Type: INCR, Value: "++"}
		}
		if l.pos < len(l.input) && l.input[l.pos] == '=' {
			l.pos++
			return Token{Type: PLUS_ASSIGN, Value: "+="}
		}
		return Token{Type: PLUS, Value: "+"}
	case '-':
		l.pos++
		if l.pos < len(l.input) && l.input[l.pos] == '-' {
			l.pos++
			return Token{Type: DECR, Value: "--"}
		}
		if l.pos < len(l.input) && l.input[l.pos] == '=' {
			l.pos++
			return Token{Type: MINUS_ASSIGN, Value: "-="}
		}
		return Token{Type: MINUS, Value: "-"}
	case '*':
		l.pos++
		if l.pos < len(l.input) && l.input[l.pos] == '=' {
			l.pos++
			return Token{Type: STAR_ASSIGN, Value: "*="}
		}
		return Token{Type: STAR, Value: "*"}
	case '%':
		l.pos++
		if l.pos < len(l.input) && l.input[l.pos] == '=' {
			l.pos++
			return Token{Type: PERCENT_ASSIGN, Value: "%="}
		}
		return Token{Type: PERCENT, Value: "%"}
	case '=':
		l.pos++
		if l.pos < len(l.input) && l.input[l.pos] == '=' {
			l.pos++
			return Token{Type: EQ, Value: "=="}
		}
		return Token{Type: ASSIGN, Value: "="}
	case '!':
		l.pos++
		if l.pos < len(l.input) && l.input[l.pos] == '=' {
			l.pos++
			return Token{Type: NEQ, Value: "!="}
		}
		return Token{Type: NOT, Value: "!"}
	case '<':
		l.pos++
		if l.pos < len(l.input) && l.input[l.pos] == '=' {
			l.pos++
			return Token{Type: LE, Value: "<="}
		}
		return Token{Type: LT, Value: "<"}
	case '>':
		l.pos++
		if l.pos < len(l.input) && l.input[l.pos] == '=' {
			l.pos++
			return Token{Type: GE, Value: ">="}
		}
		return Token{Type: GT, Value: ">"}
	case '&':
		l.pos++
		if l.pos < len(l.input) && l.input[l.pos] == '&' {
			l.pos++
			return Token{Type: AND, Value: "&&"}
		}
		return Token{Type: ILLEGAL, Value: "&"}
	case '|':
		l.pos++
		if l.pos < len(l.input) && l.input[l.pos] == '|' {
			l.pos++
			return Token{Type: OR, Value: "||"}
		}
		return Token{Type: ILLEGAL, Value: "|"}
	case '"':
		return l.readString()
	case '/':
		return l.readRegex()
	default:
		if ch >= '0' && ch <= '9' {
			return l.readNumber()
		}
		if unicode.IsLetter(ch) || ch == '_' {
			return l.readIdent()
		}
		l.pos++
		return Token{Type: ILLEGAL, Value: string(ch)}
	}
}

func (l *Lexer) skipSpaces() {
	for l.pos < len(l.input) && (l.input[l.pos] == ' ' || l.input[l.pos] == '\t' || l.input[l.pos] == '\r') {
		l.pos++
	}
}

func (l *Lexer) readString() Token {
	l.pos++ // skip opening "
	var buf strings.Builder
	for l.pos < len(l.input) && l.input[l.pos] != '"' {
		if l.input[l.pos] == '\\' && l.pos+1 < len(l.input) {
			l.pos++
			switch l.input[l.pos] {
			case 'n':
				buf.WriteByte('\n')
			case 't':
				buf.WriteByte('\t')
			case '"':
				buf.WriteByte('"')
			case '\\':
				buf.WriteByte('\\')
			default:
				buf.WriteByte('\\')
				buf.WriteRune(l.input[l.pos])
			}
		} else {
			buf.WriteRune(l.input[l.pos])
		}
		l.pos++
	}
	if l.pos < len(l.input) {
		l.pos++
	}
	return Token{Type: STRING, Value: buf.String()}
}

func (l *Lexer) readNumber() Token {
	start := l.pos
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

func (l *Lexer) readRegex() Token {
	l.pos++ // skip opening /
	var buf strings.Builder
	for l.pos < len(l.input) && l.input[l.pos] != '/' {
		if l.input[l.pos] == '\\' && l.pos+1 < len(l.input) {
			l.pos++
			buf.WriteByte('\\')
			buf.WriteRune(l.input[l.pos])
		} else {
			buf.WriteRune(l.input[l.pos])
		}
		l.pos++
	}
	if l.pos < len(l.input) {
		l.pos++
	}
	return Token{Type: REGEX, Value: buf.String()}
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
	case "BEGIN":
		return Token{Type: BEGIN, Value: s}
	case "END":
		return Token{Type: END, Value: s}
	case "NR":
		return Token{Type: NR, Value: s}
	case "NF":
		return Token{Type: NF, Value: s}
	default:
		return Token{Type: IDENT, Value: s}
	}
}
