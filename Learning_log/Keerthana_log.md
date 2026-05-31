# Keerthana's Learning Log — AWK Interpreter

## What is AWK?

AWK is a scripting tool used in Linux and Unix for processing text files. It reads input line by line, matches each line against patterns, and runs actions on matches. It's built into every Unix/Linux system and is great for log parsing, data extraction, and quick text transformations.

```bash
awk '$2 > 70 { print $1 }' data.txt   # print names where score > 70
```

---

## Why We Built an AWK Interpreter

AWK is small enough to implement fully, yet complex enough to teach you how real languages work — lexing, parsing, AST design, and evaluation. Building it from scratch gives you a deep understanding of language internals.

---

## Architecture

The interpreter follows a clean pipeline:

```
input → Lexer → tokens → Parser → AST → Evaluator → output
```

- **Lexer** — reads source text and produces tokens
- **Parser** — turns tokens into an AST (Abstract Syntax Tree)
- **AST** — a tree of nodes representing the program structure
- **Evaluator** — walks the AST and executes each node
- **Value** — dual-type (string + number) with automatic coercion, matching AWK's exact semantics

---

## Languages Used

We built the interpreter in three languages:

- **Rust** — the primary implementation. Uses recursive enums for the AST and `match` expressions for evaluation.
- **Go** — a careful port using Go's own idioms, not a line-by-line translation.
- **Crystal** — a Ruby-like language with static typing. Clean and expressive for this kind of tree-walk interpreter.

All three produce identical output for the same AWK program.

**AWK itself** doesn't require array declaration or initialisation — you just use `arr[key]` and it exists. Implementing this meant all three languages had to replicate that behaviour using their own map/hashmap types.

**Go has no enums.** In Rust, the entire AST is built from recursive enums — every node is a variant. In Go, there's no native enum type, so we used constants (`iota`) and interface types to represent AST nodes. It works, but it's more verbose and loses the exhaustiveness checking that Rust's `match` gives you for free.

**Comparing all three:**

| | Rust | Crystal | Go |
|---|---|---|---|
| AST representation | Recursive enums | Recursive enums (like Rust) | Interfaces + constants |
| Pattern matching | `match` — exhaustive | `case` — exhaustive | `switch` — not exhaustive |
| Error handling | `Result<T, E>` | Exceptions | Multiple return values |
| Arrays/maps | `HashMap` | `Hash` | `map[string]string` |
| Type system | Static, strict | Static, expressive | Static, simple |
| Feel | Verbose but safe | Closest to writing AWK itself | Simple and explicit |

Rust gave the most safety guarantees. Go was the simplest to read. Crystal was the most expressive — its syntax felt closest to the AWK language we were implementing.

---

## How specs.md Helped Us

Before writing any code, we created `specs.md` — a feature specification document that listed every feature to build, with pseudocode for each one. This gave us:

- A clear roadmap (what to build next)
- Language-agnostic pseudocode we could translate into Rust, Go, or Crystal
- An implementation status table to track progress across all three languages

It kept all three ports in sync and made it easy to pick up where we left off.

---

## Writing the Manual

Once the interpreter was working, we wrote a full user manual (`MANUAL.md`) covering how to run the interpreter, program structure, special patterns, fields, variables, operators, control flow, arrays, built-in functions, and worked examples.

Writing the manual forced us to test every feature and think from a user's perspective — it caught several edge cases we hadn't considered.

---

## What We Learned: Man Pages, troff, and make install

**What is a man page?**
A man page is the built-in documentation system on Unix/Linux. When you run `man awk` or `man ls`, the terminal displays a formatted reference page. It's offline and always available in the terminal — the standard way to document CLI tools on Unix.

**What is a `.1` file?**
Man pages are stored as files with numbered extensions. `.1` means it's a user command. Our file is `awk_interpreter.1` and it lives at `/usr/local/share/man/man1/` after installation. Running `man awk_interpreter` reads it from there.

**What is troff?**
`.1` files are written in **troff** — a typesetting markup language from the 1970s. The `man` command uses **groff** (GNU troff) to render it in the terminal. It uses macros like `.TH` (title header), `.SH` (section heading), `.TP` (option definition), `.B` (bold), `.I` (italic).

**Why `make install`?**
After writing the man page, we needed to place it in the correct system directory so `man awk_interpreter` could find it. `make install` automated copying the `.1` file to `/usr/local/share/man/man1/` and running `mandb` to update the man page index. That's the standard way CLI tools ship their documentation on Linux — binary goes to `/usr/local/bin`, man page goes to `/usr/local/share/man/man1`.
