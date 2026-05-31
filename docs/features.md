# AWK Interpreter — Feature Roadmap

Build an awk interpreter. Start with basic commands and incrementally add features.

## Implemented

1. **print** — basic `{ print }` and `{ print expr, ... }`
2. **Fields** — `$0`, `$n`, `$NF`, `$NR`, `NF`, `NR`, `FNR`, `FILENAME`, computed field `$(expr)`
3. **BEGIN/END blocks + regex patterns** — `BEGIN`/`END`/`BEGINFILE`/`ENDFILE`, `/pat/` patterns, `~`/`!~` operators, range patterns, ternary, parentheses, `!`
4. **Comparisons** — `==`, `!=`, `<`, `<=`, `>`, `>=` (as expression patterns), `&&`, `||`
5. **Variables and arithmetic** — `+`, `-`, `*`, `/`, `%`, assignment, `++`/`--`, compound assignment (`+=`, `-=`, etc.), unary minus/plus
6. **Field separator** — `FS` / `-F` flag, empty FS (char split)
7. **`if/else` statements** — conditionals with comparisons, nested if/else, chained else-if
8. **Loops** — `for`, `while`, `do-while`, `break`, `continue`, `next`, `exit`
9. **Built-in string functions** — `length`, `substr`, `toupper`/`uc`, `tolower`/`lc`, `index`, `split`, `gsub`, `sub`, `match`, `sprintf`, `gensub`
10. **CLI flags** — `-f` (script file), `-v` (set variable)
11. **String concatenation** — adjacent expressions
12. **Comments** — `#` line comments
13. **Arrays (associative)** — `split()` into arrays, indexed access `arr[i]`
14. **`printf` / `sprintf`** — formatted output via `sprintf()` function
