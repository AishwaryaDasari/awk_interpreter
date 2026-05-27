package tests

import (
	"strings"
	"testing"
)

func TestRegexExactMatch(t *testing.T) {
	t.Run(`/Sam/ { print } matches lines containing Sam`, func(t *testing.T) {
		got := runAWK(t, `/Sam/ { print }`, "Sam 67\nRam 78\nCam 89")
		check(t, got, "Sam 67\n")
	})
}

func TestRegexNoMatch(t *testing.T) {
	t.Run(`/xyz/ { print } produces no output for no match`, func(t *testing.T) {
		got := runAWK(t, `/xyz/ { print }`, "Sam 67\nRam 78")
		check(t, got, "")
	})
}

func TestRegexPartialMatch(t *testing.T) {
	t.Run(`/am/ { print $1 } matches all three records`, func(t *testing.T) {
		got := runAWK(t, `/am/ { print $1 }`, "Sam 67\nRam 78\nCam 89")
		check(t, got, "Sam\nRam\nCam\n")
	})
}

func TestRegexAnchored(t *testing.T) {
	t.Run(`/^S/ { print $1 } matches lines starting with S`, func(t *testing.T) {
		got := runAWK(t, `/^S/ { print $1 }`, "Sam 67\nRam 78\nSue 90")
		check(t, got, "Sam\nSue\n")
	})
}

func TestRegexNumeric(t *testing.T) {
	t.Run(`/67/ { print $1 } matches lines containing 67`, func(t *testing.T) {
		got := runAWK(t, `/67/ { print $1 }`, "Sam 67\nRam 78\nCam 89")
		check(t, got, "Sam\n")
	})
}

func TestRegexWithBeginEnd(t *testing.T) {
	t.Run(`BEGIN + /Ram/ regex + END combined`, func(t *testing.T) {
		got := runAWK(t,
			`BEGIN { print "Matches:" } /Ram/ { print $1, $2 } END { print "done" }`,
			"Sam 67\nRam 78\nCam 89",
		)
		check(t, got, "Matches:\nRam 78\ndone\n")
	})
}

func TestRegexWithAlwaysRule(t *testing.T) {
	t.Run(`always rule + regex rule together`, func(t *testing.T) {
		got := runAWK(t,
			`{ print NR } /Sam/ { print $1 }`,
			"Sam 67\nRam 78",
		)
		check(t, got, "1\nSam\n2\n")
	})
}

func TestRegexEngineerCount(t *testing.T) {
	t.Run(`/Engineering/ on employee file yields 27 records`, func(t *testing.T) {
		input := loadTestInput(t)
		got := runAWK(t, `/Engineering/ { print $1, $2 }`, input)
		lines := strings.Split(strings.TrimRight(got, "\n"), "\n")
		if len(lines) != 27 {
			t.Fatalf("expected 27 Engineering employees, got %d", len(lines))
		}
		if lines[0] != "E001 Alice" {
			t.Fatalf("first Engineering record: got %q, want %q", lines[0], "E001 Alice")
		}
	})
}

func TestRegexFinanceCount(t *testing.T) {
	t.Run(`/Finance/ on employee file yields 24 records`, func(t *testing.T) {
		input := loadTestInput(t)
		got := runAWK(t, `/Finance/ { print $1, $2 }`, input)
		lines := strings.Split(strings.TrimRight(got, "\n"), "\n")
		if len(lines) != 24 {
			t.Fatalf("expected 24 Finance employees, got %d", len(lines))
		}
	})
}

func TestRegexFullReport(t *testing.T) {
	t.Run(`BEGIN + /Finance/ regex + END full report`, func(t *testing.T) {
		input := loadTestInput(t)
		got := runAWK(t, `BEGIN { print "Finance dept:" } /Finance/ { print $2 } END { print "---" }`, input)
		lines := strings.Split(strings.TrimRight(got, "\n"), "\n")
		if len(lines) != 26 {
			t.Fatalf("expected 26 lines (header + 24 names + footer), got %d", len(lines))
		}
		if lines[0] != "Finance dept:" {
			t.Fatalf("header: got %q", lines[0])
		}
		if lines[len(lines)-1] != "---" {
			t.Fatalf("footer: got %q", lines[len(lines)-1])
		}
	})
}


