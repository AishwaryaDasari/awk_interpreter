# Crystal AWK Interpreter — Terminal Commands

## Build

```sh
crystal build src/awk.cr -o awk_crystal
```

## Run

```sh
# Inline program
./awk_crystal 'BEGIN{print "hello"}'

# With input file
./awk_crystal '{print $1, $3}' data.txt

# Piped input
echo "hello world" | ./awk_crystal '{print $2}'

# Field separator
./awk_crystal -F, '{print $2}' data.csv
echo "a,b,c" | ./awk_crystal -F, '{print $2}'

# Multiple files
./awk_crystal '{print FILENAME, $0}' file1.txt file2.txt

# Program from file
./awk_crystal -f program.awk data.txt
```

## Variables

```sh
# Assignment
./awk_crystal 'BEGIN{x = 42; print x}'

# Compound assignment
./awk_crystal 'BEGIN{x = 10; x += 5; print x}'       # 15
./awk_crystal 'BEGIN{x = 10; x *= 2; print x}'       # 20

# Increment / decrement
./awk_crystal 'BEGIN{x = 1; print ++x}'              # 2  (pre)
./awk_crystal 'BEGIN{x = 1; print x++; print x}'     # 1, 2 (post)

# Field variables
./awk_crystal '{x = $2; print x}' <<< "a b c"        # b
./awk_crystal '{$0 = "hello"; print}'                 # hello
```

## Arithmetic

```sh
./awk_crystal 'BEGIN{print 5 + 3 * 2}'               # 11
./awk_crystal 'BEGIN{print 2 ^ 3}'                   # 8
./awk_crystal 'BEGIN{print 10 % 3}'                  # 1
./awk_crystal 'BEGIN{print -5}'                      # -5
./awk_crystal 'BEGIN{print "a" "b" 3}'              # ab3
```

## Comparisons

```sh
./awk_crystal 'BEGIN{print 1 == 1}'                  # 1
./awk_crystal 'BEGIN{print "a" < "b"}'               # 1
./awk_crystal 'BEGIN{print 1 && 0}'                  # 0
./awk_crystal 'BEGIN{print 0 || 1}'                  # 1
./awk_crystal 'BEGIN{print !0}'                      # 1
./awk_crystal 'BEGIN{print 3 <= 2}'                  # 0
```

## Control Flow

```sh
# if / else
./awk_crystal 'BEGIN{if (1) print "y"; else print "n"}'
./awk_crystal 'BEGIN{x = 2; if (x == 1) print "a"; else if (x == 2) print "b"}'

# while
./awk_crystal 'BEGIN{i = 3; while (i > 0) {print i; i--}}'

# for
./awk_crystal 'BEGIN{for (i = 1; i <= 3; i++) print i}'

# break / continue
./awk_crystal 'BEGIN{for (i = 1; i <= 5; i++) {if (i == 3) continue; if (i > 4) break; print i}}'

# next (skip to next record)
printf "a\nb\nc\n" | ./awk_crystal '{if (NR == 2) next; print}'

# exit
./awk_crystal 'BEGIN{exit 42}'; echo $?              # 42
```

## Arrays

```sh
# Associative set / get
./awk_crystal 'BEGIN{a["x"] = 10; a["y"] = 20; print a["x"]}'

# Delete element
./awk_crystal 'BEGIN{a["x"] = 10; delete a["x"]; print a["x"]}'

# Delete whole array
./awk_crystal 'BEGIN{a["x"] = 1; delete a; print a["x"]}'

# for-in loop
./awk_crystal 'BEGIN{a[1] = "x"; a[2] = "y"; for (i in a) print i, a[i]}'

# split into array
./awk_crystal 'BEGIN{n = split("a,b,c", arr, ","); for (i in arr) print i, arr[i]}'
```

## Built-in Functions

```sh
# String
./awk_crystal 'BEGIN{print length("hello")}'          # 5
./awk_crystal 'BEGIN{print substr("hello", 2, 3)}'    # ell
./awk_crystal 'BEGIN{print index("hello", "ll")}'     # 3
./awk_crystal 'BEGIN{print toupper("abc")}'           # ABC
./awk_crystal 'BEGIN{print tolower("ABC")}'           # abc

# Match
./awk_crystal 'BEGIN{print match("hello", "ll"); print RSTART, RLENGTH}'

# Substitution
./awk_crystal 'BEGIN{s = "hello"; sub(/l/, "x", s); print s}'    # hexlo
./awk_crystal 'BEGIN{s = "hello"; gsub(/l/, "x", s); print s}'   # hexxo
./awk_crystal '{$0 = "hello"; gsub(/l/, "x"); print}'            # hexxo

# Math
./awk_crystal 'BEGIN{print int(3.7)}'                 # 3
./awk_crystal 'BEGIN{print sqrt(9)}'                  # 3
./awk_crystal 'BEGIN{print sin(0)}'                   # 0
./awk_crystal 'BEGIN{print cos(0)}'                   # 1
./awk_crystal 'BEGIN{print exp(1)}'                   # 2.71828
./awk_crystal 'BEGIN{print log(1)}'                   # 0
./awk_crystal 'BEGIN{print rand()}'                   # random 0-1

# Formatting
./awk_crystal 'BEGIN{s = sprintf("%d-%s", 42, "abc"); print s}'
```

## printf

```sh
./awk_crystal 'BEGIN{printf "%d %s\n", 42, "hello"}'
./awk_crystal 'BEGIN{printf "|%10s| %-10s|\n", "right", "left"}'
```

## Patterns

```sh
# Regex pattern
./awk_crystal '/hello/{print $2}' <<< "hello world"

# Range pattern
printf "a\nb\nc\nd\n" | ./awk_crystal '/a/,/c/{print NR, $0}'

# Expression pattern
./awk_crystal 'NR == 2 {print}' <<< "$(printf "a\nb\nc")"

# BEGIN / END
./awk_crystal 'BEGIN{print "start"} {print $1} END{print "end"}' data.txt
```

## Ternary Operator

```sh
./awk_crystal 'BEGIN{print 1 == 1 ? "yes" : "no"}'
```

## Field Separator

```sh
# Default (whitespace)
echo "a b c" | ./awk_crystal '{print $2}'             # b

# Comma separator
echo "a,b,c" | ./awk_crystal -F, '{print $2}'         # b

# In-program FS override
./awk_crystal 'BEGIN{FS = ","} {print $2}' <<< "a,b,c"
```

## Special Variables

```sh
./awk_crystal 'BEGIN{print FS}'                       # current field sep
./awk_crystal '{print NR, FNR, $0}' data.txt
./awk_crystal '{print NF}'                             # number of fields
./awk_crystal 'BEGIN{OFS = ","; print 1, 2, 3}'       # 1,2,3
./awk_crystal 'BEGIN{ORS = "\r\n"; print "hi"}'       # CRLF line ending
```
