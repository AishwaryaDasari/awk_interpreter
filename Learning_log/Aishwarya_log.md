# Aishwarya's Learning Log — AWK Interpreter

## What is AWK?

AWK is a scripting language commonly used in Linux and Unix systems for text processing and data manipulation. It processes input line by line, matches patterns, and performs actions based on those patterns. AWK is widely used for filtering data, generating reports, and processing structured text files such as CSV files.

## Why We Built an AWK Interpreter

The AWK Interpreter project was chosen because it provides a practical way to understand how programming languages work internally. Although AWK is a relatively small language, it contains many important concepts found in larger languages.

Building the interpreter from scratch helped me understand language processing, parsing, execution flow rather than simply using an existing tool.

## Interpreter Components

### 1. Lexer and Tokenization

The lexer reads the source code and converts it into tokens.

**Example:**
```
print $1
```
becomes:
```
print | $1
```

Tokens are small meaningful units that make the input easier to process.

### 2. Field Splitting

Field splitting divides input data into separate fields for processing.

**Example:**
```
John 21 CSE
```
becomes:
```
$1 = John
$2 = 21
$3 = CSE
```

### 3. Pattern-Action Matching

The interpreter checks whether a pattern is satisfied and then performs the corresponding action.

**Example:**
```
$2 > 20 { print $1 }
```
If the second field is greater than 20, the first field is printed.

### 4. Input and Output Handling

This component manages reading input data and producing the final output.

## Testing and Validation

Testing played a major role in this project.

### CSV Files

I learned about CSV files, their structure, and their applications in testing and data processing.

**Example:**
```
Name,Marks
Alice,85
Bob,70
```

### Output Validation

To verify correctness:
1. CSV test files were created.
2. AWK commands were executed using our interpreter.
3. The same commands were executed using the original AWK interpreter.
4. Outputs from both implementations were compared.

The outputs matched, which confirmed that the interpreter was functioning correctly.

This process helped me understand testing, validation, and debugging techniques.

## Technical Documentation

One of the important lessons from this project was understanding the value of technical documentation.

I learned that good documentation:
- Clearly explains project requirements
- Improves communication among team members
- Helps developers and LLMs understand project goals
- Reduces development errors

## Git and Version Control

Throughout the project, I worked with Git and became familiar with version control concepts.

Some concepts I learned include:
- Repositories
- Commits
- Push and Pull operations
- Branches
- Collaboration using Git

## Key Takeaways

- Building an interpreter provides deep insight into how programming languages work.
- Testing is essential for ensuring software correctness.
- Technical documentation is an important part of software development.
- Learning by building helps develop a stronger understanding than simply using existing tools.
- Teamwork and collaboration are crucial for successful project development.

## Conclusion

The AWK Interpreter project provided practical exposure to language processing, testing methodologies, technical documentation, version control, and collaborative software development. It strengthened my problem-solving skills, technical knowledge, and ability to learn and adapt to new technologies.
