package tests

import (
	"testing"
)

func TestBeginSingle(t *testing.T) {
	t.Run(`BEGIN { print "start" } prints before any lines`, func(t *testing.T) {
		got := runAWK(t, `BEGIN { print "start" }`, "line1\nline2")
		check(t, got, "start\n")
	})
}

func TestBeginNoInput(t *testing.T) {
	t.Run(`BEGIN { print "hello" } with empty input still runs`, func(t *testing.T) {
		got := runAWK(t, `BEGIN { print "hello" }`, "")
		check(t, got, "hello\n")
	})
}

func TestBeginMultiple(t *testing.T) {
	t.Run(`Multiple BEGIN blocks execute in order`, func(t *testing.T) {
		got := runAWK(t, `BEGIN { print "first" } BEGIN { print "second" }`, "")
		check(t, got, "first\nsecond\n")
	})
}

func TestBeginVariables(t *testing.T) {
	t.Run(`BEGIN block can set and print variables`, func(t *testing.T) {
		got := runAWK(t, `BEGIN { x = 42; print x }`, "")
		check(t, got, "42\n")
	})
}

func TestEndSingle(t *testing.T) {
	t.Run(`END { print "done" } prints after all lines`, func(t *testing.T) {
		got := runAWK(t, `END { print "done" }`, "a\nb\nc")
		check(t, got, "done\n")
	})
}

func TestEndWithNR(t *testing.T) {
	t.Run(`END { print NR } prints total record count`, func(t *testing.T) {
		got := runAWK(t, `END { print NR }`, "a\nb\nc")
		check(t, got, "3\n")
	})
}

func TestEndMultiple(t *testing.T) {
	t.Run(`Multiple END blocks execute in order`, func(t *testing.T) {
		got := runAWK(t, `END { print "first" } END { print "second" }`, "x")
		check(t, got, "first\nsecond\n")
	})
}

func TestBeginEndCombined(t *testing.T) {
	t.Run(`BEGIN + END without body prints both bookends`, func(t *testing.T) {
		got := runAWK(t, `BEGIN { print "start" } END { print "end" }`, "a\nb")
		check(t, got, "start\nend\n")
	})
}

func TestBeginBodyEnd(t *testing.T) {
	t.Run(`BEGIN { print "---" } { print $1 } END { print "---" }`, func(t *testing.T) {
		got := runAWK(t, `BEGIN { print "---" } { print $1 } END { print "---" }`, "Sam 67\nRam 78")
		check(t, got, "---\nSam\nRam\n---\n")
	})
}

func TestBeginEndFull(t *testing.T) {
	t.Run(`Full report: header, data rows, summary`, func(t *testing.T) {
		got := runAWK(t,
			`BEGIN { print "Name Marks" } { print $1, $2 } END { print "Total:", NR }`,
			"Sam 67\nRam 78\nCam 89",
		)
		check(t, got, "Name Marks\nSam 67\nRam 78\nCam 89\nTotal: 3\n")
	})
}

func TestBeginEndFileData(t *testing.T) {
	t.Run(`BEGIN { print "START" } END { print "END", NR } on employee file`, func(t *testing.T) {
		input := loadTestInput(t)
		got := runAWK(t, `BEGIN { print "START" } END { print "END", NR }`, input)
		check(t, got, "START\nEND 100\n")
	})
}


