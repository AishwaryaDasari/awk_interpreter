# AWK Interpreter — User Manual

This is the reference manual for every command and feature supported by this AWK interpreter.
Use this to look up syntax, behavior, and examples for anything you want to run.

---

## How to Run

```
awk_interpreter 'program' [file...]
```

- `program` — the AWK program, in single quotes
- `file` — one or more input files (reads from stdin if none given)

**Examples:**
```
awk_interpreter '{print}' data.txt
awk_interpreter '{print $1, $2}' data.txt
echo "hello world" | awk_interpreter '{print $2}'
```

---

## Program Structure

An AWK program is a sequence of `pattern { action }` rules.

```
pattern { action }
pattern { action }
...
```

- **pattern** — a condition that decides whether the action runs for a given line
- **action** — one or more statements to execute when the pattern matches
- If pattern is omitted, the action runs on every line
- If action is omitted, the matching line is printed

```
{ print }              # no pattern — runs on every line
/Sam/ { print }        # regex pattern — runs only on lines matching "Sam"
NR > 1 { print }       # expression pattern — runs when NR is greater than 1
BEGIN { print "start" }
END   { print "done" }
```

---

## Special Patterns

### BEGIN
Runs once before any input is read.
```
BEGIN { print "Name  Marks" }
```

### END
Runs once after all input has been processed.
```
END { print "Total lines:", NR }
```

---

## Fields

Each input line is split into fields by the field separator (default: whitespace).

| Variable | Meaning |
|----------|---------|
| `$0` | The entire current line |
| `$1` | First field |
| `$2` | Second field |
| `$n` | nth field |
| `$NF` | Last field (NF = number of fields) |

```
# Input line: "Sam 67"
{ print $1 }     # → Sam
{ print $2 }     # → 67
{ print $0 }     # → Sam 67
{ print $NF }    # → 67  (last field)
```

---

## Built-in Variables

| Variable | Default | Meaning |
|----------|---------|---------|
| `FS` | `" "` | Field separator (input) |
| `OFS` | `" "` | Output field separator |
| `RS` | `"\n"` | Record separator |
| `ORS` | `"\n"` | Output record separator |
| `NF` | — | Number of fields in current record |
| `NR` | — | Current record (line) number |
| `FNR` | — | Record number within current file |
| `FILENAME` | — | Name of current input file |

```
{ print NR, $1 }          # line number + first field
{ print NF }              # number of fields on this line
BEGIN { FS = ":" }        # set colon as field separator
```

---

## print

Prints output. Fields are separated by OFS (default space). Each print adds ORS (default newline) at the end.

```
{ print }                  # print entire line
{ print $1 }               # print first field
{ print $1, $2 }           # print two fields separated by space
{ print $1 " " $2 }        # string concatenation
{ print "Name:", $1 }      # literal string + field
```

---

## printf

Formatted output — like C's printf. Does NOT add a newline automatically.

```
{ printf "%s scored %d\n", $1, $2 }
{ printf "%-10s %5d\n", $1, $2 }
```

**Format specifiers:**

| Specifier | Meaning |
|-----------|---------|
| `%s` | String |
| `%d` | Integer |
| `%f` | Float |
| `%e` | Scientific notation |
| `%g` | Shorter of %f or %e |
| `%%` | Literal `%` |

---

## Arithmetic Operators

| Operator | Meaning |
|----------|---------|
| `+` | Addition |
| `-` | Subtraction |
| `*` | Multiplication |
| `/` | Division |
| `%` | Modulo |
| `^` | Exponentiation |

```
{ print $2 + 10 }
{ print $2 * 2 }
{ total += $2 }
END { print "Total:", total }
```

---

## Comparison Operators

| Operator | Meaning |
|----------|---------|
| `==` | Equal |
| `!=` | Not equal |
| `<` | Less than |
| `<=` | Less than or equal |
| `>` | Greater than |
| `>=` | Greater than or equal |

```
$2 > 70 { print $1, "passed" }
$1 == "Sam" { print "Found Sam" }
```

---

## String Operators

| Operator | Meaning |
|----------|---------|
| `~` | Matches regex |
| `!~` | Does not match regex |

```
$1 ~ /^S/ { print $1 }       # names starting with S
$1 !~ /am/ { print $1 }      # names not containing "am"
```

String concatenation is done by placing values next to each other:
```
{ print $1 " got " $2 " marks" }
```

---

## Assignment Operators

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

```
{ count++ }
END { print count }

{ total += $2 }
END { print total / NR }
```

---

## Regex Patterns

```
/pattern/ { action }        # matches lines containing pattern
/^Sam/    { print }         # lines starting with Sam
/[0-9]+/  { print $2 }      # lines containing a number
```

---

## if / else

```
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

---

## Loops

### while
```
{
    i = 1
    while (i <= NF) {
        print $i
        i++
    }
}
```

### for
```
{
    for (i = 1; i <= NF; i++) {
        print $i
    }
}
```

### for-in (arrays)
```
END {
    for (name in scores) {
        print name, scores[name]
    }
}
```

---

## Arrays

AWK arrays are associative (key-value). Keys can be strings or numbers.

```
{ scores[$1] = $2 }
END {
    for (name in scores) {
        print name, scores[name]
    }
}
```

```
{ count[$1]++ }
END {
    for (k in count) print k, count[k]
}
```

Delete an element:
```
delete arr[key]
```

---

## Built-in String Functions

| Function | Description |
|----------|-------------|
| `length(s)` | Length of string s (or `$0` if no arg) |
| `substr(s, start)` | Substring from position start |
| `substr(s, start, len)` | Substring of length len from start |
| `index(s, t)` | Position of t in s (0 if not found) |
| `split(s, arr, sep)` | Split s into array arr by sep, returns count |
| `sub(regex, repl, s)` | Replace first match of regex in s with repl |
| `gsub(regex, repl, s)` | Replace all matches of regex in s with repl |
| `match(s, regex)` | Returns position of regex in s (sets RSTART, RLENGTH) |
| `tolower(s)` | Convert to lowercase |
| `toupper(s)` | Convert to uppercase |
| `sprintf(fmt, ...)` | Format string like printf, return as string |

```
{ print length($1) }
{ print toupper($1) }
{ print substr($1, 1, 3) }
{ gsub(/Sam/, "Samuel"); print }
```

---

## Built-in Math Functions

| Function | Description |
|----------|-------------|
| `sin(x)` | Sine |
| `cos(x)` | Cosine |
| `atan2(y, x)` | Arctangent of y/x |
| `exp(x)` | e raised to x |
| `log(x)` | Natural log |
| `sqrt(x)` | Square root |
| `int(x)` | Truncate to integer |
| `rand()` | Random number between 0 and 1 |
| `srand(x)` | Seed random number generator |

---

## Field Separator (-F flag)

```
awk_interpreter -F: '{print $1}' /etc/passwd
awk_interpreter -F, '{print $2}' data.csv
```

Or set it inside the program:
```
BEGIN { FS = "," }
{ print $1, $2 }
```

---

## Program from File (-f flag)

For longer programs, write them to a file and use `-f`:

```
awk_interpreter -f program.awk data.txt
```

---

## Pre-set Variables (-v flag)

```
awk_interpreter -v threshold=70 '$2 > threshold { print $1 }' data.txt
```

---

## Multiple Rules

```
BEGIN         { print "--- Report ---" }
NR == 1       { next }              # skip header line
$2 >= 75      { print $1, "distinction" }
$2 >= 50 && $2 < 75 { print $1, "pass" }
$2 < 50       { print $1, "fail" }
END           { print "--- Done ---" }
```

---

## next

Skip the rest of the current rules and move to the next input line.

```
NR == 1 { next }       # skip header
{ print }
```

---

## exit

Stop processing immediately.

```
NR == 10 { exit }
END { print "stopped at line", NR }
```

---

## Pipe and Redirection (planned)

```
{ print | "sort" }
{ print > "output.txt" }
{ print >> "output.txt" }
```

---

## Quick Reference Card

```
# Print whole file
awk_interpreter '{print}' file.txt

# Print specific columns
awk_interpreter '{print $1, $3}' file.txt

# Print line numbers
awk_interpreter '{print NR, $0}' file.txt

# Skip header line
awk_interpreter 'NR > 1 {print}' file.txt

# Filter by pattern
awk_interpreter '/Sam/ {print}' file.txt

# Filter by field value
awk_interpreter '$2 > 70 {print $1}' file.txt

# Count lines
awk_interpreter 'END {print NR}' file.txt

# Sum a column
awk_interpreter '{sum += $2} END {print sum}' file.txt

# Average a column
awk_interpreter '{sum += $2} END {print sum/NR}' file.txt

# CSV input
awk_interpreter -F, '{print $1}' file.csv

# Add header and footer
awk_interpreter 'BEGIN{print "Name Marks"} {print} END{print "---"}' file.txt
```
