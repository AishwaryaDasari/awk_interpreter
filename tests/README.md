AWK Interpreter - Test Suite
==============================

This directory contains end-to-end test cases for the AWK interpreter.

Structure
---------

  run_tests.sh            Automated test runner (bash script)
  *.txt                   Input data files
  expected/*.txt          Expected output for each test case

Data Files
----------

  test1.txt               Student names and marks (4 lines, 2 cols)
  employees.txt           Employee records: name, dept, salary, years (8 lines, 4 cols)
  products.txt            Product sales: product, region, units, price (8 lines, 5 cols)
  whitespace.txt          Names and scores with irregular whitespace (6 lines, 2 cols)
  empty_lines.txt         Lines with blank lines interspersed (5 lines, 2 cols)
  numbers.txt             Purely numeric data (4 lines, 3 cols)
  single_col.txt          Single-column text (5 lines, 1 col)
  comma_sep.txt           Comma-separated values (5 lines, 1 col — header + data)
  grades.txt              Student names and letter grades (6 lines, 2 cols)

Running Tests
-------------

  From the project root:

    ./tests/run_tests.sh ./rust/target/release/awk_interpreter

  Or build first and let the script auto-detect:

    cd rust && cargo build --release
    ./tests/run_tests.sh

Test Coverage
-------------

Each data file is tested with these AWK programs:

  Program                         What it tests
  ------------------------------  -----------------------------------------
  { print }                       Printing the full line ($0)
  { print $1 }                    First field access
  { print $2 }                    Second field access
  { print $1, $2 }                Multiple comma-separated fields
  { print $1, $2, $3 }            Three fields
  { print $1, $2, $3, $4, $5 }   Many fields
  { print NF }                    Number of fields
  { print NR }                    Record number
  { print NR, $1 }                Combining NR with field access
  { print $NF }                   Dynamic field access via $NF
  { print "text", $1 }            String literal mixed with field
  { print 42 }                    Numeric literal
  { print $1; print $2 }          Multiple statements with semicolon

Edge Cases Covered
------------------

  - Empty lines (empty_lines.txt)
  - Leading/trailing whitespace (whitespace.txt)
  - Custom field separators (comma_sep.txt)
  - Single-column data (single_col.txt)
  - Out-of-range field access ($99)
  - Multiple records and NR incrementing
