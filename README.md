## AWK INTERPRETER ##

About awk -> mawk  is  an  interpreter for the AWK Programming Language.  The AWK language is useful for manipulation of data files, text retrieval and processing,
       and for prototyping and experimenting with algorithms.  mawk is a new awk meaning it implements the AWK language as  defined  in  Aho,  Kernighan  and
       Weinberger,  The AWK Programming Language, Addison-Wesley Publishing, 1988 (hereafter referred to as the AWK book.)  mawk conforms to the POSIX 1003.2
       (draft 11.3) definition of the AWK language which contains a few features not described in the AWK book, and mawk provides a small  number  of  exten‐
       sions.

       An AWK program is a sequence of pattern {action} pairs and function definitions.  Short programs are entered on the command line usually enclosed in '
       '  to  avoid  shell interpretation.  Longer programs can be read in from a file with the -f option.  Data  input is read from the list of files on the
       command line or from standard input when the list is empty.  The input is broken into records as determined by  the  record  separator  variable,  RS.
       Initially, RS = “\n” and records are synonymous with lines.  Each record is compared against each pattern and if it matches, the program text for {ac‐
       tion} is executed.


## What we are going to build ##

We are building a working AWK interpreter from scratch — one that can run real AWK programs from the terminal, the same way the system awk command does.

The interpreter is built in Rust as the primary language, then ported to Go and Crystal. All three versions run the same programs and produce identical output — the ports are not translations done in a hurry, but careful reimplementations that use each language's own idioms.
The project is purely a backend CLI tool. No web UI, no frontend — just a binary you run in the terminal.

## what we shall be learning ##

Enums and pattern matching — the entire AST is a tree of Rust enums. Every node, every expression, every statement is a variant. match expressions over these enums are how the evaluator works.

Ownership and borrowing — the interpreter holds mutable state (variables, fields, the current line) while also traversing immutable AST nodes. Getting this to compile without cloning everything teaches you how Rust's borrow checker actually thinks.

Error handling — a real interpreter needs to report errors with line numbers and context. Using Result<T, E> and the ? operator throughout the codebase.

Traits and generics — the Value type (every AWK value is simultaneously a string and a number) needs Display, From, PartialOrd, and arithmetic operator trait.

