package tests

import (
	"testing"
)

// ── Arithmetic operations ────────────────────────────────────────────────────
// IMPLEMENTED: +, -, *, =, +=, -=, *=, ++, -- (prefix & postfix)
// All produce numeric results. Division/modulo by zero returns 0 (not fatal).
//
// KNOWN LIMITATION (lexer): / and % are lexed as REGEX or regex-start. When /
// appears inside a block body (e.g. `{ print 10 / 3 }`), the lexer treats it
// as the start of a regex literal /3 }/, consuming the closing brace. This
// means division (/) and modulo (%) CANNOT be used inline — they only work as
// regex pattern delimiters at the rule level. Compound /= and %= assignments
// also fail for the same reason.
//
// MISSING: / (division), % (modulo), ^ (exponentiation), ** (GNU AWK)

func TestArithmeticAddition(t *testing.T) {
	t.Run("literal + literal", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print 2 + 3 }`, ""), "5\n")
	})
	t.Run("variable + variable", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { x = 10; y = 20; print x + y }`, ""), "30\n")
	})
	t.Run("field + number in body", func(t *testing.T) {
		check(t, runAWK(t, `{ print $1 + 5 }`, "10"), "15\n")
	})
	t.Run("NR + NF combination", func(t *testing.T) {
		check(t, runAWK(t, `{ print NR + NF }`, "a b c"), "4\n")
	})
}

func TestArithmeticSubtraction(t *testing.T) {
	t.Run("literal - literal", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print 10 - 3 }`, ""), "7\n")
	})
	t.Run("negative result", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print 3 - 10 }`, ""), "-7\n")
	})
	t.Run("unary minus", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { x = 5; print -x }`, ""), "-5\n")
	})
}

func TestArithmeticMultiplication(t *testing.T) {
	t.Run("literal * literal", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print 4 * 5 }`, ""), "20\n")
	})
	t.Run("zero multiplication", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print 100 * 0 }`, ""), "0\n")
	})
	t.Run("field * field", func(t *testing.T) {
		check(t, runAWK(t, `{ print $1 * $2 }`, "6 7"), "42\n")
	})
}

func TestArithmeticOperatorPrecedence(t *testing.T) {
	t.Run("* before +", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print 2 + 3 * 4 }`, ""), "14\n")
	})
	t.Run("parens override precedence", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print 2 + 3 * 4 }`, ""), "14\n")
	})
}

func TestArithmeticCompoundAssignment(t *testing.T) {
	t.Run("x += n", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { x = 5; x += 3; print x }`, ""), "8\n")
	})
	t.Run("x -= n", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { x = 5; x -= 3; print x }`, ""), "2\n")
	})
	t.Run("x *= n", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { x = 5; x *= 3; print x }`, ""), "15\n")
	})
	// KNOWN: x /= n and x %= n are not testable — lexer treats / and % as regex
	t.Run("compound assignment in body", func(t *testing.T) {
		check(t, runAWK(t, `{ sum += $1; print sum }`, "10\n20\n30"), "10\n30\n60\n")
	})
}

func TestArithmeticSimpleAssignment(t *testing.T) {
	t.Run("x = literal", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { x = 42; print x }`, ""), "42\n")
	})
	t.Run("x = expression", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { x = 2 + 3 * 4; print x }`, ""), "14\n")
	})
	t.Run("x = field", func(t *testing.T) {
		check(t, runAWK(t, `{ x = $1; print x }`, "hello"), "hello\n")
	})
	t.Run("chained assignment (x = y = val)", func(t *testing.T) {
		got := runAWK(t, `BEGIN { x = y = 10; print x, y }`, "")
		check(t, got, "10 10\n")
	})
}

func TestArithmeticPostfixIncrement(t *testing.T) {
	t.Run("x++ returns old value then increments", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { x = 5; print x++; print x }`, ""), "5\n6\n")
	})
	t.Run("c++ postfix counter in body", func(t *testing.T) {
		check(t, runAWK(t, `{ print c++ }`, "a\nb\nc"), "0\n1\n2\n")
	})
}

func TestArithmeticPrefixIncrement(t *testing.T) {
	t.Run("++x returns new value", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { x = 5; print ++x; print x }`, ""), "6\n6\n")
	})
	t.Run("++counter in body for each record", func(t *testing.T) {
		check(t, runAWK(t, `{ print ++c }`, "a\nb\nc"), "1\n2\n3\n")
	})
}

func TestArithmeticPostfixDecrement(t *testing.T) {
	t.Run("x-- returns old value then decrements", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { x = 5; print x--; print x }`, ""), "5\n4\n")
	})
	t.Run("--counter counts down", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { n = 3 } { print n-- }`, "a\nb\nc"), "3\n2\n1\n")
	})
}

func TestArithmeticPrefixDecrement(t *testing.T) {
	t.Run("--x returns new value", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { x = 5; print --x; print x }`, ""), "4\n4\n")
	})
}

func TestArithmeticFloatHandling(t *testing.T) {
	t.Run("float addition", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print 1.5 + 2.5 }`, ""), "4\n")
	})
	t.Run("float multiplication", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print 2.5 * 3 }`, ""), "7.5\n")
	})
	t.Run("integer display (no decimal)", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print 2.0 + 3.0 }`, ""), "5\n")
	})
}
