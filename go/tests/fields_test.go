package tests

import (
	"strings"
	"testing"
)

func TestFieldZero(t *testing.T) {
	t.Run(`{ print $0 } prints full record`, func(t *testing.T) {
		input := "Sam 67\nRam 78"
		check(t, runAWK(t, `{ print $0 }`, input), input+"\n")
	})
}

func TestFieldOne(t *testing.T) {
	t.Run(`{ print $1 } prints first field`, func(t *testing.T) {
		check(t, runAWK(t, `{ print $1 }`, "Sam 67\nRam 78"), "Sam\nRam\n")
	})
}

func TestFieldTwo(t *testing.T) {
	t.Run(`{ print $2 } prints second field`, func(t *testing.T) {
		check(t, runAWK(t, `{ print $2 }`, "Sam 67\nRam 78"), "67\n78\n")
	})
}

func TestFieldNF(t *testing.T) {
	t.Run(`{ print $NF } prints last field`, func(t *testing.T) {
		got := runAWK(t, `{ print $NF }`, "a b c\n1 2 3 4")
		check(t, got, "c\n4\n")
	})
}

func TestBuiltinNF(t *testing.T) {
	t.Run(`{ print NF } prints field count`, func(t *testing.T) {
		got := runAWK(t, `{ print NF }`, "a b c\n1 2")
		check(t, got, "3\n2\n")
	})
}

func TestBuiltinNR(t *testing.T) {
	t.Run(`{ print NR } prints record number`, func(t *testing.T) {
		got := runAWK(t, `{ print NR }`, "a\nb\nc")
		check(t, got, "1\n2\n3\n")
	})
}

func TestFieldWithNR(t *testing.T) {
	t.Run(`{ print NR, $1 } prints record number and first field`, func(t *testing.T) {
		got := runAWK(t, `{ print NR, $1 }`, "Sam 67\nRam 78\nCam 89")
		check(t, got, "1 Sam\n2 Ram\n3 Cam\n")
	})
}

func TestFieldOutOfRange(t *testing.T) {
	t.Run(`{ print $99 } out of range returns empty`, func(t *testing.T) {
		check(t, runAWK(t, `{ print $99 }`, "a b"), "\n")
	})
}

func TestFieldWithNF(t *testing.T) {
	t.Run(`{ print $1, NF } prints field and count`, func(t *testing.T) {
		got := runAWK(t, `{ print $1, NF }`, "alpha beta gamma\none two")
		check(t, got, "alpha 3\none 2\n")
	})
}

func TestFieldFileData(t *testing.T) {
	t.Run(`{ print $1 } on employee file yields E001..E100`, func(t *testing.T) {
		input := loadTestInput(t)
		got := runAWK(t, `{ print $1 }`, input)
		lines := strings.Split(strings.TrimRight(got, "\n"), "\n")
		if len(lines) != 100 {
			t.Fatalf("expected 100 lines, got %d", len(lines))
		}
		if lines[0] != "E001" {
			t.Fatalf("first $1: got %q, want %q", lines[0], "E001")
		}
		if lines[99] != "E100" {
			t.Fatalf("last $1: got %q, want %q", lines[99], "E100")
		}
	})
}


