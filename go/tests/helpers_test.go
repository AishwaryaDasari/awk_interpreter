package tests

import (
	"os"
	"strings"
	"testing"

	"awk-go/interpreter"
	"awk-go/lexer"
	"awk-go/parser"
)

func runAWK(t *testing.T, program, input string) string {
	t.Helper()
	t.Logf("run: %s", program)
	tokens := lexer.New(program).Tokenize()
	prog, err := parser.New(tokens).Parse()
	if err != nil {
		t.Fatalf("parse error: %v", err)
	}
	return interpreter.New().Run(prog, input)
}

func check(t *testing.T, got, want string) {
	t.Helper()
	t.Logf("output: %s", got)
	if got != want {
		t.Fatalf("FAIL\n  got:  %q\n  want: %q", got, want)
	}
	t.Logf("PASS")
}

func loadTestInput(t *testing.T) string {
	t.Helper()
	data, err := os.ReadFile("../testdata/test_input.txt")
	if err != nil {
		t.Fatalf("failed to read test input: %v", err)
	}
	return strings.ReplaceAll(string(data), "\r\n", "\n")
}
