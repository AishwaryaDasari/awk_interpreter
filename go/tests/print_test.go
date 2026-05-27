package tests

import (
	"strings"
	"testing"
)

func TestPrintDefault(t *testing.T) {
	t.Run(`{ print } prints $0 + newline`, func(t *testing.T) {
		check(t, runAWK(t, `{ print }`, "Sam 67\nRam 78"), "Sam 67\nRam 78\n")
	})
}

func TestPrintStringLiteral(t *testing.T) {
	t.Run(`{ print "hello" } prints string on each line`, func(t *testing.T) {
		got := runAWK(t, `{ print "hello" }`, "a\nb\nc")
		check(t, got, "hello\nhello\nhello\n")
	})
}

func TestPrintMultipleArgs(t *testing.T) {
	t.Run(`{ print "Name:", $1 } prints args space-separated`, func(t *testing.T) {
		got := runAWK(t, `{ print "Name:", $1 }`, "Sam 67\nRam 78")
		check(t, got, "Name: Sam\nName: Ram\n")
	})
}

func TestPrintFields(t *testing.T) {
	t.Run(`{ print $1, $2 } prints multiple fields`, func(t *testing.T) {
		got := runAWK(t, `{ print $1, $2 }`, "Sam 67\nRam 78\nCam 89")
		check(t, got, "Sam 67\nRam 78\nCam 89\n")
	})
}

func TestPrintEmptyInput(t *testing.T) {
	t.Run(`{ print } with empty input prints nothing`, func(t *testing.T) {
		check(t, runAWK(t, `{ print }`, ""), "")
	})
}

func TestPrintEdgeCases(t *testing.T) {
	t.Run(`{ print } with single field prints that field`, func(t *testing.T) {
		check(t, runAWK(t, `{ print }`, "onlyfield"), "onlyfield\n")
	})

	t.Run(`{ print "a", "b" } prints two string args`, func(t *testing.T) {
		check(t, runAWK(t, `{ print "a", "b" }`, "x"), "a b\n")
	})

	t.Run(`{ print NR } prints record number directly`, func(t *testing.T) {
		check(t, runAWK(t, `{ print NR }`, "a\nb"), "1\n2\n")
	})
}

func TestPrintNR(t *testing.T) {
	t.Run(`{ print NR, $0 } prints record number then line`, func(t *testing.T) {
		got := runAWK(t, `{ print NR, $0 }`, "a\nb\nc")
		lines := strings.Split(strings.TrimRight(got, "\n"), "\n")
		if len(lines) != 3 {
			t.Fatalf("expected 3 lines, got %d", len(lines))
		}
		if lines[0] != "1 a" {
			t.Fatalf("line 1: got %q", lines[0])
		}
		if lines[2] != "3 c" {
			t.Fatalf("line 3: got %q", lines[2])
		}
	})
}
