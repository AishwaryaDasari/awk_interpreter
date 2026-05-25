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



