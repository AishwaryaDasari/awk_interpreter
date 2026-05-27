# AWK Interpreter — Feature Roadmap

Build an awk interpreter. Start with basic commands and incrementally add features.

## Implemented

1. **print** — basic `{ print }` and `{ print expr, ... }`
2. **Fields** — `$0`, `$n`, `$NF`, `$NR`, `NF`, `NR`
3. **BEGIN/END blocks + regex patterns** — `BEGIN`/`END`/`BEGINFILE`/`ENDFILE`, `/pat/` patterns, `~`/`!~` operators, range patterns, ternary, parentheses, `!`
4. **Comparisons** — `==`, `!=`, `<`, `<=`, `>`, `>=` (as expression patterns), `&&`, `||`
5. **Variables and arithmetic** — `+`, `-`, `*`, `/`, `%`, assignment, `++`/`--`
6. **Field separator** — `FS` / `-F` flag
7. **`if/else` statements** — conditionals with comparisons, nested if/else, chained else-if
8. **Loops** — `for`, `while`, `do-while`, `break`, `continue`, `next`, `exit`

## Planned (in order)

9. Built-in string functions (`length`, `substr`, `gsub`, `split`, etc.)
10. Arrays (associative) and `for-in`
11. `printf`
