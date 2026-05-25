## AWK INTERPRETER ##

### About awk

AWK is a language for data manipulation, text retrieval, and prototyping algorithms. An AWK program is a sequence of `pattern { action }` pairs and function definitions. Short programs are entered on the command line (enclosed in quotes to avoid shell interpretation). Longer programs can be read from a file with the `-f` option.

Data input is read from the files given on the command line, or from standard input when no files are given. The input is broken into records as determined by the record separator `RS` (default `"\n"`, so records are lines). Each record is compared against each pattern; on a match, the corresponding action is executed.

This interpreter follows the AWK language as defined in *The AWK Programming Language* (Aho, Kernighan, Weinberger, 1988), conforms to the POSIX 1003.2 (draft 11.3) specification, and includes a small number of extensions.

---

### What we are going to build

A working AWK interpreter from scratch — one that can run real AWK programs from the terminal, the same way the system `awk` command does.

The interpreter is built in **Rust** as the primary language, then ported to **Go** and **Crystal**. All three versions run the same programs and produce identical output. The ports are not hurried translations — they are careful reimplementations that use each language's own idioms.

The project is a backend CLI tool: no web UI, no frontend — just a binary you run in the terminal.

---

### Architecture

```
input ──► Lexer ──► tokens ──► Parser ──► AST ──► Evaluator ──► output
                              ▲
                         ┌────┴────┐
                         │  Value  │
                         │ (str +  │
                         │  num)   │
                         └─────────┘
```

```
rust/src/
├── main.rs       CLI entry point — arg parsing, stdin/file I/O
├── lib.rs        Library root
├── lexer.rs      Tokeniser — produces TokenKind stream from source
├── parser.rs     Builds AST from token stream
├── ast.rs        AST node definitions — enums for Stmt, Expr, Pattern
├── value.rs      Dual-type Value (string + number) with coercion
├── interp.rs     Tree-walk evaluator over the AST
└── builtins.rs   Built-in functions — length, substr, split, gsub, etc.
```

| Layer | Role |
|-------|------|
| **Lexer** | Tokenises input into a `TokenKind` stream |
| **Parser** | Builds AST (enum tree) from the token stream |
| **AST**** | Recursive enum tree: every node is a variant |
| **Value** | Dual-type (string + number) with automatic coercion |
| **Evaluator** | Recursive descent tree-walk over the AST |
| **Builtins** | String and math functions (length, substr, split, gsub, sub, match, tolower, toupper, sprintf) |
| **Entry** | Parses CLI flags (`-f`, `-F`, `-v`), reads from files or stdin, wires everything together |

**Key design decisions:**
- The entire AST is a tree of Rust enums. Evaluation is a `match` over the enum tree — every expression, statement, and pattern is a variant.
- `Value` holds both a string and numeric representation. Arithmetic coerces to number; string ops coerce to string. This matches AWK's exact semantics.
- Tree-walk interpreter: recursively descends the AST and executes nodes in place. No bytecode, no JIT.

---

### What we shall be learning

**Enums and pattern matching** — the entire AST is a tree of Rust enums. Every node, expression, and statement is a variant. The evaluator is built entirely from `match` expressions over these enums.

**Ownership and borrowing** — the interpreter holds mutable state (variables, fields, the current line) while also traversing immutable AST nodes. Getting this to compile without cloning everything teaches you how Rust's borrow checker actually thinks.

**Error handling** — a real interpreter needs to report errors with line numbers and context. We use `Result<T, E>` and the `?` operator throughout the codebase.

**Traits and generics** — the `Value` type (every AWK value is simultaneously a string and a number) implements `Display`, `From`, `PartialOrd`, and arithmetic operator traits.
