package main

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
const testInputPath = "../../tests/test_input.txt"

func runAWK(t *testing.T, program string, input string) string {
	t.Helper()
	tokens := lexer.New(program).Tokenize()
	nodes, err := parser.New(tokens).Parse()
	if err != nil {
		t.Fatalf("parse error: %v", err)
	}
	return interpreter.New().Run(nodes, input)
}

func loadTestInput(t *testing.T) string {
	t.Helper()
	data, err := os.ReadFile(testInputPath)
	if err != nil {
		t.Fatalf("failed to read test input: %v", err)
	}
	return string(data)
}

func TestPrintEveryLine(t *testing.T) {
	result := runAWK(t, "{ print }", "hello\nworld")
	if result != "hello\nworld\n" {
		t.Fatalf("got %q, want %q", result, "hello\nworld\n")
	}
}

func TestPrintFirstField(t *testing.T) {
	result := runAWK(t, "{ print $1 }", "a b c\nd e f")
	if result != "a\nd\n" {
		t.Fatalf("got %q, want %q", result, "a\nd\n")
	}
}

func TestPrintSecondField(t *testing.T) {
	result := runAWK(t, "{ print $2 }", "a b c\nd e f")
	if result != "b\ne\n" {
		t.Fatalf("got %q, want %q", result, "b\ne\n")
	}
}

func TestNRAndFirstField(t *testing.T) {
	result := runAWK(t, "{ print NR, $1 }", "alpha\nbeta\ngamma")
	if result != "1 alpha\n2 beta\n3 gamma\n" {
		t.Fatalf("got %q, want %q", result, "1 alpha\n2 beta\n3 gamma\n")
	}
}

func TestNRAndFirstFieldFromFile(t *testing.T) {
	input := loadTestInput(t)
	firstLine := strings.SplitN(input, "\n", 2)[0]
	fields := strings.Fields(firstLine)
	eid := fields[0]
	expected := "1 " + eid + "\n"

	result := runAWK(t, "{ print NR, $1 }", strings.SplitN(input, "\n", 2)[0])
	if result != expected {
		t.Fatalf("got %q, want %q", result, expected)
	}
}

func TestNRAllRecords(t *testing.T) {
	input := loadTestInput(t)
	result := runAWK(t, "{ print NR }", input)

	lines := strings.Split(strings.TrimRight(result, "\n"), "\n")
	if len(lines) != 100 {
		t.Fatalf("expected 100 lines, got %d", len(lines))
	}
	if lines[0] != "1" {
		t.Fatalf("first line: got %q, want %q", lines[0], "1")
	}
	if lines[99] != "100" {
		t.Fatalf("last line: got %q, want %q", lines[99], "100")
	}
}

func TestFieldOne(t *testing.T) {
	input := loadTestInput(t)
	result := runAWK(t, "{ print $1 }", input)

	lines := strings.Split(strings.TrimRight(result, "\n"), "\n")
	if len(lines) != 100 {
		t.Fatalf("expected 100 lines, got %d", len(lines))
	}
	if lines[0] != "E001" {
		t.Fatalf("first line: got %q, want %q", lines[0], "E001")
	}
	if lines[99] != "E100" {
		t.Fatalf("last line: got %q, want %q", lines[99], "E100")
	}
}

func TestFieldLast(t *testing.T) {
	input := loadTestInput(t)
	result := runAWK(t, "{ print $NF }", input)

	for _, line := range strings.Split(strings.TrimRight(result, "\n"), "\n") {
		years, err := strconv.Atoi(line)
		if err != nil {
			t.Fatalf("expected a number, got %q", line)
		}
		if years < 1 || years > 30 {
			t.Fatalf("years of experience %d out of range", years)
		}
	}
}

func TestFieldCombination(t *testing.T) {
	input := loadTestInput(t)
	firstLine := strings.SplitN(input, "\n", 2)[0]
	fields := strings.Fields(firstLine)
	expected := fields[0] + " " + fields[1] + " " + fields[2] + "\n"

	result := runAWK(t, "{ print $1, $2, $3 }", firstLine)
	if result != expected {
		t.Fatalf("got %q, want %q", result, expected)
	}
}

func TestDollarZero(t *testing.T) {
	input := loadTestInput(t)
	result := runAWK(t, "{ print $0 }", input)
	if result != input {
		t.Fatalf("output does not match input")
	}
}
