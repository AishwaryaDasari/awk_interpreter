use awk_interpreter::interp::Interp;
use awk_interpreter::lexer::Lexer;
use awk_interpreter::parser::Parser;

fn run_awk(program: &str, input: &str) -> String {
    let lexer = Lexer::new(program);
    let mut parser = Parser::new(lexer);
    let node = parser.parse();

    let mut interp = Interp::new();
    let mut output = String::new();

    for line in input.lines() {
        let out = interp.exec(&node, line);
        output.push_str(&out);
    }

    output
}

fn load_test_input() -> String {
    std::fs::read_to_string("../tests/test_input.txt")
        .expect("failed to read tests/test_input.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prints_every_line() {
        let result = run_awk("{ print }", "hello\nworld");
        assert_eq!(result, "hello\nworld\n");
    }

    #[test]
    fn prints_first_field() {
        let result = run_awk("{ print $1 }", "a b c\nd e f");
        assert_eq!(result, "a\nd\n");
    }

    #[test]
    fn prints_second_field() {
        let result = run_awk("{ print $2 }", "a b c\nd e f");
        assert_eq!(result, "b\ne\n");
    }

    #[test]
    fn prints_nr_and_first_field() {
        let result = run_awk("{ print NR, $1 }", "alpha\nbeta\ngamma");
        assert_eq!(result, "1 alpha\n2 beta\n3 gamma\n");
    }

    #[test]
    fn prints_nr_and_fields_from_test_input() {
        let input = load_test_input();
        let first_line = input.lines().next().unwrap();
        let fields: Vec<&str> = first_line.split_whitespace().collect();
        let eid = fields[0];
        let expected = format!("1 {eid}\n");

        let result = run_awk("{ print NR, $1 }", &input.lines().take(1).collect::<Vec<_>>().join("\n"));
        assert_eq!(result, expected);
    }

    #[test]
    fn processes_all_100_records() {
        let input = load_test_input();
        let result = run_awk("{ print NR }", &input);
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines.len(), 100);
        assert_eq!(lines[0], "1");
        assert_eq!(lines[99], "100");
    }

    #[test]
    fn test_field_1() {
        let input = load_test_input();
        let result = run_awk("{ print $1 }", &input);
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines.len(), 100);
        assert_eq!(lines[0], "E001");
        assert_eq!(lines[99], "E100");
    }

    #[test]
    fn test_field_last() {
        let input = load_test_input();
        let result = run_awk("{ print $NF }", &input);
        for line in result.lines() {
            let years: i32 = line.parse().expect("expected a number");
            assert!(
                (1..=30).contains(&years),
                "years of experience {years} out of range"
            );
        }
    }

    #[test]
    fn test_field_combination() {
        let input = load_test_input();
        let first_line = input.lines().next().unwrap();
        let fields: Vec<&str> = first_line.split_whitespace().collect();
        let expected = format!("{} {} {}\n", fields[0], fields[1], fields[2]);

        let result = run_awk("{ print $1, $2, $3 }", first_line);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_dollar_zero() {
        let input = load_test_input();
        let result = run_awk("{ print $0 }", &input);
        assert_eq!(result, input);
    }
}
