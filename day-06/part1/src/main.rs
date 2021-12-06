use std::fs;

const DAYS_TO_BREED : usize = 8;
const AGE_ARRAY_LEN : usize = DAYS_TO_BREED + 1;

fn parse_input_numbers(input: &str) -> [usize; AGE_ARRAY_LEN] {
    let mut fishes : Vec<usize> = Vec::new();

    fishes.extend(input.trim().split(",").map(|s| s.parse::<usize>().unwrap()));

    let mut ages : [usize; AGE_ARRAY_LEN] = [0; AGE_ARRAY_LEN];

    for i in 0..AGE_ARRAY_LEN {
        ages[i] = fishes.iter().filter(|age| **age == i).count();
    }

    ages
}

fn simulate_a_day(today: [usize; AGE_ARRAY_LEN]) -> [usize; AGE_ARRAY_LEN] {
    let mut tomorrow : [usize; AGE_ARRAY_LEN] = [0; AGE_ARRAY_LEN];

    tomorrow[6] = today[0];
    tomorrow[8] = today[0];
    for i in 1..AGE_ARRAY_LEN {
        tomorrow[i-1] += today[i];
    }

    tomorrow
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to load input file");
    let mut current_day = parse_input_numbers(&input);

    for _ in 0..256 {
        current_day = simulate_a_day(current_day);
    }

    let final_count : usize = current_day.iter().sum();

    println!("{}", final_count);
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn test_parse_input_numbers() {
        let input = "4,3,4,5,2,1,1,5,5";
        let actual_result = parse_input_numbers(input);
        let expected_result = [0, 2, 1, 1, 2, 3, 0, 0, 0];
        assert_eq!(actual_result, expected_result);
    }
}
