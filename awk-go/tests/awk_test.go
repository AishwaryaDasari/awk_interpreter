package tests

// Each subtest name is the actual AWK command being tested.
// Running 'go test ./tests/ -v' shows:
//
//   === RUN   TestPrint/{ print }
//   === RUN   TestFields/$NF
//   === RUN   TestBegin/BEGIN { print "start" }
//   === RUN   TestRegex//Sam/ { print }

import (
	"os"
	"strings"
	"testing"

	"awk-go/interpreter"
	"awk-go/lexer"
	"awk-go/parser"
)

const testInputPath = "../testdata/test_input.txt"

func runAWK(t *testing.T, program, input string) string {
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
		t.Fatalf("failed to read %q: %v", testInputPath, err)
	}
	return strings.ReplaceAll(string(data), "\r\n", "\n")
}

func check(t *testing.T, got, want string) {
	t.Helper()
	if got != want {
		t.Fatalf("\n  got:  %q\n  want: %q", got, want)
	}
}

// ─────────────────────────────────────────────────────────────────────────────
// Feature 1 — print
// ─────────────────────────────────────────────────────────────────────────────

func TestPrint(t *testing.T) {
	input := "Sam 67\nRam 78\nCam 89"

	t.Run(`{ print }`, func(t *testing.T) {
		// pass: Sam 67 / Ram 78 / Cam 89
		check(t, runAWK(t, `{ print }`, input), "Sam 67\nRam 78\nCam 89\n")
	})

	t.Run(`{ print "hello" }`, func(t *testing.T) {
		// pass: hello / hello / hello
		check(t, runAWK(t, `{ print "hello" }`, "a\nb\nc"), "hello\nhello\nhello\n")
	})

	t.Run(`{ print "Name:", $1 }`, func(t *testing.T) {
		// pass: Name: Sam / Name: Ram
		check(t, runAWK(t, `{ print "Name:", $1 }`, "Sam 67\nRam 78"), "Name: Sam\nName: Ram\n")
	})
}

// ─────────────────────────────────────────────────────────────────────────────
// Feature 2 — Fields  ($0, $1, $n, $NF, NF, NR)
// ─────────────────────────────────────────────────────────────────────────────

func TestFields(t *testing.T) {
	input := "Sam 67\nRam 78\nCam 89"

	t.Run(`{ print $0 }`, func(t *testing.T) {
		// pass: Sam 67 / Ram 78 / Cam 89
		check(t, runAWK(t, `{ print $0 }`, input), input+"\n")
	})

	t.Run(`{ print $1 }`, func(t *testing.T) {
		// pass: Sam / Ram / Cam
		check(t, runAWK(t, `{ print $1 }`, input), "Sam\nRam\nCam\n")
	})

	t.Run(`{ print $2 }`, func(t *testing.T) {
		// pass: 67 / 78 / 89
		check(t, runAWK(t, `{ print $2 }`, input), "67\n78\n89\n")
	})

	t.Run(`{ print $1, $2 }`, func(t *testing.T) {
		// pass: Sam 67 / Ram 78 / Cam 89
		check(t, runAWK(t, `{ print $1, $2 }`, input), "Sam 67\nRam 78\nCam 89\n")
	})

	t.Run(`{ print $NF }`, func(t *testing.T) {
		// pass: c / 4
		check(t, runAWK(t, `{ print $NF }`, "a b c\n1 2 3 4"), "c\n4\n")
	})

	t.Run(`{ print NF }`, func(t *testing.T) {
		// pass: 3 / 2
		check(t, runAWK(t, `{ print NF }`, "a b c\n1 2"), "3\n2\n")
	})

	t.Run(`{ print NR }`, func(t *testing.T) {
		// pass: 1 / 2 / 3
		check(t, runAWK(t, `{ print NR }`, "a\nb\nc"), "1\n2\n3\n")
	})

	t.Run(`{ print NR, $1 }`, func(t *testing.T) {
		// pass: 1 Sam / 2 Ram / 3 Cam
		check(t, runAWK(t, `{ print NR, $1 }`, input), "1 Sam\n2 Ram\n3 Cam\n")
	})

	t.Run(`{ print $5 } (out of range)`, func(t *testing.T) {
		// pass: (empty line)
		check(t, runAWK(t, `{ print $5 }`, "a b"), "\n")
	})
}

// ─────────────────────────────────────────────────────────────────────────────
// Feature 3 — BEGIN / END
// ─────────────────────────────────────────────────────────────────────────────

func TestBegin(t *testing.T) {
	t.Run(`BEGIN { print "start" }`, func(t *testing.T) {
		// pass: start
		check(t, runAWK(t, `BEGIN { print "start" }`, "line1\nline2"), "start\n")
	})

	t.Run(`BEGIN { print "hello" } (no input)`, func(t *testing.T) {
		// pass: hello
		check(t, runAWK(t, `BEGIN { print "hello" }`, ""), "hello\n")
	})

	t.Run(`BEGIN { print "first" } BEGIN { print "second" }`, func(t *testing.T) {
		// pass: first / second
		check(t, runAWK(t, `BEGIN { print "first" } BEGIN { print "second" }`, ""), "first\nsecond\n")
	})
}

func TestEnd(t *testing.T) {
	t.Run(`END { print "done" }`, func(t *testing.T) {
		// pass: done
		check(t, runAWK(t, `END { print "done" }`, "a\nb\nc"), "done\n")
	})

	t.Run(`END { print NR }`, func(t *testing.T) {
		// pass: 3
		check(t, runAWK(t, `END { print NR }`, "a\nb\nc"), "3\n")
	})

	t.Run(`END { print "first" } END { print "second" }`, func(t *testing.T) {
		// pass: first / second
		check(t, runAWK(t, `END { print "first" } END { print "second" }`, "x"), "first\nsecond\n")
	})
}

func TestBeginEnd(t *testing.T) {
	t.Run(`BEGIN { print "start" } END { print "end" }`, func(t *testing.T) {
		// pass: start / end
		check(t, runAWK(t, `BEGIN { print "start" } END { print "end" }`, "a\nb"), "start\nend\n")
	})

	t.Run(`BEGIN { print "---" } { print $1 } END { print "---" }`, func(t *testing.T) {
		// pass: --- / Sam / Ram / ---
		check(t,
			runAWK(t, `BEGIN { print "---" } { print $1 } END { print "---" }`, "Sam 67\nRam 78"),
			"---\nSam\nRam\n---\n",
		)
	})

	t.Run(`BEGIN { print "Name Marks" } { print $1, $2 } END { print "Total:", NR }`, func(t *testing.T) {
		// pass: Name Marks / Sam 67 / Ram 78 / Cam 89 / Total: 3
		check(t,
			runAWK(t,
				`BEGIN { print "Name Marks" } { print $1, $2 } END { print "Total:", NR }`,
				"Sam 67\nRam 78\nCam 89",
			),
			"Name Marks\nSam 67\nRam 78\nCam 89\nTotal: 3\n",
		)
	})
}

// ─────────────────────────────────────────────────────────────────────────────
// Feature 3 — Regex patterns  /pattern/ { action }
// ─────────────────────────────────────────────────────────────────────────────

func TestRegex(t *testing.T) {
	input := "Sam 67\nRam 78\nCam 89"

	t.Run(`/Sam/ { print }`, func(t *testing.T) {
		// pass: Sam 67
		check(t, runAWK(t, `/Sam/ { print }`, input), "Sam 67\n")
	})

	t.Run(`/xyz/ { print } (no match)`, func(t *testing.T) {
		// pass: (no output)
		check(t, runAWK(t, `/xyz/ { print }`, input), "")
	})

	t.Run(`/am/ { print $1 }`, func(t *testing.T) {
		// pass: Sam / Ram / Cam
		check(t, runAWK(t, `/am/ { print $1 }`, input), "Sam\nRam\nCam\n")
	})

	t.Run(`/^S/ { print $1 }`, func(t *testing.T) {
		// pass: Sam / Sue
		check(t, runAWK(t, `/^S/ { print $1 }`, "Sam 67\nRam 78\nSue 90"), "Sam\nSue\n")
	})

	t.Run(`/67/ { print $1 }`, func(t *testing.T) {
		// pass: Sam
		check(t, runAWK(t, `/67/ { print $1 }`, input), "Sam\n")
	})

	t.Run(`BEGIN { print "Matches:" } /Ram/ { print $1, $2 } END { print "done" }`, func(t *testing.T) {
		// pass: Matches: / Ram 78 / done
		check(t,
			runAWK(t,
				`BEGIN { print "Matches:" } /Ram/ { print $1, $2 } END { print "done" }`,
				input,
			),
			"Matches:\nRam 78\ndone\n",
		)
	})
}

// ─────────────────────────────────────────────────────────────────────────────
// File-based tests  (testdata/test_input.txt — 100 employee records)
// Format: E001 Alice Engineering 95000 5
// ─────────────────────────────────────────────────────────────────────────────

func TestFile(t *testing.T) {
	input := loadTestInput(t)

	t.Run(`{ print NR } → 1 to 100`, func(t *testing.T) {
		got := runAWK(t, `{ print NR }`, input)
		lines := strings.Split(strings.TrimRight(got, "\n"), "\n")
		if len(lines) != 100 {
			t.Fatalf("expected 100 lines, got %d", len(lines))
		}
		if lines[0] != "1" || lines[99] != "100" {
			t.Fatalf("NR range wrong: first=%q last=%q", lines[0], lines[99])
		}
	})

	t.Run(`{ print $1 } → E001 to E100`, func(t *testing.T) {
		got := runAWK(t, `{ print $1 }`, input)
		lines := strings.Split(strings.TrimRight(got, "\n"), "\n")
		if len(lines) != 100 {
			t.Fatalf("expected 100 lines, got %d", len(lines))
		}
		if lines[0] != "E001" || lines[99] != "E100" {
			t.Fatalf("$1 range wrong: first=%q last=%q", lines[0], lines[99])
		}
	})

	t.Run(`{ print $0 } → matches input`, func(t *testing.T) {
		got := runAWK(t, `{ print $0 }`, input)
		if strings.TrimRight(got, "\n") != strings.TrimRight(input, "\n") {
			t.Fatalf("$0 output does not match input")
		}
	})

	t.Run(`BEGIN { print "START" } END { print "END", NR }`, func(t *testing.T) {
		// pass: START / END 100
		check(t,
			runAWK(t, `BEGIN { print "START" } END { print "END", NR }`, input),
			"START\nEND 100\n",
		)
	})

	t.Run(`/Engineering/ { print $1, $2 } → 27 employees`, func(t *testing.T) {
		got := runAWK(t, `/Engineering/ { print $1, $2 }`, input)
		lines := strings.Split(strings.TrimRight(got, "\n"), "\n")
		if len(lines) != 27 {
			t.Fatalf("expected 27 Engineering employees, got %d", len(lines))
		}
		if lines[0] != "E001 Alice" {
			t.Fatalf("first: got %q, want %q", lines[0], "E001 Alice")
		}
	})

	t.Run(`/Finance/ { print $1, $2 } → 24 employees`, func(t *testing.T) {
		got := runAWK(t, `/Finance/ { print $1, $2 }`, input)
		lines := strings.Split(strings.TrimRight(got, "\n"), "\n")
		if len(lines) != 24 {
			t.Fatalf("expected 24 Finance employees, got %d", len(lines))
		}
	})

	t.Run(`BEGIN { print "Finance dept:" } /Finance/ { print $2 } END { print "---" }`, func(t *testing.T) {
		// pass: Finance dept: / <24 names> / ---
		got := runAWK(t, `BEGIN { print "Finance dept:" } /Finance/ { print $2 } END { print "---" }`, input)
		lines := strings.Split(strings.TrimRight(got, "\n"), "\n")
		if len(lines) != 26 {
			t.Fatalf("expected 26 lines, got %d", len(lines))
		}
		if lines[0] != "Finance dept:" {
			t.Fatalf("header: got %q", lines[0])
		}
		if lines[len(lines)-1] != "---" {
			t.Fatalf("footer: got %q", lines[len(lines)-1])
		}
	})
}
