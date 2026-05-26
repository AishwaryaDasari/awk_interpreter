# AWK Interpreter — Feature Specs

## Project Overview

A working AWK interpreter built from scratch that runs real AWK programs from the terminal, the same way the system `awk` command does. The interpreter is implemented in **Rust** first, then ported to **Go** and **Crystal**. All three versions run the same programs and produce identical output.

The pipeline is: **Lexer → Parser → AST → Evaluator**. Input is read from files or stdin, broken into records, matched against patterns, and actions are executed for each match.

## Goal

Build a complete AWK interpreter that can run real AWK programs from the terminal — supporting all core language features including patterns, variables, loops, arrays, and built-in functions. The interpreter must produce output identical to the system `awk` command for the same input. Once complete in Rust, the same interpreter is ported to Go and Crystal using each language's own idioms.

---

## Features

1. **print** — `{print}`, `{print $1, $2}` ✅ Done
2. **Fields** — `$0`, `$n`, `$NF`, `NF`, `NR` ✅ Done
3. **BEGIN / END + regex patterns** — `BEGIN{}`, `END{}`, `/pattern/{}`
4. **Variables and arithmetic** — `total += $2`, `x = $1 * 2`, operators `+ - * / %`
5. **Field separator** — `-F` flag and `FS` variable
6. **if / else** — conditionals with comparisons
7. **Loops** — `for`, `while`, `break`, `continue`, `next`, `exit`
8. **Built-in string functions** — `length`, `substr`, `split`, `gsub`, `sub`, `match`, `tolower`, `toupper`, `sprintf`
9. **Arrays** — associative arrays, `for (k in arr)`, `delete`
10. **printf** — formatted output with `%s`, `%d`, `%f`, etc.

---

## Architecture

```
input ──► Lexer ──► tokens ──► Parser ──► AST ──► Evaluator ──► output
```

| Module | Role |
|--------|------|
| Lexer | Reads source text and produces a stream of tokens |
| Parser | Consumes tokens and builds an AST |
| AST | Tree of nodes representing the program structure |
| Evaluator | Walks the AST and executes each node |
| Value | Dual-type (string + number) with automatic coercion |
| Builtins | Built-in functions and runtime variables |

---

## Pseudocode (language-agnostic)

### 1. print

Outputs one or more values to stdout, separated by OFS, with ORS at the end.

```
PROCEDURE print(args):
    IF args is empty THEN
        OUTPUT current_line
    ELSE
        FOR each arg in args DO
            values.add(evaluate(arg))
        END FOR
        OUTPUT join(values, OFS) + ORS
    END IF
END PROCEDURE
```

### 2. Fields

Splits each input line into numbered fields (`$1`, `$2`, ...) and tracks `NF` and `NR`.

```
PROCEDURE set_line(line):
    current_line = line
    fields = split(line, FS)
    NF = length(fields)
    NR = NR + 1
END PROCEDURE

PROCEDURE get_field(n):
    IF n == 0 THEN return current_line
    IF n >= 1 AND n <= NF THEN return fields[n]
    ELSE return ""
    END IF
END PROCEDURE
```

### 3. BEGIN / END + regex patterns

Runs BEGIN rules before input, END rules after, and matches each line against regex or expression patterns.

```
PROCEDURE run_program(program, input_lines):
    FOR each rule WHERE pattern is BEGIN DO
        execute(rule.action)
    END FOR

    FOR each line in input_lines DO
        set_line(line)
        FOR each rule in program DO
            IF matches(rule.pattern) THEN
                execute(rule.action)
            END IF
        END FOR
    END FOR

    FOR each rule WHERE pattern is END DO
        execute(rule.action)
    END FOR
END PROCEDURE

PROCEDURE matches(pattern):
    IF pattern is ALWAYS THEN return true
    IF pattern is REGEX THEN return current_line contains pattern
    IF pattern is EXPR THEN return evaluate(pattern) is truthy
END PROCEDURE
```

### 4. Variables and arithmetic

Evaluates arithmetic expressions and stores results in named variables.

```
PROCEDURE evaluate_binop(op, left, right):
    l = to_number(evaluate(left))
    r = to_number(evaluate(right))
    IF op is + THEN return l + r
    IF op is - THEN return l - r
    IF op is * THEN return l * r
    IF op is / THEN return l / r
    IF op is % THEN return l % r
END PROCEDURE

PROCEDURE execute_assign(name, value):
    vars[name] = evaluate(value)
END PROCEDURE

PROCEDURE execute_assign_op(op, name, value):
    vars[name] = evaluate_binop(op, vars[name], value)
END PROCEDURE
```

### 5. Field separator

Controls how each input line is split into fields, via `-F` flag or the `FS` variable.

```
PROCEDURE set_fs(sep):
    FS = sep
END PROCEDURE

PROCEDURE set_line(line):
    IF FS == " " THEN
        fields = split_on_whitespace(line)
    ELSE
        fields = split(line, FS)
    END IF
    NF = length(fields)
END PROCEDURE
```

### 6. if / else

Conditionally executes a block of statements based on an expression.

```
PROCEDURE execute_if(cond, then_block, else_block):
    IF is_truthy(evaluate(cond)) THEN
        execute_stmts(then_block)
    ELSE IF else_block is not empty THEN
        execute_stmts(else_block)
    END IF
END PROCEDURE
```

### 7. Loops

Repeats a block of statements with support for `break`, `continue`, `next`, and `exit`.

```
PROCEDURE execute_while(cond, body):
    WHILE is_truthy(evaluate(cond)) DO
        signal = execute_stmts(body)
        IF signal is BREAK THEN stop loop
        IF signal is CONTINUE THEN go to condition check
        IF signal is NEXT OR EXIT THEN bubble up
    END WHILE
END PROCEDURE

PROCEDURE execute_for(init, cond, update, body):
    execute(init)
    WHILE is_truthy(evaluate(cond)) DO
        signal = execute_stmts(body)
        IF signal is BREAK THEN stop loop
        IF signal is CONTINUE THEN execute(update) and go to condition check
        execute(update)
    END WHILE
END PROCEDURE
```

### 8. Built-in string functions

Provides common string operations dispatched by function name.

```
PROCEDURE call_builtin(name, args):
    IF name is "length"  THEN return length of args[0]
    IF name is "substr"  THEN return args[0] from position args[1] for args[2] chars
    IF name is "toupper" THEN return args[0] in uppercase
    IF name is "tolower" THEN return args[0] in lowercase
    IF name is "index"   THEN return position of args[1] in args[0], or 0
    IF name is "split"   THEN split args[0] by args[2] into array args[1], return count
    IF name is "gsub"    THEN replace all matches of args[0] in args[2] with args[1], return count
    IF name is "sub"     THEN replace first match of args[0] in args[2] with args[1]
    IF name is "match"   THEN find args[1] in args[0], set RSTART and RLENGTH, return position
    IF name is "sprintf" THEN return format(args[0], rest of args) as string
END PROCEDURE
```

### 9. Arrays

Associative arrays with string keys, supporting iteration and deletion.

```
PROCEDURE array_get(name, index):
    key = to_string(index)
    IF arrays[name][key] exists THEN return arrays[name][key]
    ELSE return ""
    END IF
END PROCEDURE

PROCEDURE array_set(name, index, value):
    key = to_string(index)
    arrays[name][key] = value
END PROCEDURE

PROCEDURE execute_for_in(var, array_name, body):
    FOR each key in arrays[array_name] DO
        vars[var] = key
        execute_stmts(body)
    END FOR
END PROCEDURE

PROCEDURE execute_delete(name, index):
    IF index is given THEN remove arrays[name][index]
    ELSE remove arrays[name]
    END IF
END PROCEDURE
```

### 10. printf

Formatted output using a format string with specifiers — no automatic newline.

```
PROCEDURE execute_printf(fmt, args):
    result = ""
    FOR each character in fmt DO
        IF character is % THEN
            read the specifier (%s %d %f %e %g %%)
            take the next argument
            format the argument according to the specifier
            apply width and padding if specified
            append to result
        ELSE
            append character to result
        END IF
    END FOR
    OUTPUT result  // no newline added
END PROCEDURE
```

---

## Implementation Status

| # | Feature | Rust | Go | Crystal |
|---|---------|------|----|---------|
| 1 | print | ✅ Done | ✅ Done | ✅ Done |
| 2 | Fields | ✅ Done | ✅ Done | ✅ Done |
| 3 | BEGIN / END + regex patterns | ✅ Done | 🔲 Planned | ✅ Done |
| 4 | Variables and arithmetic | ✅ Done | 🔲 Planned | ✅ Done |
| 5 | Field separator | ✅ Done | ✅ Done | ✅ Done |
| 6 | if / else | 🔲 Planned | 🔲 Planned | 🔲 Planned |
| 7 | Loops | 🔲 Planned | 🔲 Planned | 🔲 Planned |
| 8 | Built-in string functions | 🔲 Planned | 🔲 Planned | 🔲 Planned |
| 9 | Arrays | 🔲 Planned | 🔲 Planned | 🔲 Planned |
| 10 | printf | 🔲 Planned | 🔲 Planned | 🔲 Planned |
