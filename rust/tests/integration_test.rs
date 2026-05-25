use awk_interpreter::ast::{Expr, Node, Stmt};
use awk_interpreter::builtins::Env;
use awk_interpreter::interp::Interp;
use awk_interpreter::lexer::{Lexer, Token};
use awk_interpreter::parser::Parser;
use awk_interpreter::value::Value;

// ===================== LEXER TESTS =====================

#[test]
fn lexer_empty_input() {
    let mut l = Lexer::new("");
    assert_eq!(l.next_token(), Token::EOF);
}

#[test]
fn lexer_whitespace_only() {
    let mut l = Lexer::new("   \t\n  \r  ");
    assert_eq!(l.next_token(), Token::EOF);
}

#[test]
fn lexer_single_char_tokens() {
    let mut l = Lexer::new("{}()$,;");
    assert_eq!(l.next_token(), Token::LBrace);
    assert_eq!(l.next_token(), Token::RBrace);
    assert_eq!(l.next_token(), Token::LParen);
    assert_eq!(l.next_token(), Token::RParen);
    assert_eq!(l.next_token(), Token::Dollar);
    assert_eq!(l.next_token(), Token::Comma);
    assert_eq!(l.next_token(), Token::Semicolon);
    assert_eq!(l.next_token(), Token::EOF);
}

#[test]
fn lexer_print_keyword() {
    let mut l = Lexer::new("print");
    assert_eq!(l.next_token(), Token::Print);
    assert_eq!(l.next_token(), Token::EOF);
}

#[test]
fn lexer_identifier() {
    let mut l = Lexer::new("myvar");
    assert_eq!(l.next_token(), Token::Ident("myvar".to_string()));
    assert_eq!(l.next_token(), Token::EOF);
}

#[test]
fn lexer_nf_nr_identifiers() {
    let mut l = Lexer::new("NF NR");
    assert_eq!(l.next_token(), Token::Ident("NF".to_string()));
    assert_eq!(l.next_token(), Token::Ident("NR".to_string()));
    assert_eq!(l.next_token(), Token::EOF);
}

#[test]
fn lexer_identifier_with_underscore() {
    let mut l = Lexer::new("_my_var_123");
    assert_eq!(l.next_token(), Token::Ident("_my_var_123".to_string()));
    assert_eq!(l.next_token(), Token::EOF);
}

#[test]
fn lexer_string_literal() {
    let mut l = Lexer::new("\"hello world\"");
    assert_eq!(l.next_token(), Token::String("hello world".to_string()));
    assert_eq!(l.next_token(), Token::EOF);
}

#[test]
fn lexer_empty_string() {
    let mut l = Lexer::new("\"\"");
    assert_eq!(l.next_token(), Token::String("".to_string()));
    assert_eq!(l.next_token(), Token::EOF);
}

#[test]
fn lexer_string_with_numbers() {
    let mut l = Lexer::new("\"abc123\"");
    assert_eq!(l.next_token(), Token::String("abc123".to_string()));
    assert_eq!(l.next_token(), Token::EOF);
}

#[test]
fn lexer_number_integer() {
    let mut l = Lexer::new("42");
    assert_eq!(l.next_token(), Token::Number(42.0));
    assert_eq!(l.next_token(), Token::EOF);
}

#[test]
fn lexer_number_float() {
    let mut l = Lexer::new("3.14");
    assert_eq!(l.next_token(), Token::Number(3.14));
    assert_eq!(l.next_token(), Token::EOF);
}

#[test]
fn lexer_number_negative() {
    let mut l = Lexer::new("-5");
    assert_eq!(l.next_token(), Token::Number(-5.0));
    assert_eq!(l.next_token(), Token::EOF);
}

#[test]
fn lexer_number_negative_float() {
    let mut l = Lexer::new("-3.14");
    assert_eq!(l.next_token(), Token::Number(-3.14));
    assert_eq!(l.next_token(), Token::EOF);
}

#[test]
fn lexer_number_zero() {
    let mut l = Lexer::new("0");
    assert_eq!(l.next_token(), Token::Number(0.0));
    assert_eq!(l.next_token(), Token::EOF);
}

#[test]
fn lexer_compound_program() {
    let mut l = Lexer::new("{ print $1, $2 }");
    assert_eq!(l.next_token(), Token::LBrace);
    assert_eq!(l.next_token(), Token::Print);
    assert_eq!(l.next_token(), Token::Dollar);
    assert_eq!(l.next_token(), Token::Number(1.0));
    assert_eq!(l.next_token(), Token::Comma);
    assert_eq!(l.next_token(), Token::Dollar);
    assert_eq!(l.next_token(), Token::Number(2.0));
    assert_eq!(l.next_token(), Token::RBrace);
    assert_eq!(l.next_token(), Token::EOF);
}

#[test]
fn lexer_semicolon_separated() {
    let mut l = Lexer::new("{ print $1; print $2 }");
    assert_eq!(l.next_token(), Token::LBrace);
    assert_eq!(l.next_token(), Token::Print);
    assert_eq!(l.next_token(), Token::Dollar);
    assert_eq!(l.next_token(), Token::Number(1.0));
    assert_eq!(l.next_token(), Token::Semicolon);
    assert_eq!(l.next_token(), Token::Print);
    assert_eq!(l.next_token(), Token::Dollar);
    assert_eq!(l.next_token(), Token::Number(2.0));
    assert_eq!(l.next_token(), Token::RBrace);
    assert_eq!(l.next_token(), Token::EOF);
}

#[test]
fn lexer_field_expr_nf() {
    let mut l = Lexer::new("{ print $NF }");
    assert_eq!(l.next_token(), Token::LBrace);
    assert_eq!(l.next_token(), Token::Print);
    assert_eq!(l.next_token(), Token::Dollar);
    assert_eq!(l.next_token(), Token::Ident("NF".to_string()));
    assert_eq!(l.next_token(), Token::RBrace);
    assert_eq!(l.next_token(), Token::EOF);
}

#[test]
fn lexer_unknown_char_becomes_ident() {
    let mut l = Lexer::new("@");
    assert_eq!(l.next_token(), Token::Ident("@".to_string()));
    assert_eq!(l.next_token(), Token::EOF);
}

#[test]
fn lexer_mixed_tokens_no_whitespace() {
    let mut l = Lexer::new("{print$1,\"hello\"}");
    assert_eq!(l.next_token(), Token::LBrace);
    assert_eq!(l.next_token(), Token::Print);
    assert_eq!(l.next_token(), Token::Dollar);
    assert_eq!(l.next_token(), Token::Number(1.0));
    assert_eq!(l.next_token(), Token::Comma);
    assert_eq!(l.next_token(), Token::String("hello".to_string()));
    assert_eq!(l.next_token(), Token::RBrace);
    assert_eq!(l.next_token(), Token::EOF);
}

// ===================== PARSER TESTS =====================

fn parse(input: &str) -> Node {
    let mut p = Parser::new(Lexer::new(input));
    p.parse()
}

#[test]
fn parser_empty_program() {
    let node = parse("");
    assert_eq!(node, Node::Program(vec![]));
}

#[test]
fn parser_print_no_args() {
    let node = parse("{ print }");
    assert_eq!(node, Node::Program(vec![Stmt::Print(vec![])]));
}

#[test]
fn parser_print_field() {
    let node = parse("{ print $1 }");
    assert_eq!(
        node,
        Node::Program(vec![Stmt::Print(vec![Expr::FieldExpr(Box::new(
            Expr::Num(1.0)
        ))])])
    );
}

#[test]
fn parser_print_two_fields() {
    let node = parse("{ print $1, $2 }");
    assert_eq!(
        node,
        Node::Program(vec![Stmt::Print(vec![
            Expr::FieldExpr(Box::new(Expr::Num(1.0))),
            Expr::FieldExpr(Box::new(Expr::Num(2.0))),
        ])])
    );
}

#[test]
fn parser_print_string() {
    let node = parse(r#"{ print "hello" }"#);
    assert_eq!(
        node,
        Node::Program(vec![Stmt::Print(vec![Expr::Str("hello".to_string())])])
    );
}

#[test]
fn parser_print_number() {
    let node = parse("{ print 42 }");
    assert_eq!(
        node,
        Node::Program(vec![Stmt::Print(vec![Expr::Num(42.0)])])
    );
}

#[test]
fn parser_print_identifier() {
    let node = parse("{ print NR }");
    assert_eq!(
        node,
        Node::Program(vec![Stmt::Print(vec![Expr::Ident("NR".to_string())])])
    );
}

#[test]
fn parser_print_field_expr_ident() {
    let node = parse("{ print $NF }");
    assert_eq!(
        node,
        Node::Program(vec![Stmt::Print(vec![Expr::FieldExpr(Box::new(
            Expr::Ident("NF".to_string())
        ))])])
    );
}

#[test]
fn parser_two_print_statements() {
    let node = parse("{ print $1; print $2 }");
    assert_eq!(
        node,
        Node::Program(vec![
            Stmt::Print(vec![Expr::FieldExpr(Box::new(Expr::Num(1.0)))]),
            Stmt::Print(vec![Expr::FieldExpr(Box::new(Expr::Num(2.0)))]),
        ])
    );
}

#[test]
fn parser_print_string_and_field() {
    let node = parse(r#"{ print "Name:", $1 }"#);
    assert_eq!(
        node,
        Node::Program(vec![Stmt::Print(vec![
            Expr::Str("Name:".to_string()),
            Expr::FieldExpr(Box::new(Expr::Num(1.0))),
        ])])
    );
}

#[test]
fn parser_no_braces() {
    let node = parse("print $1");
    assert_eq!(
        node,
        Node::Program(vec![Stmt::Print(vec![Expr::FieldExpr(Box::new(
            Expr::Num(1.0)
        ))])])
    );
}

// ===================== VALUE TESTS =====================

#[test]
fn value_str_as_str() {
    let v = Value::Str("hello".to_string());
    assert_eq!(v.as_str(), "hello");
}

#[test]
fn value_empty_str_as_str() {
    let v = Value::Str(String::new());
    assert_eq!(v.as_str(), "");
}

#[test]
fn value_num_integer_as_str() {
    let v = Value::Num(42.0);
    assert_eq!(v.as_str(), "42");
}

#[test]
fn value_num_negative_as_str() {
    let v = Value::Num(-5.0);
    assert_eq!(v.as_str(), "-5");
}

#[test]
fn value_num_float_as_str() {
    let v = Value::Num(3.14);
    assert_eq!(v.as_str(), "3.14");
}

#[test]
fn value_num_zero_as_str() {
    let v = Value::Num(0.0);
    assert_eq!(v.as_str(), "0");
}

#[test]
fn value_str_valid_number_as_f64() {
    let v = Value::Str("42".to_string());
    assert_eq!(v.as_f64(), 42.0);
}

#[test]
fn value_str_float_as_f64() {
    let v = Value::Str("3.14".to_string());
    assert_eq!(v.as_f64(), 3.14);
}

#[test]
fn value_str_invalid_as_f64() {
    let v = Value::Str("abc".to_string());
    assert_eq!(v.as_f64(), 0.0);
}

#[test]
fn value_str_empty_as_f64() {
    let v = Value::Str(String::new());
    assert_eq!(v.as_f64(), 0.0);
}

#[test]
fn value_num_as_f64() {
    let v = Value::Num(3.14);
    assert_eq!(v.as_f64(), 3.14);
}

#[test]
fn value_display_str() {
    let v = Value::Str("hello".to_string());
    assert_eq!(format!("{}", v), "hello");
}

#[test]
fn value_display_num_integer() {
    let v = Value::Num(42.0);
    assert_eq!(format!("{}", v), "42");
}

#[test]
fn value_display_num_float() {
    let v = Value::Num(3.14);
    assert_eq!(format!("{}", v), "3.14");
}

#[test]
fn value_display_num_negative() {
    let v = Value::Num(-10.0);
    assert_eq!(format!("{}", v), "-10");
}

// ===================== ENV / BUILTINS TESTS =====================

#[test]
fn env_new_is_empty() {
    let env = Env::new();
    assert!(env.fields.is_empty());
    assert_eq!(env.nr, 0);
}

#[test]
fn env_set_line_splits_by_whitespace() {
    let mut env = Env::new();
    env.set_line("Sam 67", " ");
    assert_eq!(env.nr, 1);
    assert_eq!(env.fields.len(), 3);
    assert_eq!(env.get_field(0).as_str(), "Sam 67");
    assert_eq!(env.get_field(1).as_str(), "Sam");
    assert_eq!(env.get_field(2).as_str(), "67");
}

#[test]
fn env_set_line_custom_separator() {
    let mut env = Env::new();
    env.set_line("a,b,c", ",");
    assert_eq!(env.fields.len(), 4);
    assert_eq!(env.get_field(1).as_str(), "a");
    assert_eq!(env.get_field(2).as_str(), "b");
    assert_eq!(env.get_field(3).as_str(), "c");
}

#[test]
fn env_set_line_custom_sep_no_delim() {
    let mut env = Env::new();
    env.set_line("hello", ",");
    assert_eq!(env.fields.len(), 2);
    assert_eq!(env.get_field(1).as_str(), "hello");
}

#[test]
fn env_get_field_out_of_bounds_returns_empty() {
    let mut env = Env::new();
    env.set_line("hello", " ");
    assert_eq!(env.get_field(99).as_str(), "");
}

#[test]
fn env_get_field_zero_empty_line() {
    let mut env = Env::new();
    env.set_line("", " ");
    assert_eq!(env.fields.len(), 1);
    assert_eq!(env.get_field(0).as_str(), "");
}

#[test]
fn env_nr_increments() {
    let mut env = Env::new();
    assert_eq!(env.nr, 0);
    env.set_line("a", " ");
    assert_eq!(env.nr, 1);
    env.set_line("b", " ");
    assert_eq!(env.nr, 2);
    env.set_line("c", " ");
    assert_eq!(env.nr, 3);
}

#[test]
fn env_nf_var() {
    let mut env = Env::new();
    env.set_line("a b c d", " ");
    assert!((env.get_var("NF").as_f64() - 4.0).abs() < f64::EPSILON);
}

#[test]
fn env_nf_single_field() {
    let mut env = Env::new();
    env.set_line("hello", " ");
    assert!((env.get_var("NF").as_f64() - 1.0).abs() < f64::EPSILON);
}

#[test]
fn env_nr_var() {
    let mut env = Env::new();
    env.set_line("first", " ");
    assert!((env.get_var("NR").as_f64() - 1.0).abs() < f64::EPSILON);
    env.set_line("second", " ");
    assert!((env.get_var("NR").as_f64() - 2.0).abs() < f64::EPSILON);
}

#[test]
fn env_undefined_var_defaults_to_zero() {
    let env = Env::new();
    let v = env.get_var("undefined_var");
    assert_eq!(v, Value::Num(0.0));
}

// ===================== INTERPRETER TESTS =====================

fn exec(program: &str, line: &str) -> String {
    let mut interp = Interp::new();
    let mut p = Parser::new(Lexer::new(program));
    let node = p.parse();
    interp.exec(&node, line)
}

#[test]
fn interp_print_empty_exprs_prints_full_line() {
    let result = exec("{ print }", "hello world");
    assert_eq!(result, "hello world\n");
}

#[test]
fn interp_print_field1() {
    let result = exec("{ print $1 }", "Sam 67");
    assert_eq!(result, "Sam\n");
}

#[test]
fn interp_print_field2() {
    let result = exec("{ print $2 }", "Sam 67");
    assert_eq!(result, "67\n");
}

#[test]
fn interp_print_two_fields() {
    let result = exec("{ print $1, $2 }", "Sam 67");
    assert_eq!(result, "Sam 67\n");
}

#[test]
fn interp_print_string_literal() {
    let result = exec(r#"{ print "hello" }"#, "ignored");
    assert_eq!(result, "hello\n");
}

#[test]
fn interp_print_number_literal() {
    let result = exec("{ print 42 }", "");
    assert_eq!(result, "42\n");
}

#[test]
fn interp_print_nr() {
    let result = exec("{ print NR }", "first");
    assert_eq!(result, "1\n");
}

#[test]
fn interp_print_nf() {
    let result = exec("{ print NF }", "a b c");
    assert_eq!(result, "3\n");
}

#[test]
fn interp_print_last_field_via_nf() {
    let result = exec("{ print $NF }", "a b c d e");
    assert_eq!(result, "e\n");
}

#[test]
fn interp_print_string_then_field() {
    let result = exec(r#"{ print "Name:", $1 }"#, "Alice");
    assert_eq!(result, "Name: Alice\n");
}

#[test]
fn interp_multiple_statements() {
    let result = exec("{ print $1; print $2 }", "hello world");
    assert_eq!(result, "hello\nworld\n");
}

#[test]
fn interp_nr_increments_across_calls() {
    let program = "{ print NR, $1 }";
    let mut interp = Interp::new();
    let mut p = Parser::new(Lexer::new(program));
    let node = p.parse();

    assert_eq!(interp.exec(&node, "alpha"), "1 alpha\n");
    assert_eq!(interp.exec(&node, "beta"), "2 beta\n");
    assert_eq!(interp.exec(&node, "gamma"), "3 gamma\n");
}

#[test]
fn interp_fields_update_each_line() {
    let program = "{ print $1 }";
    let mut interp = Interp::new();
    let mut p = Parser::new(Lexer::new(program));
    let node = p.parse();

    assert_eq!(interp.exec(&node, "first_line"), "first_line\n");
    assert_eq!(interp.exec(&node, "second_line"), "second_line\n");
}

#[test]
fn interp_multiple_fields_with_nf() {
    let result = exec("{ print $1, $NF }", "apple banana cherry");
    assert_eq!(result, "apple cherry\n");
}

#[test]
fn interp_field_out_of_range() {
    let result = exec("{ print $99 }", "a b");
    assert_eq!(result, "\n");
}

#[test]
fn interp_three_fields_all() {
    let result = exec("{ print $1, $2, $3 }", "a b c");
    assert_eq!(result, "a b c\n");
}

#[test]
fn interp_many_fields() {
    let result = exec("{ print $1, $2, $3, $4, $5 }", "1 2 3 4 5");
    assert_eq!(result, "1 2 3 4 5\n");
}

#[test]
fn interp_field_expr_direct_construct() {
    let mut interp = Interp::new();
    let node = Node::Program(vec![Stmt::Print(vec![Expr::FieldExpr(Box::new(
        Expr::Num(3.0),
    ))])]);
    let result = interp.exec(&node, "a b c");
    assert_eq!(result, "c\n");
}

#[test]
fn interp_empty_line() {
    let result = exec("{ print NR, $1 }", "");
    assert_eq!(result, "1 \n");
}
