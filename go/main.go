package main

import (
	"fmt"
	"os"

	"awk-go/interpreter"
	"awk-go/lexer"
	"awk-go/parser"
)

func main() {
	if len(os.Args) < 2 {
		fmt.Fprintf(os.Stderr, "usage: %s '<awk program>' [<input file>]\n", os.Args[0])
		os.Exit(1)
	}

	program := os.Args[1]
	var input string

	if len(os.Args) >= 3 {
		filename := os.Args[2]
		content, err := os.ReadFile(filename)
		if err != nil {
			fmt.Fprintf(os.Stderr, "error reading %s: %v\n", filename, err)
			os.Exit(1)
		}
		input = string(content)
	}

	tokens := lexer.New(program).Tokenize()
	prog, err := parser.New(tokens).Parse()
	if err != nil {
		fmt.Fprintf(os.Stderr, "parse error: %v\n", err)
		os.Exit(1)
	}

	output := interpreter.New().Run(prog, input)
	fmt.Print(output)
}
