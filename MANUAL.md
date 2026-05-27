# AWK Interpreter — User Manual

A working AWK interpreter built from scratch in **Rust**, **Go**, and **Crystal**.
All three versions run the same programs and produce identical output.

> All examples use `awk_rust`. Replace with `awk_go` or `awk_crystal` for identical results.

---

## Table of Contents

1. [Running the Interpreter](#running-the-interpreter)
2. [Program Structure](#program-structure)
3. [Special Patterns](#special-patterns)
4. [Fields and Records](#fields-and-records)
5. [Built-in Variables](#built-in-variables)
6. [Operators](#operators)
7. [Statements](#statements)
8. [Control Flow](#control-flow)
9. [Arrays](#arrays)
10. [Built-in Functions](#built-in-functions)
11. [User-Defined Functions](#user-defined-functions)
12. [CLI Flags](#cli-flags)
13. [Worked Examples](#worked-examples)

---

## Running the Interpreter

All three versions share the same command-line interface:

```
awk_rust    'program' [file...]
awk_go      'program' [file...]
awk_crystal 'program' [file...]
```

- `program` — AWK program in single quotes
- `file` — one or more input files; reads from stdin if none given

```bash
awk_rust '{print}' data.txt
awk_rust '{print $1, $2}' data.txt
echo "hello world" | awk_rust '{print $2}'
```

---

## Program Structure

An AWK program is a sequence of `pattern { action }` rules.

```awk
pattern { action }
pattern { action }
...
```

- If **pattern** is omitted, the action runs on every line.
- If **action** is omitted, the matching line is printed.
- Multiple rules are evaluated in order for each input line.

```awk
{ print }              # no pattern — runs on every line
/Sam/ { print }        # regex pattern — lines containing "Sam"
NR > 1 { print }       # expression pattern — when NR > 1
$2 > 70 { print $1 }   # field comparison pattern
```

---

## Special Patterns

### BEGIN
Runs once before any input is read.
```awk
BEGIN { print "Name  Score" }
```

### END
Runs once after all input has been processed.
```awk
END { print "Total lines:", NR }
```

### BEGINFILE / ENDFILE
Runs once at the start and end of each input file.
```awk
BEGINFILE { print "--- File:", FILENAME }
ENDFILE   { print "--- Done" }
```

### Range Patterns
Matches all lines from the first pattern to the second, inclusive.
```awk
/start/, /end/ { print }
NR == 3, NR == 7 { print }
```

---

## Fields and Records

Input is split into **records** (default: lines) and each record into **fields** (default: whitespace-separated).

| Variable | Meaning |
|----------|---------|
| `$0` | Entire current record |
| `$1` | First field |
| `$2` | Second field |
| `$n` | nth field |
| `$NF` | Last field |

```awk
# Input line: "Alice 92"
{ print $1 }     # → Alice
{ print $2 }     # → 92
{ print $NF }    # → 92
{ print $0 }     # → Alice 92
```

Dynamic field access:
```awk
{ i = 2; print $i }    # prints second field
```

---

## Built-in Variables

| Variable | Default | Meaning |
|----------|---------|---------|
| `FS` | `" "` | Input field separator |
| `OFS` | `" "` | Output field separator |
| `RS` | `"\n"` | Input record separator |
| `ORS` | `"\n"` | Output record separator |
| `NF` | — | Number of fields in current record |
| `NR` | — | Current record number (across all files) |
| `FNR` | — | Record number within current file |
| `FILENAME` | — | Name of current input file |
| `RSTART` | — | Start position set by `match()` |
| `RLENGTH` | — | Length set by `match()` |

```awk
{ print NR, $1 }          # line number + first field
BEGIN { FS = "," }        # comma-separated input
BEGIN { OFS = "\t" }      # tab-separated output
```

---

## Operators

### Arithmetic

| Operator | Meaning |
|----------|---------|
| `+` | Addition |
| `-` | Subtraction |
| `*` | Multiplication |
| `/` | Division |
| `%` | Modulo |
| `^` | Exponentiation |

### Comparison

| Operator | Meaning |
|----------|---------|
| `==` | Equal |
| `!=` | Not equal |
| `<` | Less than |
| `<=` | Less than or equal |
| `>` | Greater than |
| `>=` | Greater than or equal |

### Logical

| Operator | Meaning |
|----------|---------|
| `&&` | Logical AND |
| `\|\|` | Logical OR |
| `!` | Logical NOT |

### String / Regex

| Operator | Meaning |
|----------|---------|
| `~` | Matches regex |
| `!~` | Does not match regex |

String concatenation — place values next to each other:
```awk
{ print $1 " scored " $2 " marks" }
```

### Assignment

| Operator | Meaning |
|----------|---------|
| `=` | Assign |
| `+=` | Add and assign |
| `-=` | Subtract and assign |
| `*=` | Multiply and assign |
| `/=` | Divide and assign |
| `%=` | Modulo and assign |
| `++` | Increment |
| `--` | Decrement |

### Ternary

```awk
{ print ($2 >= 50) ? "pass" : "fail" }
```

---

## Statements

### print

Prints output separated by OFS (default space), followed by ORS (default newline).

```awk
{ print }                  # print entire line
{ print $1 }               # print first field
{ print $1, $2 }           # two fields separated by OFS
{ print $1 " " $2 }        # string concatenation
{ print "Name:", $1 }      # literal + field
```

### printf

Formatted output — like C's `printf`. Does **not** add a newline automatically.

```awk
{ printf "%s scored %d\n", $1, $2 }
{ printf "%-10s %5d\n", $1, $2 }
```

| Specifier | Meaning |
|-----------|---------|
| `%s` | String |
| `%d` | Integer |
| `%f` | Float |
| `%e` | Scientific notation |
| `%g` | Shorter of `%f` or `%e` |
| `%%` | Literal `%` |

### Output Redirection

```awk
{ print > "output.txt" }     # write to file (overwrite)
{ print >> "output.txt" }    # append to file
{ print | "sort" }           # pipe to command
```

---

## Control Flow

### if / else

```awk
{
    if ($2 >= 75) {
        print $1, "distinction"
    } else if ($2 >= 50) {
        print $1, "pass"
    } else {
        print $1, "fail"
    }
}
```

### while

```awk
{
    i = 1
    while (i <= NF) {
        print $i
        i++
    }
}
```

### for

```awk
{
    for (i = 1; i <= NF; i++) {
        print $i
    }
}
```

### for-in (arrays)

```awk
END {
    for (name in scores) {
        print name, scores[name]
    }
}
```

### next

Skip remaining rules for the current record and move to the next.

```awk
NR == 1 { next }    # skip header line
{ print }
```

### exit

Stop processing immediately. The END block still runs.

```awk
NR == 10 { exit }
END { print "stopped at line", NR }
```

### break / continue

Used inside loops.

```awk
{
    for (i = 1; i <= NF; i++) {
        if ($i == "") continue
        print $i
    }
}
```

---

## Arrays

AWK arrays are associative (key → value). Keys can be strings or numbers.

```awk
{ scores[$1] = $2 }
END {
    for (name in scores) {
        print name, scores[name]
    }
}
```

Counting occurrences:
```awk
{ count[$1]++ }
END {
    for (k in count) print k, count[k]
}
```

Delete an element:
```awk
delete arr[key]
```

Check if a key exists:
```awk
if (key in arr) { print "found" }
```

---

## Built-in Functions

### String Functions

| Function | Description |
|----------|-------------|
| `length(s)` | Length of `s`; length of `$0` if no argument |
| `substr(s, start)` | Substring from position `start` (1-based) |
| `substr(s, start, len)` | Substring of length `len` from `start` |
| `index(s, t)` | Position of `t` in `s`; 0 if not found |
| `split(s, arr, sep)` | Split `s` into array `arr` by `sep`; returns count |
| `sub(regex, repl, s)` | Replace first match of `regex` in `s` with `repl` |
| `gsub(regex, repl, s)` | Replace all matches of `regex` in `s` with `repl` |
| `match(s, regex)` | Position of `regex` in `s`; sets `RSTART`, `RLENGTH` |
| `tolower(s)` | Convert to lowercase |
| `toupper(s)` | Convert to uppercase |
| `sprintf(fmt, ...)` | Format string like `printf`, return as string |

```awk
{ print length($1) }
{ print toupper($1) }
{ print substr($1, 1, 3) }
{ gsub(/o/, "0"); print }
{ n = split($0, a, ","); print a[1] }
```

### Math Functions

| Function | Description |
|----------|-------------|
| `sin(x)` | Sine |
| `cos(x)` | Cosine |
| `atan2(y, x)` | Arctangent of y/x |
| `exp(x)` | e raised to x |
| `log(x)` | Natural logarithm |
| `sqrt(x)` | Square root |
| `int(x)` | Truncate to integer |
| `rand()` | Random float in [0, 1) |
| `srand(x)` | Seed the random number generator |

---

## User-Defined Functions

Functions are defined at the top level and can be called from any rule.

```awk
function max(a, b) {
    return (a > b) ? a : b
}

function greet(name,    msg) {
    msg = "Hello, " name
    return msg
}

{ print max($1, $2) }
{ print greet($1) }
```

- Parameters are local to the function.
- Extra parameters after a gap of spaces act as local variables.
- Functions can call other functions and themselves recursively.

---

## CLI Flags

### -F — Field Separator

```bash
awk_rust -F: '{print $1}' /etc/passwd
awk_rust -F, '{print $2}' data.csv
awk_rust -F'\t' '{print $1}' data.tsv
```

Can also be set inside the program:
```awk
BEGIN { FS = "," }
```

### -f — Program File

For longer programs, write them to a `.awk` file and use `-f`:

```bash
awk_rust -f program.awk data.txt
```

### -v — Pre-set Variable

```bash
awk_rust -v threshold=70 '$2 > threshold { print $1 }' data.txt
awk_rust -v OFS=, '{print $1, $2}' data.txt
```

Multiple `-v` flags are supported:
```bash
awk_rust -v a=1 -v b=2 'BEGIN { print a + b }'
```

---

## Worked Examples

Assume `data.txt` contains:
```
Alice 92
Bob 45
Carol 78
Dave 55
Eve 88
```

**Print every line:**
```bash
awk_rust '{print}' data.txt
```

**Print specific fields:**
```bash
awk_rust '{print $1}' data.txt
```

**Print with line numbers:**
```bash
awk_rust '{print NR, $0}' data.txt
```

**Filter by field value:**
```bash
awk_rust '$2 > 70 {print $1}' data.txt
# → Alice  Carol  Eve
```

**Sum a column:**
```bash
awk_rust '{sum += $2} END {print "Total:", sum}' data.txt
```

**Average a column:**
```bash
awk_rust '{sum += $2} END {print "Average:", sum/NR}' data.txt
```

**Count lines:**
```bash
awk_rust 'END {print NR}' data.txt
```

**Skip header line:**
```bash
awk_rust 'NR > 1 {print}' data.txt
```

**Grade report:**
```bash
awk_rust '{
    if ($2 >= 75) grade = "distinction"
    else if ($2 >= 50) grade = "pass"
    else grade = "fail"
    print $1, grade
}' data.txt
```

**CSV input:**
```bash
awk_rust -F, '{print $1, $3}' data.csv
```

**Add header and footer:**
```bash
awk_rust 'BEGIN{print "Name Score"} {print} END{print "---"}' data.txt
```

**Regex filter — names starting with A or E:**
```bash
awk_rust '$1 ~ /^[AE]/ {print}' data.txt
```

**Count occurrences of each value:**
```bash
awk_rust '{count[$1]++} END {for (k in count) print k, count[k]}' data.txt
```

**Formatted output:**
```bash
awk_rust '{printf "%-10s %3d\n", $1, $2}' data.txt
```

**Multiple rules:**
```bash
awk_rust '
BEGIN               { print "--- Report ---" }
NR == 1             { next }
$2 >= 75            { print $1, "distinction" }
$2 >= 50 && $2 < 75 { print $1, "pass" }
$2 < 50             { print $1, "fail" }
END                 { print "--- Done ---" }
' data.txt
```

**User-defined function:**
```bash
awk_rust '
function grade(score) {
    if (score >= 75) return "distinction"
    if (score >= 50) return "pass"
    return "fail"
}
{ print $1, grade($2) }
' data.txt
```
