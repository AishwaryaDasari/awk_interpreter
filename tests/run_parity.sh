#!/usr/bin/env bash
# run_parity.sh - Run awk vs awk_rust and report which test cases fail
# Usage: ./tests/run_parity.sh [awk_rust_binary]
#        If no binary provided, tries common build paths.

set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
TESTS="$ROOT/tests"

# --- Locate awk_rust binary ---
AWK_RUST="${1:-}"
if [ -z "$AWK_RUST" ]; then
    candidates=(
        "$ROOT/rust/target/release/awk_rust"
        "$ROOT/rust/target/debug/awk_rust"
        "$ROOT/rust/target/release/awk_interpreter"
        "$ROOT/rust/target/debug/awk_interpreter"
    )
    for c in "${candidates[@]}"; do
        if [ -x "$c" ]; then
            AWK_RUST="$c"
            break
        fi
    done
fi

if [ ! -x "$AWK_RUST" ]; then
    echo "ERROR: awk_rust binary not found."
    echo "Build it first: cd rust && cargo build --release"
    echo "Or provide path:  ./tests/run_parity.sh /path/to/awk_rust"
    exit 1
fi

AWK="awk"
PASS=0
FAIL=0
TOTAL=0

# --- Helpers ---
run_test() {
    local name="$1"        # test name for display
    local script="$2"      # awk script file
    local input="$3"       # input file
    local awk_opts="${4:-}" # extra awk flags e.g. -F,
    local awk_stdout awk_stderr rust_stdout rust_stderr rc_awk rc_rust

    TOTAL=$((TOTAL + 1))
    printf "\n[ RUN ] %s\n" "$name"

    # Build temp files for output
    local awk_out; awk_out=$(mktemp)
    local rust_out; rust_out=$(mktemp)

    set +e
    awk_stdout=$(awk $awk_opts -f "$script" "$input" 2>"$awk_out"; printf "%s" "$?")
    awk_stderr=$(<"$awk_out")
    rm -f "$awk_out"

    rust_stdout=$("$AWK_RUST" $awk_opts -f "$script" "$input" 2>"$rust_out"; printf "%s" "$?")
    rust_stderr=$(<"$rust_out")
    rm -f "$rust_out"
    set -e

    rc_awk="${awk_stdout: -1}"
    rc_rust="${rust_stdout: -1}"
    awk_stdout="${awk_stdout%?}"
    rust_stdout="${rust_stdout%?}"

    local fail_reason=""

    # Compare exit codes
    if [ "$rc_awk" != "$rc_rust" ]; then
        fail_reason="exit code mismatch (awk=$rc_awk, awk_rust=$rc_rust)"
    fi

    # Compare stdout
    if [ -z "$fail_reason" ] && [ "$awk_stdout" != "$rust_stdout" ]; then
        fail_reason="stdout differs"
    fi

    if [ -n "$fail_reason" ]; then
        echo "[FAIL] $name: $fail_reason"
        FAIL=$((FAIL + 1))

        # Show diff if stdout differs
        if [ "$fail_reason" = "stdout differs" ]; then
            local diff_file; diff_file=$(mktemp)
            echo "$awk_stdout" > /tmp/awk_out.$$.txt
            echo "$rust_stdout" > /tmp/rust_out.$$.txt
            diff /tmp/awk_out.$$.txt /tmp/rust_out.$$.txt > "$diff_file" || true
            echo "------- diff (awk LEFT vs awk_rust RIGHT) -------"
            sed 's/^/  /' "$diff_file"
            echo "-----------------------------------------------"
            rm -f /tmp/awk_out.$$.txt /tmp/rust_out.$$.txt "$diff_file"
        fi

        # Show stderr if any
        if [ -n "$awk_stderr" ] || [ -n "$rust_stderr" ]; then
            [ -n "$awk_stderr" ]  && echo "  awk stderr:    $awk_stderr"
            [ -n "$rust_stderr" ] && echo "  awk_rust stderr: $rust_stderr"
        fi
    elif [ -n "$awk_stderr" ] || [ -n "$rust_stderr" ]; then
        echo "[WARN] $name: outputs match, but stderr differs"
        [ -n "$awk_stderr" ]  && echo "  awk stderr:    $awk_stderr"
        [ -n "$rust_stderr" ] && echo "  awk_rust stderr: $rust_stderr"
        PASS=$((PASS + 1))
    else
        echo "[PASS] $name"
        PASS=$((PASS + 1))
    fi
}

# ============================================================
# TEST SUITES
# ============================================================
echo "============================================"
echo " awk vs awk_rust parity tests"
echo " awk_rust: $AWK_RUST"
echo " awk:      $(command -v awk)"
echo "============================================"

# --- script.awk_rust (if-else tests) ---
# Tests 1,2,4,5,6,8,9,10,11  -> test_input.txt
run_test "if-else: pattern, NR, NF, dept, ranges, nested, compound, ternary, prefix" \
    "$TESTS/script.awk_rust" "$TESTS/test_input.txt"

# Test 3 -> test1.txt (marks)
run_test "if-else: if-else-if chain (marks)" \
    "$TESTS/script.awk_rust" "$TESTS/test1.txt"

# Test 7 -> test2.csv (grades with -F,)
run_test "if-else: CSV grades with -F," \
    "$TESTS/script.awk_rust" "$TESTS/test2.csv" "-F,"

# --- script_loops.awk_rust (loop tests) ---
# Tests 1-12 -> test_input.txt
run_test "loops: while, for, for-in, do-while, nested, break, continue, arrays" \
    "$TESTS/script_loops.awk_rust" "$TESTS/test_input.txt"

# Loops -> test2.csv with -F,
run_test "loops: CSV with -F," \
    "$TESTS/script_loops.awk_rust" "$TESTS/test2.csv" "-F,"

# Loops -> test1.txt
run_test "loops: test1.txt" \
    "$TESTS/script_loops.awk_rust" "$TESTS/test1.txt"

# ============================================================
# SUMMARY
# ============================================================
echo ""
echo "============================================"
echo " RESULTS: $PASS passed, $FAIL failed out of $TOTAL tests"
echo "============================================"

if [ "$FAIL" -gt 0 ]; then
    exit 1
fi
