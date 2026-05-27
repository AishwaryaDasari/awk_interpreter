# AWK Interpreter — From Scratch in Three Languages

---

## What is AWK?

A text-processing language from Bell Labs (1977).
A program is `pattern { action }` pairs.

```awk
{ print $1, $3 }          # print 1st & 3rd field of every line
/error/ { print NR, $0 }  # print line number + full line when "error" matches
BEGIN { sum = 0 }         # runs before first line
END   { print sum }       # runs after last line
```

One binary, no dependencies — reads from files or stdin.

---

## Pipeline

```
input ──► Lexer ──► Parser ──► AST ──► Evaluator ──► output
```

| Layer | Job |
|-------|------|
| **Lexer** | Source text → tokens (`print`, `$1`, `{`, etc.) |
| **Parser** | Tokens → AST tree |
| **AST** | Statement / expression / pattern node types |
| **Evaluator** | Walk the tree, run each node |
| **Builtins** | `length`, `substr`, `gsub`, `split`, `printf`… |

---

## The Project

Implement **all 10 AWK features** in **three languages**:

| # | Feature |
|---|---------|
| 1 | `print` — output with OFS/ORS |
| 2 | Fields — `$0`, `$n`, `NF`, `NR` |
| 3 | BEGIN/END blocks + regex patterns |
| 4 | Variables + arithmetic (`+`, `-`, `*`, `/`, `%`, `^`) |
| 5 | Field separator — `-F` flag, `FS` variable |
| 6 | `if` / `else` with comparisons |
| 7 | Loops — `for`, `while`, `break`, `continue`, `next`, `exit` |
| 8 | String functions — `length`, `substr`, `gsub`, `match`… |
| 9 | Associative arrays — `a["key"]`, `delete`, `for…in` |
| 10 | `printf` — `%s`, `%d`, `%f`, width, precision |

---

## Languages & Timelines

```
Rust   ■■■■■■■■■■■■■■■■■■■■  (primary — full implementation)
Go     ■■■■■■■■■□□□□□□□□□□□  (port — features 1–5)
Crystal■■■■■■■■■■■■■■■■■■■■  (port — all 10 features)
```

### Key commits

| Date | Event |
|------|-------|
| `c1eef30` | Initial commit — project scaffold |
| `5a8a623` | Rust: basic `print` with lexer + parser + evaluator |
| `ec170b4` | Rust: test suite |
| `42bddb9` | Rust: variables, arithmetic, assignment |
| `210ef86` | Rust: if/else, FNR |
| `22ba7dd` | Rust: loops (for, while, break, continue, next, exit) |
| `ca53436` | Go: port from Rust (same architecture) |
| `e4a3311` | Crystal: initial implementation |
| `e6ddb9c` | Crystal: all 10 features complete |
| `60e2afa` | Crystal: lexer division/regex fix, printf format, test.sh |

---

## Architecture (Rust)

```
src/
├── main.rs       CLI — arg parsing, file/STDIN I/O
├── lexer.rs      Source → token stream
├── parser.rs     Token stream → AST
├── ast.rs        AST node types (enums)
├── value.rs      Dual-type (string + number)
├── builtins.rs   FS, OFS, NR, NF… + string functions
└── interp.rs     Tree-walk evaluator
```

The entire AST is a tree of enums. Evaluation is a single `match` over variants.

---

## Architecture (Crystal)

```
src/
├── awk.cr        CLI entry point
├── lexer.cr      Tokeniser
├── parser.cr     Recursive-descent parser
├── ast.cr        AST node classes
├── value.cr      AwkValue (Float64 | String)
├── builtins.cr   Env (vars + fields + arrays)
└── interp.cr     Tree-walk evaluator
```

Same architecture, but uses Crystal idioms:
- Classes instead of enums for AST
- `String::Builder` for efficient output
- `ControlFlow` enum (not exceptions) for break/continue/exit

---

## Key Design Decisions

### Dual-type Value
```
AwkValue holds either Float64 or String.
Arithmetic coerces to number; concatenation coerces to string.
```

### Control Flow as Enum (Crystal)
```crystal
enum ControlFlow
  Normal | Break | Continue | Next | Exit
end
```
Not exceptions — avoids performance pitfalls.

### No Bytecode / No JIT
Pure tree-walk interpreter. Each node knows how to evaluate itself recursively.

---

## Bugs Found & Fixed

| Bug | Root Cause | Fix |
|-----|-----------|-----|
| `x=42` stored but `print x` showed `0` | `parse_assign` discarded the `AssignExpr` | Added `return` |
| `if…else` with `;` separator crashed | Parser didn't skip `;` before `else` | Skip semicolons in `parse_if` |
| `for` loop `continue` skipped update | `next` jumped past increment expression | Fall through to update |
| `$0 = "hello"` failed | `set_field` didn't init empty field array | Handle `n == 0` regardless |
| `/` always treated as regex | Lexer had no context for division | Check preceding character |
| `printf "%f"` showed `3.14` not `3.140000` | Missing precision/width formatting | Use `sprintf` with default 6 digits |
| `match` failure set `RLENGTH=0` instead of `-1` | Wrong constant | Changed to `-1.0` |

---

## Testing

### `crystal/test.sh` — 104 tests against system awk

```
━━━ 1. Print ━━━━━━━━━━━━━━━━━━━━━
  ✓ print no args
  ✓ print string
  ✓ print number
  ⋮

━━━ 10. Printf ━━━━━━━━━━━━━━━━━━━
  ✓ printf string
  ✓ printf float
  ✓ printf width
  ⋮

══════════════════════════════════
  104 tests  ·  104 passed  ·  0 failed
══════════════════════════════════
```

Each test:
1. Runs an AWK program through **system `awk`** (reference)
2. Runs the same program through **our interpreter**
3. Compares output — byte-for-byte identical

---

## Feature Status

| # | Feature | Rust | Go | Crystal |
|---|---------|------|----|---------|
| 1 | print | ✅ | ✅ | ✅ |
| 2 | Fields | ✅ | ✅ | ✅ |
| 3 | BEGIN/END + regex patterns | ✅ | ✅ | ✅ |
| 4 | Variables & arithmetic | ✅ | ✅ | ✅ |
| 5 | Field separator | ✅ | ✅ | ✅ |
| 6 | if/else | ✅ | 🔲 | ✅ |
| 7 | Loops | ✅ | 🔲 | ✅ |
| 8 | String functions | ✅ | 🔲 | ✅ |
| 9 | Arrays | ✅ | 🔲 | ✅ |
| 10 | printf | ✅ | 🔲 | ✅ |

Rust: **10/10** · Go: **5/10** · Crystal: **10/10**

---

## Try It Yourself

```sh
cd crystal

# Build
crystal build src/awk.cr -o awk_crystal

# Run
./awk_crystal 'BEGIN{print "hello"}'
echo "hello world" | ./awk_crystal '{print $2}'
printf "a\nb\nc" | ./awk_crystal '/a/,/c/{print NR, $0}'
./awk_crystal -F, '{print $1, $3}' data.csv

# Test
./test.sh
```

---

## Architecture Diagram

```
                   ┌──────────────┐
    source ───────►│    Lexer     │─── tokens ────┐
                   └──────────────┘               │
                                                  ▼
                   ┌──────────────┐    ┌──────────────┐
                   │  Builtins    │    │   Parser     │
                   │ (FS, OFS,    │    │              │
                   │  NR, NF…)    │    └──────┬───────┘
                   └──────┬───────┘           │
                          │                   │ AST
                          ▼                   ▼
                   ┌──────────────┐    ┌──────────────┐
                   │ Evaluator    │◄───│     AST      │
                   │ (tree-walk)  │    │ (node tree)  │
                   └──────┬───────┘    └──────────────┘
                          │
                          ▼
                   ┌──────────────┐
                   │   stdout     │
                   └──────────────┘
```

---

## Lessons Learned

1. **AWK semantics are subtle** — `/` can be division or regex depending on context; `for(;;) continue` must still run the update expression; `$0` assignment resplits into fields.

2. **Tree-walk interpreters are simple but correct** — no bytecode needed to match GNU AWK output.

3. **Porting across languages exposes design debt** — Crystal's `Hash` iteration order differs from Rust's, and `Float64` comparison semantics vary.

4. **Testing against the system reference catches everything** — 104 tests, each comparing byte-for-byte against `awk`, found every bug.

---

## Questions?
