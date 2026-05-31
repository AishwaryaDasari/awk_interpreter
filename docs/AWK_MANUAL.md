# AWK-Interpreter Manual

## NAME

awk — pattern-directed scanning and processing language

## SYNOPSIS

```
awk [-F fs] [-v var=value] 'program' [file ...]
awk [-F fs] [-v var=value] -f progfile [file ...]
```

## DESCRIPTION

AWK scans each input file for lines that match any of a set of patterns 
specified literally in the program or in one or more files specified with the 
`-f` option. With each pattern there can be an associated action that will be 
performed when a line of a file matches the pattern. Each line is matched 
against the pattern portion of every pattern-action statement; the associated 
action is performed for each matched pattern. The file name `-` means the 
standard input. Any file of the form `var=value` is treated as an assignment, 
not a filename, and is executed at the time it would have been opened if it were
a filename.

The input is normally split into lines; the split is done before the program is
run.A line is by default a string of characters ending with a newline character.
However, the record separator variable `RS` can be set to change this.

Normally, each line is split into fields, where the default field separator is 
one or more blanks and tabs. The fields are denoted `$1`, `$2`, ..., while `$0` 
refers to the entire line. The field separator can be set to any character or 
regular expression by using the `-F` command-line option or by setting the field 
separator variable `FS`.

## OPTIONS

**-F** *fs* 
Set the input field separator to the string *fs*.

**-f** *progfile* 
Read the AWK program source from the file *progfile*, instead of from the first 
command line argument.

**-v** *var=value* 
Assign the value *value* to the variable *var* before execution of the program 
begins. Such variable values are available to the `BEGIN` block.

## PROGRAM STRUCTURE

An AWK program is a sequence of pattern-action statements and optional function 
definitions.

```
pattern { action }
pattern { action }
...
```

A pattern-action statement has the form:

```
pattern { action }
```

Either the pattern or the action may be omitted. If the pattern is omitted, the 
action is performed for every input line. If the action is omitted, the default
action is to print all lines that match the pattern.

## PATTERNS

Patterns control the execution of actions. The patterns are:

```
BEGIN                    # executed before any input is read
END                      # executed after all input is read
BEGINFILE                # executed before each input file
ENDFILE                  # executed after each input file
expression               # evaluated as true or false
/regular expression/     # matches current input line
pattern1, pattern2       # range pattern (from pattern1 to pattern2)
```

A pattern can be:

- **BEGIN** — matches before the first line of the first input file is read
- **END** — matches after the last line of the last input file is processed
- **BEGINFILE** — matches before the first line of each input file is read
- **ENDFILE** — matches after the last line of each input file is processed
- **expression** — an expression that evaluates to true (non-zero or non-empty 
    string) or false (zero or empty string)
- **/regular expression/** — matches if the current input line contains the 
    regular expression
- **pattern1, pattern2** — a range pattern that matches all lines from a line 
     matching pattern1 to a line matching pattern2, inclusive

### Pattern Examples

```awk
BEGIN { print "Starting..." }
END { print "Done." }
/error/ { print "Error found:", $0 }
NR > 1 { print }
NR == 1, NR == 10 { print "Lines 1-10:", $0 }
$1 ~ /^[A-Z]/ { print "Starts with capital:", $1 }
```

## ACTIONS

An action is a sequence of statements. A statement can be:

```
if (expression) statement [else statement]
while (expression) statement
for (expression; expression; expression) statement
for (var in array) statement
do statement while (expression)
break
continue
{ [statement ...] }
expression
print [expression-list]
printf format [, expression-list]
return [expression]
next
exit [expression]
```

Statements are terminated by semicolons, newlines, or right braces.

## VARIABLES AND EXPRESSIONS

### Built-in Variables

| Variable | Description | Default |
|----------|-------------|---------|
| **FS** | Input field separator | `" "` (space) |
| **OFS** | Output field separator | `" "` (space) |
| **RS** | Input record separator | `"\n"` (newline) |
| **ORS** | Output record separator | `"\n"` (newline) |
| **NF** | Number of fields in current record | — |
| **NR** | Number of records read so far | — |
| **FNR** | Number of records read from current file | — |
| **FILENAME** | Name of current input file | — |
| **RSTART** | Start position of match from `match()` | — |
| **RLENGTH** | Length of match from `match()` | — |

### Fields

Fields are referenced by `$1`, `$2`, ..., `$NF`. The entire record is `$0`.
Fields can be assigned to,and the assignment changes `$0` and recalculates `NF`.

```awk
{ print $1, $3 }           # print first and third fields
{ print $NF }              # print last field
{ print $(NF-1) }          # print second-to-last field
{ $2 = "REDACTED"; print } # modify second field
```

### Operators

#### Arithmetic Operators

| Operator | Description |
|----------|-------------|
| `+` | Addition |
| `-` | Subtraction |
| `*` | Multiplication |
| `/` | Division |
| `%` | Modulo |
| `^` | Exponentiation |
| `-x` | Unary minus |
| `+x` | Unary plus |

#### Assignment Operators

| Operator | Description |
|----------|-------------|
| `=` | Simple assignment |
| `+=` | Add and assign |
| `-=` | Subtract and assign |
| `*=` | Multiply and assign |
| `/=` | Divide and assign |
| `%=` | Modulo and assign |
| `^=` | Exponentiate and assign |
| `++` | Increment (prefix or postfix) |
| `--` | Decrement (prefix or postfix) |

#### Relational Operators

| Operator | Description |
|----------|-------------|
| `<` | Less than |
| `<=` | Less than or equal |
| `>` | Greater than |
| `>=` | Greater than or equal |
| `==` | Equal |
| `!=` | Not equal |
| `~` | Matches regular expression |
| `!~` | Does not match regular expression |

#### Logical Operators

| Operator | Description |
|----------|-------------|
| `&&` | Logical AND |
| `\|\|` | Logical OR |
| `!` | Logical NOT |

#### String Operators

String concatenation is performed by writing expressions next to each other:

```awk
{ print $1 " " $2 }        # concatenate with space
{ s = "Hello" "World" }    # s becomes "HelloWorld"
```

#### Conditional Expression

```awk
expr1 ? expr2 : expr3
```

If *expr1* is true, the value is *expr2*, otherwise *expr3*.

### Precedence

Operators in order of increasing precedence:

1. Assignment: `= += -= *= /= %= ^=`
2. Conditional: `?:`
3. Logical OR: `||`
4. Logical AND: `&&`
5. Array membership: `in`
6. Matching: `~ !~`
7. Relational: `< <= > >= == !=`
8. Concatenation: (space)
9. Addition: `+ -`
10. Multiplication: `* / %`
11. Exponentiation: `^`
12. Unary: `! - +`
13. Increment/Decrement: `++ --`
14. Field: `$`
15. Grouping: `( )`

## CONTROL FLOW STATEMENTS

### if-else

```awk
if (expression)
    statement
[else
    statement]
```

Example:
```awk
{
    if ($2 >= 90)
        grade = "A"
    else if ($2 >= 80)
        grade = "B"
    else if ($2 >= 70)
        grade = "C"
    else
        grade = "F"
    print $1, grade
}
```

### while

```awk
while (expression)
    statement
```

Example:
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
for (expression; expression; expression)
    statement
```

Example:
```awk
{
    for (i = 1; i <= NF; i++)
        print $i
}
```

### for-in (Arrays)

```awk
for (var in array)
    statement
```

Example:
```awk
{
    count[$1]++
}
END {
    for (name in count)
        print name, count[name]
}
```

### do-while

```awk
do
    statement
while (expression)
```

### break

Exit from the innermost enclosing `while`, `for`, or `do` loop.

### continue

Start the next iteration of the innermost enclosing `while`, `for`, or `do` loop.

### next

Stop processing the current input record. Read the next input record and start
over with the first pattern in the AWK program.

### exit

Stop processing input and execute any `END` blocks. If called from an `END` 
block, exit immediately.

```awk
NR > 100 { exit }          # stop after 100 lines
END { print "Processed", NR, "lines" }
```

## ARRAYS

Arrays in AWK are associative, meaning they are indexed by strings. Array 
elements are created by assignment:

```awk
arr[key] = value
```

Array elements can be accessed:

```awk
x = arr[key]
```

Test if an array element exists:

```awk
if (key in arr)
    print "exists"
```

Delete an array element:

```awk
delete arr[key]
```

Delete an entire array:

```awk
delete arr
```

Iterate over array elements:

```awk
for (key in arr)
    print key, arr[key]
```

### Multi-dimensional Arrays

AWK simulates multi-dimensional arrays by concatenating indices with the 
`SUBSEP` character (default: `"\034"`):

```awk
matrix[1,2] = 10           # stored as matrix["1\0342"]
```

## OUTPUT STATEMENTS

### print

```awk
print [expression-list] [> file]
```

Prints the expressions separated by the output field separator (`OFS`), 
followed by the output record separator (`ORS`). If no expressions are given, 
prints `$0`.

Examples:
```awk
{ print }                  # print entire line
{ print $1, $3 }           # print fields 1 and 3
{ print "Total:", sum }    # print literal and variable
```

### printf

```awk
printf format [, expression-list] [> file]
```

Formatted output similar to C's `printf`. Does not append a newline automatically.

#### Format Specifiers

| Specifier | Description |
|-----------|-------------|
| `%c` | ASCII character |
| `%d`, `%i` | Decimal integer |
| `%e`, `%E` | Scientific notation |
| `%f` | Floating point |
| `%g`, `%G` | Shorter of `%e` or `%f` |
| `%o` | Unsigned octal |
| `%s` | String |
| `%x`, `%X` | Unsigned hexadecimal |
| `%%` | Literal `%` |

#### Format Modifiers

- **Width**: `%10s` — minimum field width of 10
- **Precision**: `%.2f` — 2 decimal places
- **Left-justify**: `%-10s` — left-justify in field
- **Zero-pad**: `%05d` — pad with zeros

Examples:
```awk
{ printf "%s\t%d\n", $1, $2 }
{ printf "%-10s %6.2f\n", $1, $2 }
{ printf "%04d-%02d-%02d\n", year, month, day }
```

## BUILT-IN FUNCTIONS

### String Functions

**length([s])**  
Returns the length of string *s*, or of `$0` if *s* is not supplied.

```awk
{ print length($1) }
```

**substr(s, start [, length])**  
Returns the substring of *s* starting at position *start* (1-indexed). If 
*length* is supplied, returns at most *length* characters.

```awk
{ print substr($1, 1, 3) }     # first 3 characters
```

**index(s, t)**  
Returns the position in *s* where string *t* first occurs, or 0 if it doesn't 
occur.

```awk
{ print index($0, "error") }
```

**split(s, a [, fs])**  
Splits string *s* into array *a* on field separator *fs* (or `FS` if *fs* is not 
supplied). Returns the number of fields.

```awk
{ n = split($0, arr, ":"); print n }
```

**sub(regex, replacement [, target])**  
Substitutes the first occurrence of *regex* in *target* (or `$0` if *target* is
not supplied) with *replacement*. Returns the number of substitutions made 
(0 or 1).

```awk
{ sub(/old/, "new"); print }
{ sub(/old/, "new", $1); print }
```

**gsub(regex, replacement [, target])**  
Globally substitutes all occurrences of *regex* in *target* (or `$0` if *target*
is not supplied) with *replacement*. Returns the number of substitutions made.

```awk
{ gsub(/old/, "new"); print }
```

**match(s, regex)**  
Returns the position in *s* where *regex* occurs, or 0 if it doesn't. Sets 
`RSTART` to the start position and `RLENGTH` to the length of the matched string.

```awk
{ if (match($0, /[0-9]+/)) print "Found number at", RSTART }
```

**tolower(s)**  
Returns *s* with all uppercase characters converted to lowercase.

```awk
{ print tolower($1) }
```

**toupper(s)**  
Returns *s* with all lowercase characters converted to uppercase.

```awk
{ print toupper($1) }
```

**sprintf(format, expression, ...)**  
Returns a string formatted according to *format* (like `printf` but returns the
string instead of printing it).

```awk
{ s = sprintf("%s: %d", $1, $2); print s }
```

### Numeric Functions

**int(x)**  
Returns *x* truncated to an integer.

```awk
{ print int(3.7) }             # prints 3
```

**sqrt(x)**  
Returns the square root of *x*.

```awk
{ print sqrt($1) }
```

**exp(x)**  
Returns *e* raised to the power *x*.

```awk
{ print exp($1) }
```

**log(x)**  
Returns the natural logarithm of *x*.

```awk
{ print log($1) }
```

**sin(x)**  
Returns the sine of *x* (in radians).

```awk
{ print sin($1) }
```

**cos(x)**  
Returns the cosine of *x* (in radians).

```awk
{ print cos($1) }
```

**atan2(y, x)**  
Returns the arctangent of *y/x* in radians.

```awk
{ print atan2($1, $2) }
```

**rand()**  
Returns a random number between 0 and 1.

```awk
{ print rand() }
```

**srand([x])**  
Sets the seed for `rand()` to *x*, or uses the time of day if *x* is not 
supplied. Returns the previous seed.

```awk
BEGIN { srand() }
{ print int(rand() * 100) }    # random number 0-99
```

## REGULAR EXPRESSIONS

Regular expressions are enclosed in slashes: `/regex/`

### Basic Patterns

| Pattern | Matches |
|---------|---------|
| `.` | Any single character |
| `^` | Beginning of line |
| `$` | End of line |
| `[abc]` | Any character in the set |
| `[^abc]` | Any character not in the set |
| `[a-z]` | Any character in the range |
| `\` | Escape special character |

### Repetition

| Pattern | Matches |
|---------|---------|
| `*` | Zero or more occurrences |
| `+` | One or more occurrences |
| `?` | Zero or one occurrence |
| `{n}` | Exactly *n* occurrences |
| `{n,}` | At least *n* occurrences |
| `{n,m}` | Between *n* and *m* occurrences |

### Grouping

| Pattern | Matches |
|---------|---------|
| `(expr)` | Grouping |
| `expr1\|expr2` | Alternation (expr1 or expr2) |

### Examples

```awk
/^#/                           # lines starting with #
/error$/                       # lines ending with "error"
/[0-9]+/                       # lines containing digits
/^[A-Z][a-z]+/                 # capitalized words at start
$1 ~ /^[aeiou]/                # field 1 starts with vowel
$0 !~ /test/                   # lines not containing "test"
```

## EXAMPLES

### Print specific columns

```bash
awk '{print $1, $3}' file.txt
```

### Print lines matching a pattern

```bash
awk '/error/ {print}' logfile.txt
```

### Sum a column

```bash
awk '{sum += $2} END {print sum}' data.txt
```

### Calculate average

```bash
awk '{sum += $2} END {print sum/NR}' data.txt
```

### Count occurrences

```bash
awk '{count[$1]++} END {for (k in count) print k, count[k]}' data.txt
```

### Print line numbers

```bash
awk '{print NR, $0}' file.txt
```

### Skip header line

```bash
awk 'NR > 1 {print}' data.txt
```

### Filter by field value

```bash
awk '$3 > 100 {print $1, $3}' data.txt
```

### CSV processing

```bash
awk -F, '{print $1, $3}' data.csv
```

### Multiple patterns

```bash
awk 'BEGIN {print "Report"} /error/ {count++} END {print count, "errors"}' log.txt
```

### Field manipulation

```bash
awk '{$2 = $2 * 1.1; print}' prices.txt
```

### Conditional output

```bash
awk '{if ($2 >= 90) print $1, "A"; else print $1, "B"}' grades.txt
```

### Range patterns

```bash
awk '/START/,/END/ {print}' file.txt
```

### Format output

```bash
awk '{printf "%-10s %6.2f\n", $1, $2}' data.txt
```

### Multiple input files

```bash
awk '{print FILENAME, NR, FNR, $0}' file1.txt file2.txt
```

### Set variables from command line

```bash
awk -v threshold=100 '$2 > threshold {print}' data.txt
```

### Complex calculations

```bash
awk '{
    total += $2
    count++
}
END {
    avg = total / count
    print "Total:", total
    print "Count:", count
    print "Average:", avg
}' data.txt
```

## INITIALIZATION AND CLEANUP

### BEGIN Block

The `BEGIN` block is executed once before any input is read. It's useful for:
- Printing headers
- Initializing variables
- Setting field separators

```awk
BEGIN {
    FS = ":"
    OFS = "\t"
    print "Name", "ID", "Home"
}
```

### END Block

The `END` block is executed once after all input has been processed. It's useful for:
- Printing summaries
- Final calculations
- Cleanup

```awk
END {
    print "Total lines:", NR
    print "Total sum:", sum
}
```

### BEGINFILE and ENDFILE Blocks

`BEGINFILE` is executed before each input file is read.  
`ENDFILE` is executed after each input file is processed.

```awk
BEGINFILE {
    print "Processing:", FILENAME
}
ENDFILE {
    print "Finished:", FILENAME, "with", FNR, "lines"
}
```

## NOTES

### Type Conversion

AWK automatically converts between strings and numbers as needed:

- In numeric context, strings are converted to numbers (non-numeric strings become 0)
- In string context, numbers are converted to strings
- Uninitialized variables have the value 0 and the string value ""

```awk
x = "3" + "4"              # x = 7 (numeric)
x = "3" "4"                # x = "34" (string concatenation)
```

### Comparison Rules

- If both operands are numeric, numeric comparison is used
- If one operand is numeric and the other is a string that looks like a number, 
numeric comparison is used
- Otherwise, string comparison is used

### Field Assignment

Assigning to a field that doesn't exist extends the record:

```awk
{ $10 = "new"; print }     # creates fields $1 through $10 if needed
```

Assigning to `$0` re-splits the record into fields.

### Whitespace

AWK is free-form. Statements can span multiple lines. Newlines can be escaped 
with backslash.

```awk
{
    print \
        $1, \
        $2
}
```

## EXIT STATUS

The exit status is 0 if no errors occurred, and non-zero if errors occurred.

## SEE ALSO

sed(1), grep(1), regex(7)


## AUTHORS

Built as an educational project implementing AWK from scratch in Rust, Go, and Crystal.

## BUGS

Report bugs to the project repository.

---


