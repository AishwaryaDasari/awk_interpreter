# AWK Three Ways: A Study in Rust, Go and Crystal

*Project Title: AWK Interpreter*
*Rust: edition 2021 (package v0.1.0) &nbsp;·&nbsp; Go: 1.21 &nbsp;·&nbsp; Crystal: package v0.1.0*
*Date: May 2026 &nbsp;·&nbsp; Organization: WingsAI / We4Shakthi*

## What is AWK?

AWK is a language for manipulation of data files, text retrieval and processing, and for prototyping algorithms. An AWK program is a sequence of `pattern { action }` pairs and function definitions. Short programs are entered on the command line (enclosed in quotes to avoid shell interpretation). Longer programs can be read from a file with the `-f` option. Data input is read from files on the command line or from standard input when the list is empty. The input is broken into records as determined by the record separator variable `RS` (default `"\n"`, so records are lines). Each record is compared against each pattern; on a match, the corresponding action is executed.

## The Three Implementations
This repository provides a three-way implementation of an AWK interpreter in Rust, Go, and Crystal. All three follow the same architecture — a lexer, recursive-descent parser, AST, and tree-walking interpreter with built-in AWK functions — making the project a useful resource for studying how the same language can be implemented across different compiled, type-safe runtimes. Rust serves as the primary implementation (installable via cargo install), while Go and Crystal offer side-by-side comparisons of performance, ergonomics, and language features.

## Approach

All three implementations share the same overall approach: a hand-written character-level lexer feeding a recursive descent parser (Go alone uses a Pratt parser) that produces an AST, which is then walked by a tree-walking interpreter. There is no intermediate representation, bytecode, or JIT — the AST is evaluated directly against input records. 

The core design mirrors the classic AWK execution model: BEGIN blocks run first, then a line-by-line pattern-match loop over input files, then END blocks. String and numeric values are unified throughout under implicit coercion — no explicit casting, just AWK semantics.

### Architecture Overview (three-way identical)

| Stage | Input → Output | Key Data |
|---|---|---|
| **Lexer** | AWK source → token stream | Keywords (`print`, `if`), operators (`+`, `=`), literals (strings, numbers, regexes) |
| **Parser** | Token stream → AST | Recursive descent (Rust/Crystal) or Pratt (Go); builds typed node tree |
| **AST** | Program node containing rules | Rule variants (Begin/End/Pattern); Stmt variants (Print/If/While/For); Expr with binary/unary ops |
| **Interpreter** | AST + input lines → output | Tree-walker with environment (variables, fields, FS); exec_begin / exec_line / exec_end phases |
| **Entry** | CLI args → execution | Parses `-F`, `-v`, `-f`; reads stdin/files; orchestrates Begin → line loop → End phases |

## Where They Diverge

| | Rust | Go | Crystal |
|---|---|---|---|
| **Completeness** | Mid-range — `do-while`, `gensub`, `#` comments | Minimal — 5 keywords, no control flow, no built-ins | Most complete — 18 built-ins, `printf`, `delete`, `^` |
| **Value type** | Algebraic enum `Value { Str, Num }` | Struct with `IsNum` bool flag | Union type `String \| Float64` |
| **AST** | Enums everywhere, exhaustive `match` | Interface/struct, type-switch dispatch | Class hierarchy, `case/when` dispatch |
| **Parser** | Layered recursive descent | Pratt parser | Recursive descent |
| **`/` disambiguation** | Token-context flag (`prev_was_operand`) | None — `/` always = REGEX token | Char-context heuristic (prev char) |

**Rust** — the primary implementation — leverages algebraic enums and exhaustive `match` throughout, making the AST and value system type-safe at compile time. Its lexer's `prev_was_operand` flag is the most principled approach to `/` disambiguation among the three. The parser uses layered recursive descent with separate methods per precedence level. Rust alone implements `gensub`, `do-while`, and `#` comment handling.

**Go** takes a distinctly different approach: it uses a Pratt parser (single `parseExpr(minPrec)` function) instead of layered recursive descent, and values are a struct with an `IsNum` boolean flag to simulate a tagged union — Go lacks algebraic types, so this is the pragmatic alternative. The lexer has no `/` disambiguation at all (`/` always produces a token). Go is the most minimal implementation: 5 keywords (print, BEGIN, END, NR, NF), no control flow, no built-in functions — essentially an AWK subset that handles print, fields, patterns, and arithmetic.

**Crystal** is the most feature-complete — 18 built-in functions (including `sin`, `cos`, `sqrt`, `rand`), a `printf` statement, `delete`, `^` exponentiation, and the `in` array-membership operator. It uses a class hierarchy for the AST with `case/when` dispatch and a language-level union type (`String | Float64`) for values. Its lexer uses a simpler character-context heuristic (checking the previous character) for `/` disambiguation, and it has the most complete string escape handling (octal, hex, etc.).

## Core Data Structure

All program state lives in a single AST (shown here in Rust syntax):

A program is a list of rules, where each rule is one of:

```rust
Rule::Begin(Vec<Stmt>)             // BEGIN { ... }
Rule::End(Vec<Stmt>)               // END { ... }
Rule::BeginFile(Vec<Stmt>)         // BEGINFILE { ... }
Rule::EndFile(Vec<Stmt>)           // ENDFILE { ... }
Rule::Pattern(Pattern, Vec<Stmt>)  // pattern { ... }
```

Patterns are either a bare expression or a range:

```rust
Pattern::Expr(Expr)           // matches when truthy
Pattern::Range(Expr, Expr)    // matches from expr1 to expr2
```

Each statement block is `Vec<Stmt>`, where a statement is:

```rust
Stmt::Print(Vec<Expr>)                                         // print [expr ...]
Stmt::Expr(Expr)                                               // expression statement
Stmt::If(Expr, Vec<Stmt>, Option<Vec<Stmt>>)                   // if (cond) { ... } [else { ... }]
Stmt::While(Expr, Vec<Stmt>)                                   // while (cond) { ... }
Stmt::DoWhile(Vec<Stmt>, Expr)                                 // do { ... } while (cond)
Stmt::For(Option<Expr>, Option<Expr>, Option<Expr>, Vec<Stmt>) // for (init; cond; incr) { ... }
Stmt::ForIn(String, Expr, Vec<Stmt>)                           // for (var in array) { ... }
Stmt::Break / Continue / Next
Stmt::Exit(Option<Expr>)                                       // exit [code]
```

Expressions are the recursive core:

```rust
Expr::Field(usize)                              // $N
Expr::FieldExpr(Box<Expr>)                      // $(expr)
Expr::Ident(String)                             // variable name
Expr::Str(String)                               // string literal
Expr::Num(f64)                                  // numeric literal
Expr::BinOp(Box<Expr>, BinOpKind, Box<Expr>)    // lhs op rhs
Expr::UnaryOp(UnaryOpKind, Box<Expr>)           // -expr / !expr
Expr::Ternary(Box<Expr>, Box<Expr>, Box<Expr>)  // cond ? then : else
Expr::Assign(String, Box<Expr>)                 // var = expr
Expr::AssignOp(BinOpKind, String, Box<Expr>)    // var op= expr
Expr::Incr(String, bool)                        // var++ / ++var
Expr::Decr(String, bool)                        // var-- / --var
Expr::Call(String, Vec<Expr>)                   // func(args...)
Expr::ArrayAccess(String, Box<Expr>)            // arr[idx]
```

Binary operators (`BinOpKind`): `Add`, `Sub`, `Mul`, `Div`, `Mod`, `Eq`, `Ne`, `Lt`, `Gt`, `Le`, `Ge`, `And`, `Or`, `Match`, `NotMatch`, `Concat`

Unary operators (`UnaryOpKind`): `Neg`, `Not`

## The Four-Phase Pipeline

**Phase 1 — Lex (Tokenization)**
The source string is read character-by-character into a `Lexer` that produces a flat token list. It recognizes AWK keywords (`print`, `if`, `while`, `for`, `break`, `next`, `exit`, `begin`, `end`), operators (`+`, `-`, `*`, `/`, `%`, `~`, `!~`, `==`, `!=`, `<`, `>`, `<=`, `>=`, `&&`, `||`), assignment variants (`=`, `+=`, `-=`, etc.), literals (strings in double quotes, numbers as floats, regexes between `/` delimiters), identifiers, field references (`$1`, `$NF`), and structural tokens (`{`, `}`, `(`, `)`, `[`, `]`, `,`, `;`). Newlines and semicolons both act as statement terminators. The lexer tracks no state — it emits a linear token stream consumed immediately by the parser.

**Phase 2 — Parse (Recursive Descent / Pratt)**
Each `Parser` wraps its `Lexer` and follows AWK grammar: a program is a sequence of `Rules` — each rule is either `BEGIN`, `END`, `BEGINFILE`, `ENDFILE` (each followed by a brace block), or a `Pattern` (an expression or range) paired with a brace block. Expressions are parsed with operator precedence (unary, multiplicative, additive, comparison, pattern matching, logical AND, logical OR). Rust and Crystal use recursive descent with separate methods per precedence level; Go uses a single Pratt/precedence-climbing `parseExpr(minPrec)` function. Rust/Crystal additionally handle ternary, `for-in`, `do-while`, `if-else`, `break`/`continue`/`next`/`exit`, array access, and implicit string concatenation. On success, a typed AST is returned; on failure, a syntax error is reported.

**Phase 3 — Interpret (Tree-Walking)**
The `Interp` holds an `Env` containing all variables (including `$0`, `$1`... fields, field count `NF`, record number `NR`, filename `FILENAME`, field separator `FS`, etc.) and walks the AST in four sequential passes:

- `exec_begin` — runs all `BEGIN` rules.
- `exec_begin_file` / `exec_line` loop — for each input file (or stdin), runs `BEGINFILE` rules, then for every line reads the record, splits into fields via `FS`, evaluates pattern matches on each rule, and executes matching statement blocks. Supports `next` (skip to next line), `break/continue` (inside loops), and `exit` (immediate termination).
- `exec_end_file` — runs `ENDFILE` rules after each file.
- `exec_end` — runs all `END` rules after all input is consumed.

Expression evaluation recursively resolves `$N` field access, array indexing, built-in function calls, and all operators — producing values with implicit numeric/string coercion matching AWK semantics. The supported built-in set varies by implementation: Rust has 11 string functions (`length`, `substr`, `index`, `split`, `sub`, `gsub`, `gensub`, `match`, `sprintf`, `tolower`, `toupper`), Crystal has 18 (adding `printf`, `int`, `sqrt`, `sin`, `cos`, `exp`, `log`, `rand`, `srand`), and Go has none.

**Phase 4 — Output**
All `print` and `printf` statements write directly to an accumulating string buffer returned by each execution phase. The entry point prints these buffers to stdout in order: Begin output, per-line output (for each file), End output. If `exit(expr)` was called, the process exits with the given code. No post-processing or pretty-printing is applied — the output is raw AWK result text.

## Lessons Learned

### What Went Well

- **Clean architecture from day one** — committing to lex → parse → AST → eval early meant each language port was a straightforward translation of the same pipeline, not a redesign.
- **Parity testing against system awk** — the `verify_parity.sh` runner caught subtle coercion and edge-case bugs that unit tests alone would have missed.
- **Incremental build order** — following a numbered feature list (print → fields → BEGIN/END → variables → if/else → loops → functions → arrays → printf) meant each stage was testable before the next began.
- **Three-language portability** — proving the same design works across Rust, Go, and Crystal validated the architecture and revealed each language's strengths (Rust's enums, Crystal's union types, Go's fast compile-turnaround).

### What Went Wrong

- **Go lexer had no `/` disambiguation** — `/` always tokenises as a regex literal, making inline division expressions impossible without surrounding spaces. This was a design oversight that would require a significant refactor to fix.
- **Crystal CLI flags incomplete** — `-F`, `-f`, and `-v` were never wired into the Crystal entry point, so it relies on inline programs only.
- **String escape handling was inconsistent** — Rust's lexer initially had no escape handling at all (`\n` was two characters), while Crystal had full octal/hex support. Achieving parity required retrofitting.
- **Arrays and field assignment were added late** — the Rust AST had no `ArrayAccess` on the left-hand side of assignment initially, requiring a restructuring when `arr[i] = val` and `$n = val` were added.

### How learnings drove improvements

- **The `/` disambiguation bug in Go** led to designing the lexer with context tracking from the start — the Rust implementation's `prev_was_operand` flag was carried into the Crystal port's character heuristic, and both work correctly for real AWK programs.
- **String escape gaps** were resolved by consolidating a single escape-handling routine that was then ported across all three languages, ensuring `\n`, `\t`, `\"`, and `\\` are consistent everywhere.
- **The incremental build order** itself was a key insight: starting with `print`, then adding fields, patterns, expressions, and so on — each stage produced a working (if minimal) interpreter, which kept motivation high and made debugging linear rather than combinatorial.
- **Porting to Go and Crystal** revealed which parts of the design were genuinely language-agnostic (the AST structure, the exec phases) and which were Rust-specific (exhaustive enum matching, the `Result`-based error propagation). This forced cleaner abstractions in the Rust original.

## Conclusion

This project demonstrates that a non-trivial language interpreter can be cleanly implemented across three different compiled languages using a shared architecture. The lex → parse → AST → eval pipeline proved portable and maintainable, while each language's type system shaped the implementation in distinct ways — Rust's algebraic enums enable exhaustive matching, Crystal's union types simplify value representation, and Go's interfaces enable straightforward polymorphism despite the lack of sum types. The primary Rust implementation is feature-complete for core AWK, with Crystal matching or exceeding it in several areas. The parity test suite ensures all three produce identical output, making this a useful reference for studying both AWK semantics and cross-language design patterns.

## References

- Aho, A. V., Kernighan, B. W., & Weinberger, P. J. (1988). *The AWK Programming Language*. Addison-Wesley.
- IEEE/ISO/IEC 1003.1-2008 (POSIX). Shell & Utilities — `awk` Utility.
- Rust Edition 2021. https://doc.rust-lang.org/
- Go 1.21. https://go.dev/doc/
- Crystal Language. https://crystal-lang.org/
- BurntSushi. `regex` crate (Rust). https://crates.io/crates/regex
- Project repository: https://repo.we4shakthi.in/wingsai/awk_interpreter

## Status

| Implementation | Completeness | Notes |
|----------------|-------------|-------|
| **Rust** | Full core — print, fields, BEGIN/END, patterns (regex + range), variables, arithmetic, if/else, all loop types, 11 built-in string functions, arrays, ternary, CLI flags, sprintf, BEGINFILE/ENDFILE | Primary implementation. Missing: printf statement, math functions, output redirection, user-defined functions |
| **Go** | Minimal subset — print, fields (NR/NF only), BEGIN/END, regex patterns, variables, arithmetic | 5 keywords only. Lacks if/else, loops, string functions, arrays, CLI flags, range patterns |
| **Crystal** | Full + extras — everything Rust has plus printf statement, 18 built-in functions (incl. math), ^ exponentiation, delete, in operator, octal/hex escapes | Most feature-complete. Missing: CLI flags (-F, -f, -v) |

## Running It / Contributing

```bash
# Rust (primary) — build once, then run
cargo build --release
cargo run -- '{print $1}' tests/test1.txt

# Go
cd go && go run . '{print $1, $2}' ../tests/test1.txt

# Crystal
cd crystal && crystal run src/awk.cr -- '{print $1, $2}' ../tests/test1.txt

# Install system-wide (Rust only)
make install

# Run parity tests (compare all three against system awk)
./tests/run_parity.sh
```

To contribute: fork the repository (see References for the URL), implement a feature in Rust first (following the numbered feature order in `docs/IMPLEMENTATION.md`), then port to Go and Crystal. Submit a merge request with parity test results confirming output matches system `awk`.
