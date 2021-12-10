use std::fs;
use std::collections::HashMap;

type CodeLine = Vec<char>;
type Code = Vec<CodeLine>;

fn parse_input(input: &str) -> Code {
    input.split_whitespace().map(|line| line.chars().collect()).collect()
}

fn find_missing_chars(line: &CodeLine) -> CodeLine {
    let mut stack = CodeLine::new();
    let closing: HashMap<char, char> = HashMap::from([
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
        ('<', '>'),
    ]);

    for c in line {
        match c {
            '(' | '[' | '{' | '<' => stack.push(*c),
            ')' | ']' | '}' | '>' => {
                let p = stack.pop();
                match p {
                    None => break,
                    Some(pp) => {
                        if *c != closing[&pp] {
                            return vec![];
                        }
                    }
                }
            }
            _ => panic!("Invalid character in input")
        }
    }
    stack.reverse();
    stack.iter().map(|c| closing[c]).collect()
}

fn evaluate_line_of_missing(line: & CodeLine) -> u64 {
    let values = HashMap::<char, u64>::from([
        (')', 1),
        (']', 2),
        ('}', 3),
        ('>', 4),

    ]);

    let mut line_total : u64 = 0;
    for v in line.iter().map(|c| values[c]) {
        line_total *= 5;
        line_total += v;
    }

    line_total
}

fn median(mut data: Vec<u64>) -> u64 {
    data.sort_unstable();
    data[data.len()/2]
}

fn evaluate_lines_of_missing(lines: & Code) -> u64 {
    let mut totals : Vec<u64> = lines.iter().map(|line| evaluate_line_of_missing(&line)).collect();
    totals = totals.iter().filter(|v| **v > 0).map(|v| *v).collect();
    median(totals)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to load input file");
    let lines = parse_input(&input);
    let missing = lines.iter().map(|line| find_missing_chars(&line)).collect();
    println!("{}", evaluate_lines_of_missing(&missing));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn given_input() -> String {
        [ 
            "[({(<(())[]>[[{[]{<()<>>",
            "[(()[<>])]({[<{<<[]>>(",
            "{([(<{}[<>[]}>{[]{[(<()>",
            "(((({<>}<{<{<>}{[]{[]{}",
            "[[<[([]))<([[{}[[()]]]",
            "[{[{({}]{}}([{[{{{}}([]",
            "{<[[]]>}<{[{[{[]{()[[[]",
            "[<(<(<(<{}))><([]([]()",
            "<{([([[(<>()){}]>(<<{{",
            "<{([{{}}[<[[[<>{}]]]>[]]",
            ""
        ].join("\n")
    }

    fn parsed_input() -> Code {
        vec![
            vec!['[','(','{','(','<','(','(',')',')','[',']','>','[','[','{','[',']','{','<','(',')','<','>','>'],
            vec!['[','(','(',')','[','<','>',']',')',']','(','{','[','<','{','<','<','[',']','>','>','('],
            vec!['{','(','[','(','<','{','}','[','<','>','[',']','}','>','{','[',']','{','[','(','<','(',')','>'],
            vec!['(','(','(','(','{','<','>','}','<','{','<','{','<','>','}','{','[',']','{','[',']','{','}'],
            vec!['[','[','<','[','(','[',']',')',')','<','(','[','[','{','}','[','[','(',')',']',']',']'],
            vec!['[','{','[','{','(','{','}',']','{','}','}','(','[','{','[','{','{','{','}','}','(','[',']'],
            vec!['{','<','[','[',']',']','>','}','<','{','[','{','[','{','[',']','{','(',')','[','[','[',']'],
            vec!['[','<','(','<','(','<','(','<','{','}',')',')','>','<','(','[',']','(','[',']','(',')'],
            vec!['<','{','(','[','(','[','[','(','<','>','(',')',')','{','}',']','>','(','<','<','{','{'],
            vec!['<','{','(','[','{','{','}','}','[','<','[','[','[','<','>','{','}',']',']',']','>','[',']',']'],
        ]
    }

    fn missing_chars() -> Vec<Vec<char>> {
        vec![
            vec!['}', '}', ']', ']', ')', '}', ')', ']'],
            vec![')', '}', '>', ']', '}', ')'],
            vec![],
            vec!['}', '}', '>', '}', '>', ')', ')', ')', ')'],
            vec![],
            vec![],
            vec![']', ']', '}', '}', ']', '}', ']', '}', '>'],
            vec![],
            vec![],
            vec![']', ')', '}', '>'],
        ]
    }

#[test]
    fn test_parse_input() {
        assert_eq!(parse_input(&given_input()), parsed_input());
    }

#[test]
    fn test_find_incomplete_lines() {
        for (input, missing) in parsed_input().iter().zip(missing_chars().iter()) {
            assert_eq!(find_missing_chars(input), *missing);
        }
    }

#[test]
    fn test_evaluate_line_of_missing() {
        assert_eq!(evaluate_line_of_missing(&vec!['}', '}', ']', ']', ')', '}', ')', ']']), 288957);
    }

#[test]
    fn test_median() {
        assert_eq!(median(vec![288957, 5566, 1480781, 995444, 294]), 288957);
    }

#[test]
    fn test_evaluate_lines_of_missing() {
        assert_eq!(evaluate_lines_of_missing(&missing_chars()), 288957);
    }

}
