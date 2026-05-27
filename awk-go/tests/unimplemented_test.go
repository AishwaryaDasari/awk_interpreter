package tests

import (
	"testing"
)

// ── Unimplemented AWK features ───────────────────────────────────────────────
//
// This file documents AWK features that this interpreter does NOT yet support.
// Tests use t.Skip() to show what *would* be tested if the feature were
// implemented. This serves as a living checklist of missing functionality.

// # Feature status overview
//
//  ✅ = DONE      🔲 = PLANNED (from docs)   ❌ = MISSING (not in code)
//
//  1.  print                     ✅ DONE
//  2.  Fields ($0, $1, $NF)      ✅ DONE
//  3.  BEGIN/END + regex rules   ✅ DONE
//  4.  Variables + arithmetic    ✅ DONE  (except /% in expressions due to lexer)
//  5.  Field separator (FS/-F)   ❌ MISSING — always splits on whitespace
//  6.  Expression patterns       ❌ MISSING — e.g.  $2 > 10 { ... }
//  7.  Range patterns            ❌ MISSING — e.g.  /start/,/end/ { ... }
//  8.  if/else statements        ❌ MISSING
//  9.  while/for loops           ❌ MISSING (+ break/continue)
// 10.  next / exit               ❌ MISSING
// 11.  Built-in string funcs     ❌ MISSING — length, substr, gsub, split, etc.
// 12.  Built-in math funcs       ❌ MISSING — int, sqrt, sin, cos, rand, etc.
// 13.  printf / sprintf          ❌ MISSING
// 14.  Associative arrays        ❌ MISSING
// 15.  ~ and !~ operators        ❌ MISSING — regex match / no-match
// 16.  ^ (exponentiation)        ❌ MISSING
// 17.  ?: (ternary conditional)  ❌ MISSING
// 18.  OFS / ORS / RS / FNR     ❌ MISSING — other built-in variables
// 19.  FILENAME                  ❌ MISSING
// 20.  Comments (#)              ❌ MISSING — lexer treats # as ILLEGAL
// 21.  -F / -f / -v CLI flags   ❌ MISSING
// 22.  Multi-file / stdin        ❌ MISSING
// 23.  / and % in expressions    ❌ MISSING — lexer always reads / as regex
// 24.  String concatenation      ❌ BUGGY  — parser consumes next token
// 25.  Expression patterns as
//      rule selectors            ❌ MISSING — only BEGIN, END, /regex/, always

func TestUnimplementedExpressionPattern(t *testing.T) {
	t.Skip("Expression patterns not implemented — parser accepts only BEGIN, END, /regex/, or implicit-always as rule patterns")
	_ = runAWK(t, `$2 > 50 { print $1 }`, "Sam 30\nRam 70")
}

func TestUnimplementedRangePattern(t *testing.T) {
	t.Skip("Range patterns not implemented — e.g. /start/,/end/")
	_ = runAWK(t, `/start/,/end/ { print }`, "a\nstart\nb\nend\nc")
}

func TestUnimplementedIfElse(t *testing.T) {
	t.Skip("if/else statements not implemented — no AST node or parser support")
	_ = runAWK(t, `BEGIN { x = 10; if (x > 5) print "big"; else print "small" }`, "")
}

func TestUnimplementedWhileLoop(t *testing.T) {
	t.Skip("while loops not implemented")
	_ = runAWK(t, `BEGIN { x = 3; while (x > 0) { print x; x-- } }`, "")
}

func TestUnimplementedForLoop(t *testing.T) {
	t.Skip("for loops not implemented")
	_ = runAWK(t, `BEGIN { for (i = 1; i <= 3; i++) print i }`, "")
}

func TestUnimplementedForInArray(t *testing.T) {
	t.Skip("for-in loops and arrays not implemented")
	_ = runAWK(t, `BEGIN { a["x"] = 1; for (k in a) print k }`, "")
}

func TestUnimplementedNext(t *testing.T) {
	t.Skip("'next' statement not implemented")
	_ = runAWK(t, `/Engineering/ { print $1; next } { print $1, "other" }`, "E001 Engineering\nE002 Marketing")
}

func TestUnimplementedExit(t *testing.T) {
	t.Skip("'exit' statement not implemented")
	_ = runAWK(t, `BEGIN { exit; print "no" }`, "")
}

func TestUnimplementedLengthFunction(t *testing.T) {
	t.Skip("built-in function 'length' not implemented")
	_ = runAWK(t, `BEGIN { print length("hello") }`, "")
}

func TestUnimplementedSubstrFunction(t *testing.T) {
	t.Skip("built-in function 'substr' not implemented")
	_ = runAWK(t, `BEGIN { print substr("hello", 2, 3) }`, "")
}

func TestUnimplementedGsubFunction(t *testing.T) {
	t.Skip("built-in function 'gsub' not implemented")
	_ = runAWK(t, `BEGIN { s = "aXbXc"; gsub(/X/, "Y", s); print s }`, "")
}

func TestUnimplementedSplitFunction(t *testing.T) {
	t.Skip("built-in function 'split' not implemented")
	_ = runAWK(t, `BEGIN { n = split("a,b,c", arr, ","); print n }`, "")
}

func TestUnimplementedPrintf(t *testing.T) {
	t.Skip("printf not implemented")
	_ = runAWK(t, `BEGIN { printf("%-10s %d\n", "hello", 42) }`, "")
}

func TestUnimplementedMatchOperator(t *testing.T) {
	t.Skip("~ and !~ match operators not implemented — not in lexer")
	_ = runAWK(t, `BEGIN { s = "hello"; print s ~ /ell/ }`, "")
}

func TestUnimplementedExponentiation(t *testing.T) {
	t.Skip("^ exponentiation operator not implemented — not in lexer")
	_ = runAWK(t, `BEGIN { print 2 ^ 10 }`, "")
}

func TestUnimplementedTernary(t *testing.T) {
	t.Skip("?: ternary conditional not implemented")
	_ = runAWK(t, `BEGIN { x = 10; print (x > 5) ? "big" : "small" }`, "")
}

func TestUnimplementedFieldSeparator(t *testing.T) {
	t.Skip("FS variable / -F flag not implemented — fields always split on whitespace")
	_ = runAWK(t, `BEGIN { FS = "|" } { print $2 }`, "a|b|c")
}

func TestUnimplementedOFS(t *testing.T) {
	t.Skip("OFS variable not implemented — print separator always space")
	_ = runAWK(t, `BEGIN { OFS = "," } { print $1, $2 }`, "a b")
}

func TestUnimplementedBuiltinVariables(t *testing.T) {
	t.Skip("FNR, RS, ORS, FILENAME not implemented")
	_ = runAWK(t, `BEGIN { print FILENAME }`, "")
}

func TestUnimplementedComments(t *testing.T) {
	t.Skip("# comments not implemented — lexer treats # as ILLEGAL")
	_ = runAWK(t, `BEGIN { print "hi" } # this is a comment`, "")
}

func TestUnimplementedCLIFlags(t *testing.T) {
	t.Skip("-F, -f, -v CLI flags not implemented — only '<program> <file>' args")
}

func TestUnimplementedMultiFileStdin(t *testing.T) {
	t.Skip("Multi-file input and stdin fallback not implemented")
}

func TestUnimplementedMathFunctions(t *testing.T) {
	t.Skip("Math built-in functions not implemented: int, sqrt, sin, cos, atan2, rand, srand, log, exp")
	_ = runAWK(t, `BEGIN { print sqrt(16); print int(3.7); print sin(0) }`, "")
}

func TestKnownParserBugConcat(t *testing.T) {
	t.Skip("KNOWN BUG: String concatenation via adjacency is broken — infixOp consumes the next token via p.advance()")
	// These would fail:
	//   runAWK(t, `BEGIN { print "hello" " " "world" }`, "")       → loses " "
	//   runAWK(t, `BEGIN { x = "a"; y = "b"; print x y }`, "")    → loses y
	//   runAWK(t, `{ print $1 $2 }`, "a b")                    → loses $2 token
}

func TestKnownLexerBugDivision(t *testing.T) {
	t.Skip("KNOWN BUG: Lexer always reads / as regex start — division / and modulo % unusable in expression bodies")
	// These would fail because / 3 } is parsed as regex / 3 }:
	//   runAWK(t, `BEGIN { print 10 / 3 }`, "")
	//   runAWK(t, `BEGIN { x = 10; x /= 2; print x }`, "")
}
