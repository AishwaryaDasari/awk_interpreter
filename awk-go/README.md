# AWK Interpreter — Go

This is the Go implementation of the AWK interpreter.
Same behaviour as the Rust version, written in Go idioms.

---

## 1. Install Go

### Windows

1. Go to https://go.dev/dl/
2. Download the Windows installer (`.msi` file) — pick the latest stable version
3. Run the installer, it sets up everything including PATH automatically
4. Open a new terminal and verify:

```
go version
```

You should see something like `go version go1.22.0 windows/amd64`.

### macOS

Using Homebrew:
```
brew install go
```

Or download the `.pkg` installer from https://go.dev/dl/ and run it.

Verify:
```
go version
```

### Linux

```
sudo apt install golang-go        # Debian / Ubuntu
sudo dnf install golang           # Fedora
sudo pacman -S go                 # Arch
```

Or download the tarball from https://go.dev/dl/ and follow the manual install steps there.

Verify:
```
go version
```

---

## 2. Project Structure

```
awk-go/
├── go.mod          — module definition (module name: awk-go)
├── main.go         — CLI entry point
├── lexer.go        — tokeniser
├── parser.go       — builds AST from tokens
├── interpreter.go  — walks AST and executes against input
└── tests/
    └── awk_test.go — test suite
```

The module name is `awk-go` (defined in `go.mod`). All packages import each other using that prefix:
```go
import "awk-go/lexer"
import "awk-go/parser"
import "awk-go/interpreter"
```

---

## 3. Build the Interpreter

Navigate to the `awk-go` folder:

```
cd awk-go
```

Build the binary:

```
go build -o awk_go .
```

This produces an executable called `awk_go` (or `awk_go.exe` on Windows) in the same folder.

---

## 4. Run the Interpreter

```
awk_go '<program>' <input_file>
```

**Examples using the test file:**

```
# Print every line
awk_go '{print}' ../tests/test1.txt

# Print first field only
awk_go '{print $1}' ../tests/test1.txt

# Print first and second fields
awk_go '{print $1, $2}' ../tests/test1.txt

# Print line number + first field
awk_go '{print NR, $1}' ../tests/test1.txt

# Print last field
awk_go '{print $NF}' ../tests/test1.txt

# Print entire line ($0)
awk_go '{print $0}' ../tests/test1.txt
```

If you haven't built the binary yet, you can also run directly with `go run`:

```
go run . '{print $1}' ../tests/test1.txt
```

---

## 5. Run the Tests

The test suite lives in `tests/awk_test.go`. It requires a test input file at `tests/../../tests/test_input.txt` (i.e., `tests/test_input.txt` one level up from `awk-go`).

**Create the test input file first** (100 lines of employee data):

```
tests/test_input.txt
```

Format expected by the tests:
```
E001 Alice Engineering 95000 5
E002 Bob Marketing 72000 3
...
E100 Zara Finance 88000 12
```

Fields: `EmployeeID Name Department Salary YearsOfExperience`

Once the file exists, run all tests from inside `awk-go`:

```
go test ./tests/
```

Run with verbose output to see each test name:

```
go test -v ./tests/
```

Run a single specific test:

```
go test -v ./tests/ -run TestPrintFirstField
```

---

## 6. What the Tests Cover

| Test | What it checks |
|------|---------------|
| `TestPrintEveryLine` | `{print}` prints every line |
| `TestPrintFirstField` | `{print $1}` prints first field |
| `TestPrintSecondField` | `{print $2}` prints second field |
| `TestNRAndFirstField` | `{print NR, $1}` prints line number + field |
| `TestNRAndFirstFieldFromFile` | Same but reads from test_input.txt |
| `TestNRAllRecords` | NR goes from 1 to 100 across 100 lines |
| `TestFieldOne` | `$1` returns E001 through E100 |
| `TestFieldLast` | `$NF` returns years of experience (1–30) |
| `TestFieldCombination` | `{print $1, $2, $3}` prints three fields |
| `TestDollarZero` | `{print $0}` output matches input exactly |

---

## 7. Common Issues

**`go: cannot find module`**
Make sure you are inside the `awk-go` directory when running `go build` or `go test`.

**`package awk-go/lexer is not in GOROOT`**
Same fix — run commands from inside `awk-go/`, not from the repo root.

**`failed to read test input`**
The test file `tests/test_input.txt` (relative to the repo root) doesn't exist yet. Create it with 100 lines in the format shown above.

**Windows: `awk_go` not found after build**
The binary is `awk_go.exe` on Windows. Run it as `.\awk_go.exe` or add the folder to PATH.

---

## 8. Difference from the Rust Version

The Go version uses the same pipeline (Lexer → Parser → Interpreter) but with Go idioms:

| Concept | Rust | Go |
|---------|------|----|
| AST nodes | `enum` with variants | `interface` with type switch |
| Error handling | `Result<T, E>` + `?` | `value, err` + `if err != nil` |
| Heap allocation | `Box<T>` explicit | implicit via interfaces |
| Pattern matching | `match` (exhaustive) | `switch` (not exhaustive) |

Both versions produce identical output for the same input.
