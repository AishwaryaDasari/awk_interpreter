package tests

import (
	"testing"
)

// ── Expressions and edge cases ───────────────────────────────────────────────
// IMPLEMENTED: parenthesized expressions, field expressions ($()),
// multiple print args, semicolons, string escape sequences,
// negation of comparisons.
//
// KNOWN LIMITATIONS:
// - `print (expr) something` misparses: after print's closing paren, remaining
//   tokens become a separate expression statement (e.g. `* 4` is parsed as
//   a second statement). Avoid `print (expr) OP val`.
// - Field concatenation ($1 $2) is broken due to concat consuming the next $.
// - $ over a non-numeric field expression (e.g. $$1) produces unexpected output
//   because the inner $ is consumed, leaving the number. This is a concat issue.
// - The / regex ambiguity makes inline / unusable in expressions.
//
// MISSING: ternary (cond ? a : b), function calls, array indexing

func TestParenthesizedExpressions(t *testing.T) {
	t.Run("parenthesized variable assignment", func(t *testing.T) {
		got := runAWK(t, `BEGIN { x = (2 + 3) + 4; print x }`, "")
		check(t, got, "9\n")
	})
	t.Run("nested parentheses", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print ((2 + 3) * 4) }`, ""), "20\n")
	})
	t.Run("parentheses around comparison", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print (3 < 5) && (6 > 2) }`, ""), "1\n")
	})
	t.Run("parenthesized assignment", func(t *testing.T) {
		got := runAWK(t, `BEGIN { x = (1 + 2) * 3; print x }`, "")
		check(t, got, "9\n")
	})
}

func TestFieldExpressions(t *testing.T) {
	t.Run("$NF is last field", func(t *testing.T) {
		check(t, runAWK(t, `{ print $NF }`, "a b c d"), "d\n")
	})
	t.Run("$(NF - 1) is second-to-last field", func(t *testing.T) {
		check(t, runAWK(t, `{ print $(NF - 1) }`, "a b c d"), "c\n")
	})
	t.Run("$NR selects field by record number", func(t *testing.T) {
		got := runAWK(t, `{ print $NR }`, "x y z\np q\nm")
		// Record 1: NR=1, $1 = "x"
		// Record 2: NR=2, $2 = "q"
		// Record 3: NR=3, $3 = "" (out of range, zero value)
		check(t, got, "x\nq\n\n")
	})
	t.Run("$(1) parenthesized field index", func(t *testing.T) {
		check(t, runAWK(t, `{ print $(1) }`, "a b"), "a\n")
	})
	t.Run("field computed from variable", func(t *testing.T) {
		check(t, runAWK(t, `{ f = 2; print $f }`, "a b c"), "b\n")
	})
}

func TestMultiplePrintArgs(t *testing.T) {
	t.Run("print with comma separator", func(t *testing.T) {
		check(t, runAWK(t, `{ print $1, $2, $3 }`, "a b c"), "a b c\n")
	})
	t.Run("print mixed types", func(t *testing.T) {
		got := runAWK(t, `{ print "id:", $1, "score:", $2 }`, "Sam 95")
		check(t, got, "id: Sam score: 95\n")
	})
	t.Run("print empty arg list ignores field", func(t *testing.T) {
		check(t, runAWK(t, `{ print }`, "hello"), "hello\n")
	})
	t.Run("print with explicit parentheses", func(t *testing.T) {
		check(t, runAWK(t, `{ print($1, $2) }`, "a b"), "a b\n")
	})
}

func TestStringEscapes(t *testing.T) {
	t.Run("string with \\n (newline)", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print "line1\nline2" }`, ""), "line1\nline2\n")
	})
	t.Run("string with \\t (tab)", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print "a\tb" }`, ""), "a\tb\n")
	})
	t.Run("string with \\\" (escaped quote)", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print "she said \"hi\"" }`, ""), "she said \"hi\"\n")
	})
	t.Run("string with \\\\ (backslash)", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print "path\\to\\file" }`, ""), "path\\to\\file\n")
	})
}

func TestSemicolons(t *testing.T) {
	t.Run("multiple statements on one line with ;", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { x = 1; y = 2; print x, y }`, ""), "1 2\n")
	})
	t.Run("semicolon after print", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print "hello"; }`, ""), "hello\n")
	})
}

func TestNegationOfComparison(t *testing.T) {
	t.Run("negation of false comparison", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print !(5 < 3) }`, ""), "1\n")
	})
	t.Run("negation of true comparison", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print !(5 > 3) }`, ""), "0\n")
	})
}

func TestEdgeCaseExpressions(t *testing.T) {
	t.Run("empty input produces no output", func(t *testing.T) {
		check(t, runAWK(t, `{ print $1 }`, ""), "")
	})
	t.Run("single record with trailing newline", func(t *testing.T) {
		check(t, runAWK(t, `{ print $1 }`, "hello\n"), "hello\n")
	})
}
