# AWK Interpreter

## Introduction

This document provides a complete overview of building a simplified AWK 
interpreter. It explains the architecture, core components, execution flow, 
implementation stages, and recommended technologies.

The project is designed as a systems/programming-language engineering project 
that teaches:
* Lexing
* Parsing
* AST construction
* Interpreters
* Runtime systems
* Text processing
* Regex handling
* Compiler fundamentals

---

# What is AWK?

AWK is a text-processing and pattern-scanning programming language commonly 
used in Unix/Linux systems.

It processes input line-by-line and applies pattern-action rules.

AWK is heavily used for:
* Log processing
* Data extraction
* Text transformation
* CSV processing
* Shell scripting
* DevOps automation
* System administration

---

# Project Goal

The goal of this project is to build a simplified AWK interpreter capable of:

1. Reading AWK source code
2. Tokenizing the source
3. Parsing the source
4. Building an AST
5. Processing text input line-by-line
6. Executing pattern-action rules
7. Producing transformed output

---

# High-Level Architecture

```text
            AWK Source
                                ↓
              Lexer
                                ↓
              Tokens
                                ↓
              Parser
                                ↓
                AST
                                ↓
           Interpreter Runtime
                                ↓
          Input File Processing
                                ↓
               Output
```

---

# Core Concepts

## 1. Lexer

A lexer (tokenizer) converts raw source code into tokens.

Example:

```awk
{ print $1 + 5 }
```

Lexer output:

```text
LBRACE
PRINT
FIELD(1)
PLUS
NUMBER(5)
RBRACE
```

### Responsibilities

* Read characters
* Group characters into tokens
* Ignore whitespace/comments
* Recognize operators and symbols
* Detect keywords and identifiers

### Common Tokens

| Token  | Meaning        |
| ------ | -------------- |
| PRINT  | print keyword  |
| NUMBER | numeric value  |
| STRING | string literal |
| PLUS   | + operator     |
| LBRACE | {              |
| RBRACE | }              |
| FIELD  | $1, $2, etc    |

---

# 2. Parser

The parser takes tokens and validates syntax according to grammar rules.

It converts tokens into structured program representation.

Example:

Tokens:

```text
NUMBER(1)
PLUS
NUMBER(2)
STAR
NUMBER(3)
```

Parser understands precedence:

```text
1 + (2 * 3)
```

not:

```text
(1 + 2) * 3
```

### Responsibilities

* Validate syntax
* Handle precedence rules
* Build AST nodes
* Detect syntax errors

---

# 3. AST (Abstract Syntax Tree)

An AST is a tree representation of the program.

Example:

```text
      +
     / \
    1   *
       / \
      2   3
```

The AST represents:

```text
1 + (2 * 3)
```

### Why AST Exists

ASTs make programs easier to:

* Execute
* Analyze
* Optimize
* Compile

### AWK AST Example

Source:

```awk
{ print $1 }
```

AST:

```text
Program
 └── Action
      └── Print
           └── Field(1)
```

---

# AWK Execution Model

AWK is line-oriented and stream-based.

Each input line becomes the current record.

Example runtime state:

```text
$0 = "hello world"
$1 = "hello"
$2 = "world"
NF = 2
NR = 1
```

Where:

| Variable | Meaning             |
| -------- | ------------------- |
| $0       | Entire line         |
| $1       | First field         |
| $2       | Second field        |
| NF       | Number of fields    |
| NR       | Current line number |

---

# Pattern-Action Model

The core AWK model:

```awk
pattern { action }
```

Example:

```awk
/error/ { print $0 }
```

Meaning:

* If line matches regex `/error/`
* Execute action block

---

# Recommended Language Choices

## Go

Best for:

* Fast implementation
* CLI tools
* Easy development
* Simpler debugging

Advantages:

* Clean syntax
* Excellent standard library
* Easy file streaming
* Good regex support
* Fast iteration

Recommended for first implementation.

---

## Rust

Best for:

* Systems programming
* High-performance interpreters
* Compiler engineering
* Advanced language tooling

Advantages:

* Memory safety
* Pattern matching
* Powerful enums
* Excellent performance

Challenges:

* Ownership model
* Borrow checker complexity

---

## Elixir

Best for:

* Streaming pipelines
* Concurrent runtimes
* Functional experimentation

Advantages:

* Functional architecture
* Excellent concurrency
* Pipeline-based processing

Challenges:

* Fewer compiler/interpreter tutorials
* Less traditional systems programming

# Project Structure

Suggested Go project structure:

```text
awk/
├── cmd/
├── lexer/
├── parser/
├── ast/
├── evaluator/
├── runtime/
├── token/
├── object/
└── main.go
```

# Development Roadmap

## Phase 1 — Lexer

Implement tokenization.

Features:

* Keywords
* Numbers
* Strings
* Operators
* Braces
* Variables

---

## Phase 2 — Parser

Build recursive descent parser.

Features:

* Expressions
* Statements
* Precedence handling
* Syntax validation

---

## Phase 3 — AST

Define AST node structures.

Example:

```text
PrintStmt
 └── FieldExpr(1)
```

---

## Phase 4 — Runtime Loop

Implement:

```text
for each line:
    split fields
    execute AST
```

---

## Phase 5 — Print Statements

Support:

```awk
{ print $1 }
```

---

## Phase 6 — Expressions

Support:

```awk
{ print $1 + 5 }
```

Features:

* Addition
* Subtraction
* Multiplication
* Division

---

## Phase 7 — Pattern Matching

Support regex patterns:

```awk
/error/ { print }
```

---

## Phase 8 — Variables

Support:

```awk
{ sum += $1 }
END { print sum }
```

---

## Phase 9 — Associative Arrays

Support:

```awk
{ count[$1]++ }
```

AWK arrays behave like hash maps.

---

## Phase 10 — BEGIN and END

Support:

```awk
BEGIN { print "start" }
END { print "done" }
```

# Dynamic Typing

AWK automatically converts values.

Example:

```awk
"10" + 5
```

becomes:

```text
15
```

Your interpreter must support automatic coercion.

---

# Associative Arrays

AWK arrays are string-keyed hash maps.

Example:

```awk
count["foo"]++
```

Implementation idea:

```go
map[string]Value
```

---

# Runtime Flow Example

Source:

```awk
{ print $1 }
```

Input:

```text
hello world
```

Runtime state:

```text
$0 = "hello world"
$1 = "hello"
$2 = "world"
NF = 2
```

Execution:

```text
print($1)
```

Output:

```text
hello
```

---

# Key Technical Challenges

## 1. Operator Precedence

Need correct parsing of:

```text
1 + 2 * 3
```

---

## 2. Dynamic Typing

Automatic conversion between:

* strings
* numbers
* booleans

---

## 3. Regex Matching

Support pattern matching:

```awk
/error/
```

---

## 4. Implicit Variables

Automatically maintain:

* $0
* $1
* NF
* NR

---

# Learning Outcomes

This project teaches:

* Interpreter design
* Compiler fundamentals
* Parsing theory
* Runtime systems
* Regex engines
* Language semantics
* Unix philosophy
* Text processing systems



