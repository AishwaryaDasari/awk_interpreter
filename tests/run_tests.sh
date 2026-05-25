#!/usr/bin/env bash
#
# AWK Interpreter Test Runner
# Usage: ./run_tests.sh [path-to-awk-interpreter-binary]
#
# If no binary path is given, it looks for:
#   1. ../rust/target/release/awk_rust
#   2. ../rust/target/debug/awk_rust

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
BIN="${1:-}"

if [ -z "$BIN" ]; then
    if [ -f "$SCRIPT_DIR/../rust/target/release/awk_rust" ]; then
        BIN="$SCRIPT_DIR/../rust/target/release/awk_rust"
    elif [ -f "$SCRIPT_DIR/../rust/target/debug/awk_rust" ]; then
        BIN="$SCRIPT_DIR/../rust/target/debug/awk_rust"
    else
        echo "ERROR: awk_interpreter binary not found."
        echo "Build it first: cd rust && cargo build"
        echo "Then run:  ./tests/run_tests.sh ./rust/target/release/awk_interpreter"
        exit 1
    fi
fi

if [ ! -x "$BIN" ]; then
    echo "ERROR: $BIN is not executable or not found."
    exit 1
fi

PASSED=0
FAILED=0
FAILED_TESTS=""

run_test() {
    local name="$1"
    local program="$2"
    local data_file="$3"
    local expected_file="$4"

    local actual
    actual=$("$BIN" "$program" "$data_file" 2>&1 || true)

    if [ ! -f "$expected_file" ]; then
        echo "  FAIL  $name  (expected file missing: $expected_file)"
        echo "    Actual output was:"
        echo "$actual" | sed 's/^/    /'
        FAILED=$((FAILED + 1))
        FAILED_TESTS="$FAILED_TESTS  - $name\n"
        return
    fi

    local expected
    expected=$(cat "$expected_file")

    if [ "$actual" = "$expected" ]; then
        echo "  PASS  $name"
        PASSED=$((PASSED + 1))
    else
        echo "  FAIL  $name"
        echo "    Expected:"
        echo "$expected" | sed 's/^/      > /'
        echo "    Actual:"
        echo "$actual" | sed 's/^/      > /'
        FAILED=$((FAILED + 1))
        FAILED_TESTS="$FAILED_TESTS  - $name\n"
    fi
}

echo "============================================"
echo "  AWK Interpreter - Test Suite"
echo "  Binary: $BIN"
echo "============================================"
echo ""

# ==================== test1.txt (Students) ====================
echo "--- test1.txt (Students) ---"

run_test "print \$0" \
    '{ print }' \
    "$SCRIPT_DIR/test1.txt" \
    "$SCRIPT_DIR/expected/print_all_test1.txt"

run_test "print \$1" \
    '{ print $1 }' \
    "$SCRIPT_DIR/test1.txt" \
    "$SCRIPT_DIR/expected/print_field1_test1.txt"

run_test "print \$2" \
    '{ print $2 }' \
    "$SCRIPT_DIR/test1.txt" \
    "$SCRIPT_DIR/expected/print_field2_test1.txt"

run_test "print \$1, \$2" \
    '{ print $1, $2 }' \
    "$SCRIPT_DIR/test1.txt" \
    "$SCRIPT_DIR/expected/print_fields_12_test1.txt"

run_test "print NF" \
    '{ print NF }' \
    "$SCRIPT_DIR/test1.txt" \
    "$SCRIPT_DIR/expected/print_nf_test1.txt"

run_test "print NR" \
    '{ print NR }' \
    "$SCRIPT_DIR/test1.txt" \
    "$SCRIPT_DIR/expected/print_nr_test1.txt"

run_test "print \$NF (last field)" \
    '{ print $NF }' \
    "$SCRIPT_DIR/test1.txt" \
    "$SCRIPT_DIR/expected/print_last_test1.txt"

run_test "print string label + \$1" \
    '{ print "Name:", $1 }' \
    "$SCRIPT_DIR/test1.txt" \
    "$SCRIPT_DIR/expected/print_label_test1.txt"

run_test "print NR and \$1" \
    '{ print NR, $1 }' \
    "$SCRIPT_DIR/test1.txt" \
    "$SCRIPT_DIR/expected/print_nr_field1_test1.txt"

run_test "print \$1; print \$2 (multi-stmt)" \
    '{ print $1; print $2 }' \
    "$SCRIPT_DIR/test1.txt" \
    "$SCRIPT_DIR/expected/print_multi_stmt_test1.txt"

echo ""

# ==================== employees.txt ====================
echo "--- employees.txt ---"

run_test "print all fields" \
    '{ print }' \
    "$SCRIPT_DIR/employees.txt" \
    "$SCRIPT_DIR/expected/print_all_employees.txt"

run_test "print \$1 (names)" \
    '{ print $1 }' \
    "$SCRIPT_DIR/employees.txt" \
    "$SCRIPT_DIR/expected/print_field1_employees.txt"

run_test "print \$2 (depts)" \
    '{ print $2 }' \
    "$SCRIPT_DIR/employees.txt" \
    "$SCRIPT_DIR/expected/print_field2_employees.txt"

run_test "print \$1, \$2, \$3" \
    '{ print $1, $2, $3 }' \
    "$SCRIPT_DIR/employees.txt" \
    "$SCRIPT_DIR/expected/print_fields_123_employees.txt"

run_test "print NF" \
    '{ print NF }' \
    "$SCRIPT_DIR/employees.txt" \
    "$SCRIPT_DIR/expected/print_nf_employees.txt"

run_test "print NR and \$1" \
    '{ print NR, $1 }' \
    "$SCRIPT_DIR/employees.txt" \
    "$SCRIPT_DIR/expected/print_nr_field1_employees.txt"

run_test "print \$NF (salary)" \
    '{ print $NF }' \
    "$SCRIPT_DIR/employees.txt" \
    "$SCRIPT_DIR/expected/print_last_employees.txt"

echo ""

# ==================== products.txt ====================
echo "--- products.txt ---"

run_test "print all fields" \
    '{ print }' \
    "$SCRIPT_DIR/products.txt" \
    "$SCRIPT_DIR/expected/print_all_products.txt"

run_test "print \$1, \$2 (product+region)" \
    '{ print $1, $2 }' \
    "$SCRIPT_DIR/products.txt" \
    "$SCRIPT_DIR/expected/print_fields_12_products.txt"

run_test "print \$3, \$4, \$5 (units+price)" \
    '{ print $3, $4, $5 }' \
    "$SCRIPT_DIR/products.txt" \
    "$SCRIPT_DIR/expected/print_fields_345_products.txt"

run_test "print NR and \$1" \
    '{ print NR, $1 }' \
    "$SCRIPT_DIR/products.txt" \
    "$SCRIPT_DIR/expected/print_nr_field1_products.txt"

run_test "print \$NF (price)" \
    '{ print $NF }' \
    "$SCRIPT_DIR/products.txt" \
    "$SCRIPT_DIR/expected/print_last_products.txt"

echo ""

# ==================== whitespace.txt ====================
echo "--- whitespace.txt ---"

run_test "print all (irregular whitespace)" \
    '{ print }' \
    "$SCRIPT_DIR/whitespace.txt" \
    "$SCRIPT_DIR/expected/print_all_whitespace.txt"

run_test "print \$1 (names, trimmed)" \
    '{ print $1 }' \
    "$SCRIPT_DIR/whitespace.txt" \
    "$SCRIPT_DIR/expected/print_field1_whitespace.txt"

run_test "print \$2 (scores)" \
    '{ print $2 }' \
    "$SCRIPT_DIR/whitespace.txt" \
    "$SCRIPT_DIR/expected/print_field2_whitespace.txt"

run_test "print NF" \
    '{ print NF }' \
    "$SCRIPT_DIR/whitespace.txt" \
    "$SCRIPT_DIR/expected/print_nf_whitespace.txt"

run_test "print NR, \$1, \$2" \
    '{ print NR, $1, $2 }' \
    "$SCRIPT_DIR/whitespace.txt" \
    "$SCRIPT_DIR/expected/print_nr_fields_whitespace.txt"

echo ""

# ==================== empty_lines.txt ====================
echo "--- empty_lines.txt ---"

run_test "print all (with empty lines)" \
    '{ print }' \
    "$SCRIPT_DIR/empty_lines.txt" \
    "$SCRIPT_DIR/expected/print_all_empty_lines.txt"

run_test "print NR, \$1 (empty lines)" \
    '{ print NR, $1 }' \
    "$SCRIPT_DIR/empty_lines.txt" \
    "$SCRIPT_DIR/expected/print_nr_field1_empty_lines.txt"

echo ""

# ==================== numbers.txt ====================
echo "--- numbers.txt ---"

run_test "print all (numbers)" \
    '{ print }' \
    "$SCRIPT_DIR/numbers.txt" \
    "$SCRIPT_DIR/expected/print_all_numbers.txt"

run_test "print \$1, \$3" \
    '{ print $1, $3 }' \
    "$SCRIPT_DIR/numbers.txt" \
    "$SCRIPT_DIR/expected/print_fields_13_numbers.txt"

run_test "print \$1, \$2, \$3" \
    '{ print $1, $2, $3 }' \
    "$SCRIPT_DIR/numbers.txt" \
    "$SCRIPT_DIR/expected/print_all_fields_numbers.txt"

echo ""

# ==================== single_col.txt ====================
echo "--- single_col.txt ---"

run_test "print all (single column)" \
    '{ print }' \
    "$SCRIPT_DIR/single_col.txt" \
    "$SCRIPT_DIR/expected/print_all_single_col.txt"

run_test "print NR, \$1 (single column)" \
    '{ print NR, $1 }' \
    "$SCRIPT_DIR/single_col.txt" \
    "$SCRIPT_DIR/expected/print_nr_field1_single_col.txt"

echo ""

# ==================== grades.txt ====================
echo "--- grades.txt ---"

run_test "print all (grades)" \
    '{ print }' \
    "$SCRIPT_DIR/grades.txt" \
    "$SCRIPT_DIR/expected/print_all_grades.txt"

run_test "print \$1, \$2 (student+grade)" \
    '{ print $1, $2 }' \
    "$SCRIPT_DIR/grades.txt" \
    "$SCRIPT_DIR/expected/print_fields_12_grades.txt"

run_test "print \$2 (grade only)" \
    '{ print $2 }' \
    "$SCRIPT_DIR/grades.txt" \
    "$SCRIPT_DIR/expected/print_field2_grades.txt"

echo ""

# ==================== Summary ====================
TOTAL=$((PASSED + FAILED))
echo "============================================"
echo "  Results: $PASSED / $TOTAL passed"
echo "============================================"

if [ "$FAILED" -gt 0 ]; then
    echo ""
    echo "Failed tests:"
    echo -e "$FAILED_TESTS"
    exit 1
fi
