use std::fs;

fn parse_input(input: &str) -> Vec<(Vec<String>, Vec<String>)> {
    let mut result : Vec<(Vec<String>, Vec<String>)> = Vec::new();

    for line in input.trim().split("\n") {
        let mut line_split = line.split(" | ");
        let left : &str = line_split.next().expect("failed to read left");
        let right : &str = line_split.next().expect("failed to read right");

        let mut left_vec : Vec<String> = Vec::new();
        left_vec.extend(left.trim().split(" ").map(|s| String::from(s)));
        let mut right_vec : Vec<String> = Vec::new();
        right_vec.extend(right.trim().split(" ").map(|s| String::from(s)));

        result.push((left_vec, right_vec));
    }

    result
}

fn main() {
    let input_string = fs::read_to_string("input.txt").expect("failed to read input string");
    let input_data = parse_input(&input_string);

    let mut count: usize = 0;
    for data_line in input_data {
        for number in data_line.1 {
            let len = number.len();
            if len == 2 || len == 4 || len == 3 || len == 7 {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
