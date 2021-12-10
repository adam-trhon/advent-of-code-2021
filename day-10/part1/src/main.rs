use std::fs;
use std::collections::HashMap;

type CodeLine = Vec<char>;
type Code = Vec<CodeLine>;

fn parse_input(input: &str) -> Code {
    input.split_whitespace().map(|line| line.chars().collect()).collect()
}

fn find_invalid_char(line: &CodeLine) -> Option<char> {
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
                            return Some(*c);
                        }
                    }
                }
            }
            _ => panic!("Invalid character in input")
        }
    }
    None
}

fn evaluate_invalid_chars(invalid_chars: & Vec<Option<char>>) -> u32 {
    let mut result: u32 = 0;
    for c in invalid_chars {
        match c {
            None => continue,
            Some(')') => result += 3,
            Some(']') => result += 57,
            Some('}') => result += 1197,
            Some('>') => result += 25137,
            _ => panic!("invalid char for evaluation")
        }
    }
    result
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to load input file");
    let lines = parse_input(&input);
    let invalid_chars : Vec<Option<char>> = lines.iter().map(|l| find_invalid_char(l)).collect();
    let evaluation = evaluate_invalid_chars(&invalid_chars);
    println!("{}", evaluation);
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

    fn invalid_chars() -> Vec<Option<char>> {
        vec![
            None,
            None,
            Some('}'),
            None,
            Some(')'),
            Some(']'),
            None,
            Some(')'),
            Some('>'),
            None,
        ]
    }

    fn evaluation() -> u32 {
        26397
    }

#[test]
    fn test_parse_input() {
        assert_eq!(parse_input(&given_input()), parsed_input());
    }

#[test]
    fn test_find_invalid_character() {
        for (input, invalid) in parsed_input().iter().zip(invalid_chars().iter()) {
            assert_eq!(find_invalid_char(input), *invalid);
        }
    }

#[test]
    fn test_evaluate_invalid_chars() {
        assert_eq!(evaluate_invalid_chars(&invalid_chars()), evaluation());
    }
}
