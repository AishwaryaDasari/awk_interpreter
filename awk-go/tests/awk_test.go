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

// testInputPath is relative to awk-go/tests/ when 'go test ./tests/' is run.
const testInputPath = "../testdata/test_input.txt"

// runAWK is a helper that lexes, parses, and interprets a program against input.
func runAWK(t *testing.T, program string, input string) string {
	t.Helper()
	tokens := lexer.New(program).Tokenize()
	nodes, err := parser.New(tokens).Parse()
	if err != nil {
		t.Fatalf("parse error: %v", err)
	}
	return interpreter.New().Run(nodes, input)
}

// loadTestInput reads the shared 100-line employee test file.
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
	result := runAWK(t, "{ print }", "hello\nworld")
	want := "hello\nworld\n"
	if result != want {
		t.Fatalf("got %q, want %q", result, want)
	}
}

func TestPrintString(t *testing.T) {
	result := runAWK(t, `{ print "hi" }`, "line1\nline2")
	want := "hi\nhi\n"
	if result != want {
		t.Fatalf("got %q, want %q", result, want)
	}
}

// ── Feature 2: Fields ($0, $n, $NF, NF, NR) ──────────────────────────────────

func TestDollarZero(t *testing.T) {
	input := "Name Marks\nSam 67\nRam 78\nCam 89"
	result := runAWK(t, "{ print $0 }", input)
	// $0 should reproduce each line exactly
	want := input + "\n"
	if result != want {
		t.Fatalf("got %q, want %q", result, want)
	}
}

func TestPrintFirstField(t *testing.T) {
	result := runAWK(t, "{ print $1 }", "a b c\nd e f")
	want := "a\nd\n"
	if result != want {
		t.Fatalf("got %q, want %q", result, want)
	}
}

func TestPrintSecondField(t *testing.T) {
	result := runAWK(t, "{ print $2 }", "a b c\nd e f")
	want := "b\ne\n"
	if result != want {
		t.Fatalf("got %q, want %q", result, want)
	}
}

func TestPrintMultipleFields(t *testing.T) {
	result := runAWK(t, "{ print $1, $2 }", "Sam 67\nRam 78\nCam 89")
	want := "Sam 67\nRam 78\nCam 89\n"
	if result != want {
		t.Fatalf("got %q, want %q", result, want)
	}
}

func TestNF(t *testing.T) {
	// Each line has 2 fields — NF should be 2
	result := runAWK(t, "{ print NF }", "Sam 67\nRam 78")
	want := "2\n2\n"
	if result != want {
		t.Fatalf("got %q, want %q", result, want)
	}
}

func TestNR(t *testing.T) {
	result := runAWK(t, "{ print NR }", "a\nb\nc")
	want := "1\n2\n3\n"
	if result != want {
		t.Fatalf("got %q, want %q", result, want)
	}
}

func TestNRAndFirstField(t *testing.T) {
	result := runAWK(t, "{ print NR, $1 }", "alpha\nbeta\ngamma")
	want := "1 alpha\n2 beta\n3 gamma\n"
	if result != want {
		t.Fatalf("got %q, want %q", result, want)
	}
}

func TestDollarNF(t *testing.T) {
	// $NF is the last field
	result := runAWK(t, "{ print $NF }", "a b c\n1 2 3 4")
	want := "c\n4\n"
	if result != want {
		t.Fatalf("got %q, want %q", result, want)
	}
}

func TestOutOfRangeField(t *testing.T) {
	// $5 on a 2-field line should return empty string
	result := runAWK(t, "{ print $5 }", "a b")
	want := "\n"
	if result != want {
		t.Fatalf("got %q, want %q", result, want)
	}
}

// ── File-based tests (require testdata/test_input.txt) ────────────────────────

func TestNRAllRecords(t *testing.T) {
	input := loadTestInput(t)
	result := runAWK(t, "{ print NR }", input)

	lines := strings.Split(strings.TrimRight(result, "\n"), "\n")
	if len(lines) != 100 {
		t.Fatalf("expected 100 lines, got %d", len(lines))
	}
	if lines[0] != "1" {
		t.Fatalf("first NR: got %q, want %q", lines[0], "1")
	}
	if lines[99] != "100" {
		t.Fatalf("last NR: got %q, want %q", lines[99], "100")
	}
}

func TestFieldOneFromFile(t *testing.T) {
	input := loadTestInput(t)
	result := runAWK(t, "{ print $1 }", input)

	lines := strings.Split(strings.TrimRight(result, "\n"), "\n")
	if len(lines) != 100 {
		t.Fatalf("expected 100 lines, got %d", len(lines))
	}
	if lines[0] != "E001" {
		t.Fatalf("first $1: got %q, want %q", lines[0], "E001")
	}
	if lines[99] != "E100" {
		t.Fatalf("last $1: got %q, want %q", lines[99], "E100")
	}
}

func TestFieldLastFromFile(t *testing.T) {
	// Last field is years of experience (1–30)
	input := loadTestInput(t)
	result := runAWK(t, "{ print $NF }", input)

	for _, line := range strings.Split(strings.TrimRight(result, "\n"), "\n") {
		years, err := strconv.Atoi(line)
		if err != nil {
			t.Fatalf("expected a number, got %q", line)
		}
		if years < 1 || years > 30 {
			t.Fatalf("years of experience %d out of expected range 1–30", years)
		}
	}
}

func TestFieldCombinationFromFile(t *testing.T) {
	input := loadTestInput(t)
	firstLine := strings.SplitN(input, "\n", 2)[0]
	fields := strings.Fields(firstLine)
	want := fields[0] + " " + fields[1] + " " + fields[2] + "\n"

	result := runAWK(t, "{ print $1, $2, $3 }", firstLine)
	if result != want {
		t.Fatalf("got %q, want %q", result, want)
	}
}

func TestNRAndFirstFieldFromFile(t *testing.T) {
	input := loadTestInput(t)
	firstLine := strings.SplitN(input, "\n", 2)[0]
	fields := strings.Fields(firstLine)
	want := "1 " + fields[0] + "\n"

	result := runAWK(t, "{ print NR, $1 }", firstLine)
	if result != want {
		t.Fatalf("got %q, want %q", result, want)
	}
}

func TestDollarZeroFromFile(t *testing.T) {
	input := loadTestInput(t)
	result := runAWK(t, "{ print $0 }", input)
	if result != input {
		t.Fatalf("$0 output does not match input exactly")
	}
}
