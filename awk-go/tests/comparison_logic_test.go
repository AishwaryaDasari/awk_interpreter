package tests

import (
	"testing"
)

// ── Comparison & logical operators ───────────────────────────────────────────
// IMPLEMENTED: ==, !=, <, <=, >, >=, &&, ||, !
// Comparisons return 1 (true) or 0 (false). Numeric vs string comparison is
// context-dependent: if both operands look numeric, compare as numbers;
// otherwise compare lexicographically as strings.
//
// KNOWN LIMITATIONS:
// - String concatenation (adjacent expressions) is broken in the parser:
//   infixOp's "concat" triggers p.advance(), consuming the next expression's
//   start token. Do NOT test `x y` or `"a" "b"` concatenation — it will fail.
// - The / regex ambiguity means you cannot use / inside expressions.
//
// MISSING: ~ (match), !~ (no match), ?: (ternary), ^ (exponentiation)

func TestComparisonEqual(t *testing.T) {
	t.Run("numeric == true", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print 5 == 5 }`, ""), "1\n")
	})
	t.Run("numeric == false", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print 5 == 3 }`, ""), "0\n")
	})
	t.Run("string == true", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print "abc" == "abc" }`, ""), "1\n")
	})
	t.Run("string == false", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print "abc" == "xyz" }`, ""), "0\n")
	})
	t.Run("variable == literal", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { x = 42; print x == 42 }`, ""), "1\n")
	})
	t.Run("field == string", func(t *testing.T) {
		check(t, runAWK(t, `{ print $1 == "Sam" }`, "Sam 67"), "1\n")
	})
}

func TestComparisonNotEqual(t *testing.T) {
	t.Run("numeric != true", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print 5 != 3 }`, ""), "1\n")
	})
	t.Run("numeric != false", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print 5 != 5 }`, ""), "0\n")
	})
	t.Run("string != true", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print "abc" != "xyz" }`, ""), "1\n")
	})
}

func TestComparisonLessThan(t *testing.T) {
	t.Run("numeric < true", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print 3 < 5 }`, ""), "1\n")
	})
	t.Run("numeric < false", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print 5 < 3 }`, ""), "0\n")
	})
	t.Run("equal < returns false", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print 5 < 5 }`, ""), "0\n")
	})
	t.Run("string < lexicographic", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print "apple" < "banana" }`, ""), "1\n")
	})
	t.Run("string < false", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print "banana" < "apple" }`, ""), "0\n")
	})
}

func TestComparisonLessEqual(t *testing.T) {
	t.Run("<= true (less)", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print 3 <= 5 }`, ""), "1\n")
	})
	t.Run("<= true (equal)", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print 5 <= 5 }`, ""), "1\n")
	})
	t.Run("<= false", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print 6 <= 5 }`, ""), "0\n")
	})
}

func TestComparisonGreaterThan(t *testing.T) {
	t.Run("> true", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print 5 > 3 }`, ""), "1\n")
	})
	t.Run("> false", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print 3 > 5 }`, ""), "0\n")
	})
}

func TestComparisonGreaterEqual(t *testing.T) {
	t.Run(">= true (greater)", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print 5 >= 3 }`, ""), "1\n")
	})
	t.Run(">= true (equal)", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print 5 >= 5 }`, ""), "1\n")
	})
}

func TestComparisonNumericVsString(t *testing.T) {
	t.Run("numeric string compared as numbers", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print "10" < "5" }`, ""), "0\n")
	})
	t.Run("mixed numeric and number-string", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { x = 42; print x == "42" }`, ""), "1\n")
	})
}

func TestLogicalAnd(t *testing.T) {
	t.Run("true && true = 1", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print 1 && 1 }`, ""), "1\n")
	})
	t.Run("true && false = 0", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print 1 && 0 }`, ""), "0\n")
	})
	t.Run("false && true = 0", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print 0 && 1 }`, ""), "0\n")
	})
	t.Run("false && false = 0", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print 0 && 0 }`, ""), "0\n")
	})
	t.Run("string truthy && 1", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print "hello" && 1 }`, ""), "1\n")
	})
	t.Run("empty string && 1 = 0", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print "" && 1 }`, ""), "0\n")
	})
}

func TestLogicalOr(t *testing.T) {
	t.Run("true || true = 1", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print 1 || 1 }`, ""), "1\n")
	})
	t.Run("true || false = 1", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print 1 || 0 }`, ""), "1\n")
	})
	t.Run("false || true = 1", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print 0 || 1 }`, ""), "1\n")
	})
	t.Run("false || false = 0", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print 0 || 0 }`, ""), "0\n")
	})
}

func TestLogicalNot(t *testing.T) {
	t.Run("!1 = 0", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print !1 }`, ""), "0\n")
	})
	t.Run("!0 = 1", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print !0 }`, ""), "1\n")
	})
	t.Run("!true = 0", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { x = 42; print !x }`, ""), "0\n")
	})
	t.Run("!false = 1", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { x = 0; print !x }`, ""), "1\n")
	})
	t.Run("!! = double negation", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print !!42 }`, ""), "1\n")
	})
	t.Run("! on empty string = 1", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print !"" }`, ""), "1\n")
	})
	t.Run("! on non-empty string = 0", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print !"hello" }`, ""), "0\n")
	})
}

func TestPrecedenceComparisonsAndLogic(t *testing.T) {
	t.Run("&& before ||", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print 1 || 0 && 0 }`, ""), "1\n")
	})
	t.Run("comparison before &&", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print 3 < 5 && 6 > 2 }`, ""), "1\n")
	})
	t.Run("arithmetic before comparison", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print 2 + 3 == 5 }`, ""), "1\n")
	})
}

func TestTruthiness(t *testing.T) {
	t.Run("0 is falsy", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print 0 || 0 }`, ""), "0\n")
	})
	t.Run("non-zero is truthy", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print 0.001 && 1 }`, ""), "1\n")
	})
	t.Run("empty string is falsy", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print "" || 1 }`, ""), "1\n")
	})
	t.Run("non-empty string is truthy", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print "false" && 1 }`, ""), "1\n")
	})
}
