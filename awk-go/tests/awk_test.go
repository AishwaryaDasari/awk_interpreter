package tests

import (
	"os"
	"strconv"
	"strings"
	"testing"

	"awk-go/interpreter"
	"awk-go/lexer"
	"awk-go/parser"
)

const testInputPath = "../testdata/test_input.txt"

// runAWK lexes, parses, and runs a program against the given input string.
func runAWK(t *testing.T, program string, input string) string {
	t.Helper()
	tokens := lexer.New(program).Tokenize()
	prog, err := parser.New(tokens).Parse()
	if err != nil {
		t.Fatalf("parse error: %v", err)
	}
	return interpreter.New().Run(prog, input)
}

func loadTestInput(t *testing.T) string {
	t.Helper()
	data, err := os.ReadFile(testInputPath)
	if err != nil {
		t.Fatalf("failed to read test input %q: %v", testInputPath, err)
	}
	return string(data)
}

// ── Feature 1: print ─────────────────────────────────────────────────────────

func TestPrintEveryLine(t *testing.T) {
	got := runAWK(t, "{ print }", "hello\nworld")
	want := "hello\nworld\n"
	if got != want {
		t.Fatalf("got %q, want %q", got, want)
	}
}

func TestPrintLiteralString(t *testing.T) {
	got := runAWK(t, `{ print "hi" }`, "a\nb")
	want := "hi\nhi\n"
	if got != want {
		t.Fatalf("got %q, want %q", got, want)
	}
}

// ── Feature 2: Fields ─────────────────────────────────────────────────────────

func TestDollarZero(t *testing.T) {
	input := "Sam 67\nRam 78\nCam 89"
	got := runAWK(t, "{ print $0 }", input)
	want := input + "\n"
	if got != want {
		t.Fatalf("got %q, want %q", got, want)
	}
}

func TestPrintFirstField(t *testing.T) {
	got := runAWK(t, "{ print $1 }", "a b c\nd e f")
	want := "a\nd\n"
	if got != want {
		t.Fatalf("got %q, want %q", got, want)
	}
}

func TestPrintSecondField(t *testing.T) {
	got := runAWK(t, "{ print $2 }", "a b c\nd e f")
	want := "b\ne\n"
	if got != want {
		t.Fatalf("got %q, want %q", got, want)
	}
}

func TestPrintMultipleFields(t *testing.T) {
	got := runAWK(t, "{ print $1, $2 }", "Sam 67\nRam 78\nCam 89")
	want := "Sam 67\nRam 78\nCam 89\n"
	if got != want {
		t.Fatalf("got %q, want %q", got, want)
	}
}

func TestNF(t *testing.T) {
	got := runAWK(t, "{ print NF }", "Sam 67\nRam 78")
	want := "2\n2\n"
	if got != want {
		t.Fatalf("got %q, want %q", got, want)
	}
}

func TestNR(t *testing.T) {
	got := runAWK(t, "{ print NR }", "a\nb\nc")
	want := "1\n2\n3\n"
	if got != want {
		t.Fatalf("got %q, want %q", got, want)
	}
}

func TestNRAndFirstField(t *testing.T) {
	got := runAWK(t, "{ print NR, $1 }", "alpha\nbeta\ngamma")
	want := "1 alpha\n2 beta\n3 gamma\n"
	if got != want {
		t.Fatalf("got %q, want %q", got, want)
	}
}

func TestDollarNF(t *testing.T) {
	got := runAWK(t, "{ print $NF }", "a b c\n1 2 3 4")
	want := "c\n4\n"
	if got != want {
		t.Fatalf("got %q, want %q", got, want)
	}
}

func TestOutOfRangeField(t *testing.T) {
	got := runAWK(t, "{ print $5 }", "a b")
	want := "\n"
	if got != want {
		t.Fatalf("got %q, want %q", got, want)
	}
}

// ── Feature 3: BEGIN / END ────────────────────────────────────────────────────

// BEGIN runs once before any input is read.
func TestBeginOnly(t *testing.T) {
	got := runAWK(t, `BEGIN { print "start" }`, "ignored\nlines")
	want := "start\n"
	if got != want {
		t.Fatalf("got %q, want %q", got, want)
	}
}

// END runs once after all input is consumed.
func TestEndOnly(t *testing.T) {
	got := runAWK(t, `END { print "done" }`, "a\nb\nc")
	want := "done\n"
	if got != want {
		t.Fatalf("got %q, want %q", got, want)
	}
}

// BEGIN runs before lines, END runs after.
func TestBeginAndEnd(t *testing.T) {
	got := runAWK(t, `BEGIN { print "start" } END { print "end" }`, "a\nb")
	want := "start\nend\n"
	if got != want {
		t.Fatalf("got %q, want %q", got, want)
	}
}

// BEGIN + body + END all together.
func TestBeginBodyEnd(t *testing.T) {
	got := runAWK(t, `BEGIN { print "---" } { print $1 } END { print "---" }`, "Sam 67\nRam 78")
	want := "---\nSam\nRam\n---\n"
	if got != want {
		t.Fatalf("got %q, want %q", got, want)
	}
}

// BEGIN with no input — should still run.
func TestBeginWithNoInput(t *testing.T) {
	got := runAWK(t, `BEGIN { print "hello" }`, "")
	want := "hello\n"
	if got != want {
		t.Fatalf("got %q, want %q", got, want)
	}
}

// END sees the correct final NR value.
func TestEndSeesNR(t *testing.T) {
	got := runAWK(t, `END { print NR }`, "a\nb\nc")
	want := "3\n"
	if got != want {
		t.Fatalf("got %q, want %q", got, want)
	}
}

// BEGIN prints a header, body prints data, END prints total line count.
func TestBeginHeaderEndCount(t *testing.T) {
	input := "Sam 67\nRam 78\nCam 89"
	got := runAWK(t, `BEGIN { print "Name Marks" } { print $1, $2 } END { print "Total:", NR }`, input)
	want := "Name Marks\nSam 67\nRam 78\nCam 89\nTotal: 3\n"
	if got != want {
		t.Fatalf("got %q, want %q", got, want)
	}
}

// Multiple BEGIN blocks — both should run in order.
func TestMultipleBeginBlocks(t *testing.T) {
	got := runAWK(t, `BEGIN { print "first" } BEGIN { print "second" }`, "")
	want := "first\nsecond\n"
	if got != want {
		t.Fatalf("got %q, want %q", got, want)
	}
}

// Multiple END blocks — both should run in order.
func TestMultipleEndBlocks(t *testing.T) {
	got := runAWK(t, `END { print "first" } END { print "second" }`, "x")
	want := "first\nsecond\n"
	if got != want {
		t.Fatalf("got %q, want %q", got, want)
	}
}

// ── File-based tests ──────────────────────────────────────────────────────────

func TestNRAllRecords(t *testing.T) {
	input := loadTestInput(t)
	got := runAWK(t, "{ print NR }", input)
	lines := strings.Split(strings.TrimRight(got, "\n"), "\n")
	if len(lines) != 100 {
		t.Fatalf("expected 100 lines, got %d", len(lines))
	}
	if lines[0] != "1" {
		t.Fatalf("first NR: got %q, want 1", lines[0])
	}
	if lines[99] != "100" {
		t.Fatalf("last NR: got %q, want 100", lines[99])
	}
}

func TestFieldOneFromFile(t *testing.T) {
	input := loadTestInput(t)
	got := runAWK(t, "{ print $1 }", input)
	lines := strings.Split(strings.TrimRight(got, "\n"), "\n")
	if len(lines) != 100 {
		t.Fatalf("expected 100 lines, got %d", len(lines))
	}
	if lines[0] != "E001" {
		t.Fatalf("first $1: got %q, want E001", lines[0])
	}
	if lines[99] != "E100" {
		t.Fatalf("last $1: got %q, want E100", lines[99])
	}
}

func TestFieldLastFromFile(t *testing.T) {
	input := loadTestInput(t)
	got := runAWK(t, "{ print $NF }", input)
	for _, line := range strings.Split(strings.TrimRight(got, "\n"), "\n") {
		years, err := strconv.Atoi(line)
		if err != nil {
			t.Fatalf("expected a number, got %q", line)
		}
		if years < 1 || years > 30 {
			t.Fatalf("years %d out of range 1–30", years)
		}
	}
}

func TestDollarZeroFromFile(t *testing.T) {
	input := loadTestInput(t)
	// Normalize CRLF → LF so the comparison works on both Windows and Unix.
	input = strings.ReplaceAll(input, "\r\n", "\n")
	got := runAWK(t, "{ print $0 }", input)
	if strings.TrimRight(got, "\n") != strings.TrimRight(input, "\n") {
		t.Fatalf("$0 output does not match input.\ngot:  %q\nwant: %q",
			got[:min(len(got), 100)], input[:min(len(input), 100)])
	}
}

func TestBeginEndWithFile(t *testing.T) {
	input := loadTestInput(t)
	got := runAWK(t, `BEGIN { print "START" } END { print "END", NR }`, input)
	want := "START\nEND 100\n"
	if got != want {
		t.Fatalf("got %q, want %q", got, want)
	}
}
