# Praharshitha's Learning Log — AWK Interpreter

## 1. Basic Architecture of the AWK Interpreter

The interpreter follows a classic pipeline:

```
input → Lexer → tokens → Parser → AST → Evaluator → output
```

Each stage has a single, clear responsibility:

| Stage | What it does |
|-------|-------------|
| Lexer | Reads raw source text and breaks it into tokens (keywords, operators, literals, identifiers) |
| Parser | Consumes the token stream and builds an Abstract Syntax Tree (AST) |
| AST | A recursive enum tree — every expression, statement, and pattern is a variant |
| Value | Dual-type (string + number) with automatic coercion — the heart of AWK's semantics |
| Evaluator | Recursively walks the AST and executes each node |
| Builtins | String and math functions (length, substr, split, gsub, sub, match, sprintf, etc.) |

**Key insight:** The Value type is what makes AWK feel dynamic. A variable can be used as a number in arithmetic and as a string in concatenation — the same value, coerced on demand. Getting this right was the most important design decision in the whole project.

**Pattern-action model:** An AWK program is a list of `pattern { action }` pairs. The evaluator loops over every input record, tests each pattern, and runs the matching action. `BEGIN` and `END` are special patterns that run before and after all input.

---

## 2. Languages Exploration — Rust, Go, and Crystal

The interpreter was built in three languages. Each taught something different.

**Rust (primary)**
- The AST is a recursive enum. Every node is a variant. `match` over the enum is exhaustive — the compiler forces you to handle every case.
- Ownership and borrowing made the evaluator harder to write but eliminated a whole class of bugs (use-after-free, double-free).
- `cargo build --release` produces a fast, self-contained binary.

**Go (port)**
- Go has no enums. We used `iota` constants and interface types to represent AST nodes.
- This works, but you lose exhaustiveness checking — the compiler won't tell you if you forgot a case.
- Go's simplicity made the port readable, but the lack of algebraic data types made the AST more verbose.

**Crystal (port)**
- Crystal looks like Ruby but compiles to native code with static typing.
- It has proper enums and exhaustive `case` expressions, similar to Rust.
- The syntax is the cleanest of the three for a tree-walk interpreter.

**What comparing three languages taught:**

The same algorithm — lexing, parsing, tree-walking — looks very different depending on what the language gives you. Rust's enums and `match` are the best fit for an interpreter. Go's interfaces are workable but noisy. Crystal sits in between. The architecture stayed the same; only the expression of it changed.

---

## 3. Running Test Cases and Efficiency

**How we tested**
- Created structured input files: student records, CSV data, employee tables, numeric data, edge cases (empty lines, irregular whitespace, single-column data).
- Ran the same AWK program against our interpreter and against the system `awk`.
- Compared outputs. A match means the interpreter is correct.

**What the tests covered**
- Basic field access (`$1`, `$2`, `$NF`)
- Record and field counts (`NR`, `NF`)
- Pattern matching (regex, comparison)
- Arithmetic and string operations
- `BEGIN` / `END` blocks
- Custom field separators (`-F`)
- Edge cases: out-of-range fields, blank lines, leading/trailing whitespace

**Efficiency lessons**
- A tree-walk interpreter is simple to build and correct to reason about, but slower than bytecode or JIT approaches. For AWK programs on typical input sizes, this is fine.
- The bottleneck in practice is I/O (reading lines), not the tree-walk itself.
- Parity testing (comparing output against system `awk`) is the most reliable way to catch correctness bugs — more reliable than writing expected outputs by hand.

**Scripts used**
- `tests/run_parity.sh` — runs a suite of AWK programs on all three interpreters and diffs the output.
- `tests/verify_parity.sh` — focused parity check between Rust and Crystal builds.

---

## 4. Why a Manual and Documentation Matter

**The manual (`docs/MANUAL.md`)**

Writing the manual was not an afterthought — it was a forcing function. To document a feature, you have to run it, understand its edge cases, and explain it clearly. Every section of the manual that was hard to write pointed to a feature that was either incomplete or poorly understood.

The manual covers:
- How to run the interpreter (`-f`, `-F`, `-v` flags)
- Program structure (pattern-action pairs, BEGIN/END)
- Fields, records, and built-in variables
- Operators, control flow, arrays, built-in functions
- Worked examples

**The spec (`docs/specs.md`)**

Before writing code, we wrote `specs.md` — a feature list with pseudocode for each feature. This gave us:
- A clear build order (what to implement next)
- Language-agnostic pseudocode that translated into Rust, Go, and Crystal
- A status table to track which features were done in which language

Without the spec, the three ports would have drifted apart. With it, they stayed in sync.

**The man page (`docs/awki.1`)**

Man pages are the standard documentation format for CLI tools on Unix/Linux. They are written in troff markup and rendered by groff. Key macros: `.TH` (title), `.SH` (section), `.TP` (option), `.B` (bold), `.I` (italic).

After writing the `.1` file, `make install` copies it to `/usr/local/share/man/man1/` and runs `mandb` to update the index. After that, `man awki` works from any terminal — offline, always available, the same way `man awk` or `man ls` works.

**The core lesson:** Documentation is not separate from the project — it is part of it. A tool without a manual is incomplete. Writing the manual, the spec, and the man page made the interpreter more correct, more consistent across three languages, and actually usable by someone who wasn't on the team.

---

## Summary

| Area | Key takeaway |
|------|-------------|
| Architecture | A clean pipeline (Lexer → Parser → AST → Evaluator) makes each layer independently testable and replaceable |
| Languages | The same algorithm looks different in each language; the language shapes how naturally the architecture fits |
| Testing | Parity testing against system `awk` is the ground truth; structured edge-case inputs catch the bugs that happy-path tests miss |
| Documentation | Writing the manual and spec forces correctness; the man page is the standard for CLI tools and should be written, not skipped |
