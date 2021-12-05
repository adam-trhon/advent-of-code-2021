use std::fs;
use std::collections::HashSet;

const BOARD_SIZE : usize = 5;

fn parse_input(line: &str) -> Vec<u32> {
    let mut result : Vec<u32> = Vec::new();
    let numbers = line.split(",");
    for number in numbers {
        result.push(number.parse().unwrap());
    }
    result
}

fn parse_board(lines: &[&str]) -> Vec<Vec<u32>> {
    let mut result : Vec<Vec<u32>> = Vec::new();

    for line in lines {
        let mut line_numbers : Vec<u32> = Vec::new();
        for number in line.split_whitespace() {
            line_numbers.push(number.parse().unwrap())
        }
        result.push(line_numbers);
    }

    result
}

fn generate_board_sets(board: Vec<Vec<u32>>) -> Vec<HashSet<u32>> {
    let board_size = board.len();
    let mut result : Vec<HashSet<u32>> = vec![HashSet::new(); 2*board_size+2];

    for (i, row) in board.iter().enumerate() {
        assert_eq!(row.len(), board_size);
        for (j, number) in row.iter().enumerate() {
            assert_eq!(result[i].contains(&number), false);
            result[i].insert(number.clone());
            result[board_size + j].insert(number.clone());
            if i == j {
                assert_eq!(result[board_size*2].contains(&number), false);
                result[board_size*2].insert(number.clone());
            }
            if i + j == board_size-1 {
                assert_eq!(result[board_size*2+1].contains(&number), false);
                result[board_size*2+1].insert(number.clone());
            }
        }
    }

    result
}

fn sum_board_remains(board: &Vec<HashSet<u32>>) -> u32 {
    let mut result : u32 = 0;
    for set in &board[0..BOARD_SIZE] {
        for number in set {
            result += number;
        }
    }
    result
}

fn main()  {
    let contents = fs::read_to_string("input.txt").expect("Input file read failed");
    let mut lines : Vec<&str> = Vec::new();
    lines.extend(contents.split("\n").filter(|l| ! l.is_empty()));

    let input_numbers = parse_input(lines[0]);

    let mut board_state : Vec<Vec<HashSet<u32>>> = Vec::new();
    for i in (1..lines.len()).step_by(BOARD_SIZE) {
        board_state.push(generate_board_sets(parse_board(&lines[i..i+BOARD_SIZE])))
    }

    'input_loop: for input in input_numbers {
        for board in &mut board_state {
            for board_set in &mut *board {
                board_set.remove(&input);
                if board_set.is_empty() {
                    println!("{}", sum_board_remains(board)*input);
                    break 'input_loop;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn test_parse_input() {
        assert_eq!(parse_input("85,84"), vec![85,84]);
    }

#[test]
    fn test_parse_board() {
        let input : Vec<&str> = vec![
            "78 13  8 62 67",
            "42 89 97 16 65",
            " 5 12 73 50 56",
            "45 10 63 41 64",
            "49  1 95 71 17",
        ];

        let output = parse_board(&input[..]);

        let expected_output : Vec<Vec<u32>> = vec![
            vec![78, 13,  8, 62, 67],
            vec![42, 89, 97, 16, 65],
            vec![ 5, 12, 73, 50, 56],
            vec![45, 10, 63, 41, 64],
            vec![49,  1, 95, 71, 17],
        ];

        assert_eq!(output, expected_output);

    }

#[test]
    fn test_generate_board_sets() {
        let input : Vec<Vec<u32>> = vec![
            vec![78, 13,  8, 62, 67],
            vec![42, 89, 97, 16, 65],
            vec![ 5, 12, 73, 50, 56],
            vec![45, 10, 63, 41, 64],
            vec![49,  1, 95, 71, 17],
        ];

        let output = generate_board_sets(input);

        let expected_output : Vec<HashSet<u32>> = vec![
            [78, 13,  8, 62, 67].iter().cloned().collect(),
            [42, 89, 97, 16, 65].iter().cloned().collect(),
            [ 5, 12, 73, 50, 56].iter().cloned().collect(),
            [45, 10, 63, 41, 64].iter().cloned().collect(),
            [49,  1, 95, 71, 17].iter().cloned().collect(),

            [78, 42,  5, 45, 49].iter().cloned().collect(),
            [13, 89, 12, 10,  1].iter().cloned().collect(),
            [ 8, 97, 73, 63, 95].iter().cloned().collect(),
            [62, 16, 50, 41, 71].iter().cloned().collect(),
            [67, 65, 56, 64, 17].iter().cloned().collect(),

            [78, 89, 73, 41, 17].iter().cloned().collect(),
            [49, 10, 73, 16, 67].iter().cloned().collect(),
        ];

        assert_eq!(output, expected_output);
    }
}
