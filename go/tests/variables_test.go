package tests

import (
	"testing"
)

// ── User-defined variables ───────────────────────────────────────────────────
// IMPLEMENTED: user-defined variables via =, +=, -=, *=, ++, --
// Variables persist across BEGIN → per-line → END blocks.
// Undefined variables default to empty string / zero.
//
// KNOWN LIMITATIONS:
// - NR/NF built-in vars shadow user assignments: getVar("NR") always reads
//   i.nr directly, not the variables map. Assigning to NR has no visible effect.
// - /= and %= are not testable (lexer treats / and % as regex).
// - String concatenation is broken in the parser (see comparison_logic_test.go).
//   Use comma-separated print args instead.
//
// MISSING: arrays (associative), FS/OFS/ORS/FNR/FILENAME built-in vars,
// variable assignment from CLI with -v flag, field separator (FS)

func TestVariableAssignmentAndPrint(t *testing.T) {
	t.Run("assign number in BEGIN", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { x = 42; print x }`, ""), "42\n")
	})
	t.Run("assign string in BEGIN", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { name = "Alice"; print name }`, ""), "Alice\n")
	})
	t.Run("assign from expression", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { x = 2 + 3 * 4; print x }`, ""), "14\n")
	})
	t.Run("assign from field", func(t *testing.T) {
		check(t, runAWK(t, `{ x = $1; print x }`, "hello"), "hello\n")
	})
}

func TestVariablePersistenceAcrossBlocks(t *testing.T) {
	t.Run("variable set in BEGIN accessible in body", func(t *testing.T) {
		got := runAWK(t, `BEGIN { msg = "hello" } { print msg }`, "a\nb")
		check(t, got, "hello\nhello\n")
	})
	t.Run("variable set in body accessible in END", func(t *testing.T) {
		got := runAWK(t, `{ count = NR } END { print count }`, "a\nb\nc")
		check(t, got, "3\n")
	})
	t.Run("variable accumulates across body", func(t *testing.T) {
		got := runAWK(t, `{ sum += $1 } END { print sum }`, "10\n20\n30")
		check(t, got, "60\n")
	})
	t.Run("BEGIN sets initial, body modifies, END reads", func(t *testing.T) {
		got := runAWK(t, `BEGIN { total = 0 } { total += $1 } END { print total }`, "5\n10\n15")
		check(t, got, "30\n")
	})
}

func TestVariableDefaultValue(t *testing.T) {
	t.Run("uninitialized variable is empty string", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print x }`, ""), "\n")
	})
	t.Run("uninitialized variable treated as 0 in arithmetic", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { print x + 5 }`, ""), "5\n")
	})
}

func TestVariableReassignment(t *testing.T) {
	t.Run("reassign variable multiple times", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { x = 1; x = 2; x = 3; print x }`, ""), "3\n")
	})
	t.Run("self-assignment", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { x = 5; x = x; print x }`, ""), "5\n")
	})
	t.Run("swap via temp", func(t *testing.T) {
		got := runAWK(t, `BEGIN { a = 1; b = 2; t = a; a = b; b = t; print a, b }`, "")
		check(t, got, "2 1\n")
	})
}

func TestVariableNaming(t *testing.T) {
	t.Run("simple name", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { x = 1; print x }`, ""), "1\n")
	})
	t.Run("underscore in name", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { my_var = 10; print my_var }`, ""), "10\n")
	})
	t.Run("mixed case", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { MyVar = 42; print MyVar }`, ""), "42\n")
	})
	t.Run("name with digits", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { var2 = 99; print var2 }`, ""), "99\n")
	})
}

func TestVariableBuiltins(t *testing.T) {
	t.Run("NR is record number", func(t *testing.T) {
		check(t, runAWK(t, `{ print NR }`, "a\nb\nc"), "1\n2\n3\n")
	})
	t.Run("NF is field count", func(t *testing.T) {
		check(t, runAWK(t, `{ print NF }`, "a b c"), "3\n")
	})
	// In this implementation NR/NF cannot be user-assigned because getVar's
	// switch case always takes priority over the variables map.
}

func TestVariableScope(t *testing.T) {
	t.Run("multiple rules share global scope", func(t *testing.T) {
		got := runAWK(t, `BEGIN { x = 10 } { x = x + 1 } END { print x }`, "a\nb\nc")
		check(t, got, "13\n")
	})
	t.Run("counter via variable in body", func(t *testing.T) {
		check(t, runAWK(t, `{ c++; print c }`, "a\nb\nc"), "1\n2\n3\n")
	})
}

func TestVariableComplexExpressions(t *testing.T) {
	t.Run("variable in arithmetic", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { x = 5; y = 10; print x * y + 2 }`, ""), "52\n")
	})
	t.Run("variable in comparison", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { x = 5; print x > 3 }`, ""), "1\n")
	})
	t.Run("multiple variables in expression", func(t *testing.T) {
		check(t, runAWK(t, `BEGIN { a = 2; b = 3; c = 4; print a + b * c }`, ""), "14\n")
	})
}

func TestVariableAggregationFile(t *testing.T) {
	t.Run("sum of salary field ($4) across all records", func(t *testing.T) {
		input := loadTestInput(t)
		got := runAWK(t, `{ sum += $4 } END { print sum }`, input)
		check(t, got, "8632000\n")
	})
	t.Run("count records matching Engineering regex", func(t *testing.T) {
		input := loadTestInput(t)
		got := runAWK(t, `/Engineering/ { count++ } END { print count }`, input)
		check(t, got, "27\n")
	})
}
