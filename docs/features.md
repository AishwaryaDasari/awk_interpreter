# AWK Interpreter — Feature Roadmap

Build an awk interpreter. Start with basic commands and incrementally add features.

## Implemented

1. **print** — basic `{ print }` and `{ print expr, ... }`
2. **Fields** — `$0`, `$n`, `$NF`, `$NR`, `NF`, `NR`

## Planned (in order)

3. BEGIN/END blocks + regex patterns
4. Variables and arithmetic
5. Field separator (FS / `-F`)
6. `if/else` + comparisons
7. Loops (`for`, `while`)
8. Built-in string functions
9. Arrays
10. `printf`
