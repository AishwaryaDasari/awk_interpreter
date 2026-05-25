# AWK Interpreter — Implementation Guide

Internal dev guide. Tracks what to build, which files to touch, and what
"done" looks like. Follows the same order as `features.txt`.

---

## Current State

What works:
- `{print}` — prints the whole line
- `{print $1, $2}` — specific fields
- `{print NR, $1}` — NR and NF variables
- `{print $NF}` — dynamic field access
- Whitespace field splitting, reads from file or stdin

What is missing (everything below):

---

## Architecture

```
CLI (main.rs)
  └─► Lexer (lexer.rs)      — source text → token stream
        └─► Parser (parser.rs)  — tokens → AST
              └─► AST (ast.rs)      — node definitions
                    └─► Interp (interp.rs) — walks AST, executes
                          └─► Env (builtins.rs) — runtime state
                                └─► Value (value.rs) — str/num dual type
```

---

## Feature 1 — print ✅ Done

`{print}`, `{print $1}`, `{print $1, $2}` all work.

---

## Feature 2 — Fields ✅ Done

`$0`, `$1`, `$n`, `$NF`, `NF`, `NR` all work.

---

## Feature 3 — BEGIN / END + Regex Patterns

**What it enables:**
```
BEGIN { print "header" }
END   { print "done" }
/Sam/ { print }
NR > 1 { print }
```

**Lexer** (`lexer.rs`) — add tokens:
- `Token::Begin`, `Token::End` — keywords
- `Token::Regex(String)` — `/pattern/` literal

**AST** (`ast.rs`) — restructure the top level:
```rust
pub struct Program { pub rules: Vec<Rule> }

pub struct Rule {
    pub pattern: Pattern,
    pub action: Vec<Stmt>,
}

pub enum Pattern {
    Always,           // no pattern
    Begin,
    End,
    Regex(String),    // /pat/
    Expr(Expr),       // NR > 1, $2 == "Sam", etc.
}
```

**Parser** (`parser.rs`) — top-level loop now reads `pattern { stmts }` pairs instead of a single block.

**Interpreter** (`interp.rs`) — `exec` takes a `Program`:
- Run `BEGIN` rules once before the line loop
- For each line: check each rule's pattern, run action if it matches
- Run `END` rules once after the line loop
- Regex match: check `$0` against the pattern string using Rust's `contains` or the `regex` crate

**Files:** `lexer.rs`, `ast.rs`, `parser.rs`, `interp.rs`, `main.rs`

**Done when:**
```
cargo run -- 'BEGIN{print "Name Marks"} {print} END{print "---"}' tests/test1.txt
cargo run -- '/Sam/{print}' tests/test1.txt
cargo run -- 'NR>1{print}' tests/test1.txt
```

---

## Feature 4 — Variables and Arithmetic

**What it enables:**
```
{ total += $2 }
END { print total / NR }
{ print $2 * 2 }
{ x = $1; print x }
```

**Lexer** (`lexer.rs`) — add tokens:
```
Plus  Minus  Star  Slash  Percent
Assign(=)  PlusAssign(+=)  MinusAssign(-=)  StarAssign(*=)  SlashAssign(/=)  PercentAssign(%=)
Incr(++)  Decr(--)
Eq(==)  Ne(!=)  Lt(<)  Le(<=)  Gt(>)  Ge(>=)
And(&&)  Or(||)  Not(!)
```

**AST** (`ast.rs`) — add expression variants:
```rust
pub enum Expr {
    // existing: Num, Str, Ident, Field, FieldExpr
    BinOp  { op: BinOp, left: Box<Expr>, right: Box<Expr> },
    UnaryOp { op: UnaryOp, expr: Box<Expr> },
    Assign  { name: String, value: Box<Expr> },
    AssignOp { op: BinOp, name: String, value: Box<Expr> },
    Incr    { name: String, delta: i64, post: bool },
}

pub enum BinOp { Add, Sub, Mul, Div, Mod, Eq, Ne, Lt, Le, Gt, Ge, And, Or, Concat }
pub enum UnaryOp { Neg, Not }
```

**Parser** (`parser.rs`) — implement Pratt / precedence-climbing for expressions.
Precedence (low → high):
1. `||`
2. `&&`
3. `== != < <= > >=`
4. `+ -` and string concat
5. `* / %`
6. unary `- !`
7. `$` `++` `--`

**Interpreter** (`interp.rs`) — add `eval_expr` arms:
- `BinOp::Add/Sub/Mul/Div/Mod` → coerce both sides to f64, return `Value::Num`
- `BinOp::Concat` → coerce both sides to string, return `Value::Str`
- `BinOp::Eq/Ne/Lt/Le/Gt/Ge` → compare, return `Value::Num(1.0)` or `Value::Num(0.0)`
- `Assign` / `AssignOp` → evaluate RHS, store in `env.vars`
- `Incr` → read var, add delta, store back, return old (post) or new (pre) value

**Env** (`builtins.rs`) — add `set_var(name, value)` method.

**Files:** `lexer.rs`, `ast.rs`, `parser.rs`, `interp.rs`, `builtins.rs`, `value.rs`

**Done when:**
```
cargo run -- '{print $2 + 10}' tests/test1.txt          # 77 88 99
cargo run -- '{total += $2} END{print total}' tests/test1.txt   # 234
cargo run -- '{print $1 " scored " $2}' tests/test1.txt
cargo run -- '{x = $2 * 2; print x}' tests/test1.txt
```

---

## Feature 5 — Field Separator (FS / -F)

**What it enables:**
```
awk_interpreter -F, '{print $1}' data.csv
awk_interpreter 'BEGIN{FS=":"} {print $1}' /etc/passwd
```

**CLI** (`main.rs`) — parse `-F sep` before the program string:
```rust
// args: awk_interpreter -F, '{print $1}' file.txt
if arg == "-F" { fs = next_arg.to_string(); }
```

**Env** (`builtins.rs`) — `Env::new` already accepts `fs: String`. Wire it through:
- `set_line` uses `self.fs` to split fields
- Writing `FS` from AWK (`BEGIN { FS = "," }`) updates `self.fs` for subsequent lines

**Interpreter** (`interp.rs`) — pass `fs` from CLI into `Interp::new(fs)`.

**Files:** `main.rs`, `builtins.rs`, `interp.rs`

**Done when:**
```
cargo run -- -F, '{print $2}' tests/test_csv.txt
cargo run -- 'BEGIN{FS=","}{print $1}' tests/test_csv.txt
```

---

## Feature 6 — if / else + Comparisons

**What it enables:**
```
{ if ($2 >= 75) print $1, "distinction"
  else if ($2 >= 50) print $1, "pass"
  else print $1, "fail" }
```

Note: comparison operators (`==`, `>`, `<`, etc.) are already added in Feature 4.
This feature adds the `if/else` statement on top of them.

**AST** (`ast.rs`) — add statement variant:
```rust
pub enum Stmt {
    // existing...
    If { cond: Expr, then: Vec<Stmt>, else_: Option<Vec<Stmt>> },
}
```

**Parser** (`parser.rs`) — parse `if (expr) { stmts }` with optional `else { stmts }` or `else if`.

**Interpreter** (`interp.rs`) — evaluate condition with `value.is_truthy()`, run the matching branch.

**Files:** `ast.rs`, `parser.rs`, `interp.rs`

**Done when:**
```
cargo run -- '{if($2>=75)print $1,"pass"; else print $1,"fail"}' tests/test1.txt
cargo run -- '$2>70{print $1}' tests/test1.txt
```

---

## Feature 7 — Loops

**What it enables:**
```
{ for (i = 1; i <= NF; i++) print $i }
{ i = 1; while (i <= NF) { print $i; i++ } }
```

**AST** (`ast.rs`) — add statement variants:
```rust
pub enum Stmt {
    // existing...
    While   { cond: Expr, body: Vec<Stmt> },
    For     { init: Option<Box<Stmt>>, cond: Option<Expr>, update: Option<Expr>, body: Vec<Stmt> },
    Break,
    Continue,
    Next,
    Exit(Option<Expr>),
}
```

**Parser** (`parser.rs`):
- `while (cond) { body }`
- `for (init; cond; update) { body }`
- `break`, `continue`, `next`, `exit`

**Interpreter** (`interp.rs`):
- Use a `ControlFlow` enum (`Break`, `Continue`, `Next`, `Exit`) as a return signal from `exec_stmts`
- Loop body checks the signal and breaks/continues accordingly
- `next` bubbles up to the main rule loop to skip to the next input line
- `exit` bubbles up to stop all processing

**Files:** `ast.rs`, `parser.rs`, `interp.rs`

**Done when:**
```
cargo run -- '{for(i=1;i<=NF;i++) print $i}' tests/test1.txt
cargo run -- 'NR==1{next} {print}' tests/test1.txt
```

---

## Feature 8 — Built-in String Functions

**What it enables:**
```
{ print length($1) }
{ print toupper($1) }
{ print substr($1, 1, 2) }
{ gsub(/a/, "A"); print }
```

**AST** (`ast.rs`) — add function call expression:
```rust
pub enum Expr {
    // existing...
    Call { name: String, args: Vec<Expr> },
}
```

**Parser** (`parser.rs`) — when an `Ident` is followed by `(`, parse it as a `Call`.

**Interpreter** (`interp.rs`) — dispatch `Call` by name to builtin implementations.

**Builtins** (`builtins.rs`) — implement:

| Function | Behaviour |
|----------|-----------|
| `length(s)` | length of string (or `$0` if no arg) |
| `substr(s, start)` | substring from 1-based position |
| `substr(s, start, len)` | substring of given length |
| `index(s, t)` | 1-based position of t in s, 0 if not found |
| `split(s, arr, sep)` | split s into array arr, return count |
| `sub(regex, repl, s)` | replace first match in s |
| `gsub(regex, repl, s)` | replace all matches in s |
| `match(s, regex)` | position of match, sets RSTART / RLENGTH |
| `tolower(s)` | lowercase |
| `toupper(s)` | uppercase |
| `sprintf(fmt, ...)` | format string, return as value |

Add `regex` crate for `sub`/`gsub`/`match`:
```toml
# Cargo.toml
[dependencies]
regex = "1.10"
```

**Files:** `ast.rs`, `parser.rs`, `interp.rs`, `builtins.rs`, `Cargo.toml`

**Done when:**
```
cargo run -- '{print toupper($1)}' tests/test1.txt
cargo run -- '{print substr($1,1,2)}' tests/test1.txt
cargo run -- '{gsub(/a/,"@"); print}' tests/test1.txt
```

---

## Feature 9 — Arrays

**What it enables:**
```
{ count[$1]++ }
END { for (k in count) print k, count[k] }
```

**Env** (`builtins.rs`) — add a separate array store:
```rust
pub arrays: HashMap<String, HashMap<String, Value>>,
```

**AST** (`ast.rs`) — add variants:
```rust
pub enum Expr {
    // existing...
    ArrayAccess { name: String, index: Box<Expr> },
}

pub enum Stmt {
    // existing...
    Delete { name: String, index: Option<Box<Expr>> },
}
```

**Parser** (`parser.rs`):
- `arr[expr]` → `ArrayAccess`
- Assignment to `arr[expr]` → handled in `Assign` by detecting the array access on the left
- `for (k in arr) { }` → `ForIn` variant (add to `Stmt`)
- `delete arr[k]` / `delete arr`

**Interpreter** (`interp.rs`):
- `ArrayAccess` → look up in `env.arrays`
- Assignment to array element → store in `env.arrays`
- `ForIn` → iterate over the array's keys
- `Delete` → remove key or entire array

**Files:** `ast.rs`, `parser.rs`, `interp.rs`, `builtins.rs`

**Done when:**
```
cargo run -- '{count[$1]++} END{for(k in count)print k,count[k]}' tests/test1.txt
cargo run -- '{scores[$1]=$2} END{print scores["Sam"]}' tests/test1.txt
```

---

## Feature 10 — printf

**What it enables:**
```
{ printf "%-10s %3d\n", $1, $2 }
```

**AST** (`ast.rs`) — add statement variant:
```rust
pub enum Stmt {
    // existing...
    Printf { fmt: Expr, args: Vec<Expr> },
}
```

**Parser** (`parser.rs`) — `printf` keyword followed by format expr and comma-separated args.

**Interpreter** (`interp.rs`) — implement format string processing:

| Specifier | Meaning |
|-----------|---------|
| `%s` | string |
| `%d` | integer |
| `%f` | float |
| `%e` | scientific notation |
| `%g` | shorter of %f / %e |
| `%-10s` | left-align with width |
| `%05d` | zero-pad |
| `%%` | literal `%` |

No automatic newline (unlike `print`).

**Files:** `ast.rs`, `parser.rs`, `interp.rs`

**Done when:**
```
cargo run -- '{printf "%-10s %3d\n", $1, $2}' tests/test1.txt
cargo run -- 'END{printf "Total: %d\n", total}' tests/test1.txt
```

---

## Build Order

```
1  print              ✅ done
2  Fields             ✅ done
3  BEGIN/END + regex  ← start here
4  Variables + arith  ← then this (needed by patterns anyway)
5  Field separator    ← quick win, independent
6  if/else            ← builds on 4
7  Loops              ← builds on 6
8  String functions   ← needs function call parsing from 4
9  Arrays             ← needs for-in from 7
10 printf             ← last, needs format string logic
```

---

## Testing

Add test files under `tests/` as features are added:

```
tests/
├── test1.txt           — name/marks (exists)
├── test_csv.txt        — comma-separated data (for -F)
├── test_arithmetic.txt — numeric data
└── README.md
```

For each feature, run our binary and compare output to real `awk` (via WSL or Git Bash):

```bash
awk '{print $2 + 10}' tests/test1.txt          # real awk
cargo run -- '{print $2 + 10}' tests/test1.txt  # ours
```

Output must be identical.
