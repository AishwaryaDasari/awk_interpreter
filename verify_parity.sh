#!/usr/bin/env bash
set -euo pipefail

GO="${GO:-go}"
if ! command -v "$GO" &>/dev/null; then
	GO="/tmp/go/bin/go"
fi

ROOT="$(cd "$(dirname "$0")" && pwd)"

TEST_FILE="$ROOT/tests/test_input.txt"
if [ ! -f "$TEST_FILE" ]; then
	echo "FAIL: test input not found at $TEST_FILE"
	exit 1
fi

PROGRAMS=(
	'{ print NR }'
	'{ print $1 }'
	'{ print NF }'
	'{ print $1, $NF }'
	'{ print $0 }'
)

PASS=0
FAIL=0

for prog in "${PROGRAMS[@]}"; do
	go_out="$(cd "$ROOT/awk-go" && "$GO" run main.go "$prog" "$TEST_FILE" 2>/dev/null || true)"
	awk_out="$(awk "$prog" "$TEST_FILE" 2>/dev/null || true)"

	if [ "$go_out" = "$awk_out" ]; then
		echo "[PASS] $prog"
		PASS=$((PASS + 1))
	else
		echo "[FAIL] $prog"
		FAIL=$((FAIL + 1))
	fi
done

echo "---"
echo "Results: $PASS passed, $FAIL failed"
