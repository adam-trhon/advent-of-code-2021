use std::fs;

type InputMapRow = Vec<u32>;
type InputMap = Vec<InputMapRow>;

fn make_input_map(input: &str) -> InputMap {
    let mut result = InputMap::new();

    for line in input.split_whitespace() {
        let mut row = InputMapRow::new();
        row.push(10);
        for height in line.chars() {
            row.push(height.to_digit(10).unwrap());
        }
        row.push(10);
        if result.is_empty() {
            result.push(vec![10; row.len()]);
        }
        assert_eq!(row.len(), result[0].len());
        result.push(row);
    }
    result.push(vec![10; result[0].len()]);

    result
}

fn find_low_spots(map: &InputMap) -> Vec<u32> {
    let mut low_spots = Vec::<u32>::new();
    for i in 1..map.len()-1 {
        for j in 1..map[i].len()-1 {
            if map[i-1][j] <= map[i][j] {
                continue;
            }
            if map[i+1][j] <= map[i][j] {
                continue;
            }
            if map[i][j-1] <= map[i][j] {
                continue;
            }
            if map[i][j+1] <= map[i][j] {
                continue;
            }
            low_spots.push(map[i][j]);
        }
    }
    low_spots
}

fn evaluate_risk_level(low_spots: &Vec<u32>) -> u32 {
    low_spots.iter().map(|spot| spot+1).sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to read input file");
    let input_map = make_input_map(&input);
    let low_spots = find_low_spots(&input_map);
    let risk_level = evaluate_risk_level(&low_spots);
    println!("{}", risk_level);
}

#[cfg(test)]
mod tests {
    use super::*;
    fn given_input() -> String {
        [ 
        "2199943210",
        "3987894921",
        "9856789892",
        "8767896789",
        "9899965678",
        ].join("\n")
    }

    fn parsed_map() -> InputMap {
        vec![
        	vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,  10], 
        	vec![10,  2,  1,  9,  9,  9,  4,  3,  2,  1,  0,  10], 
        	vec![10,  3,  9,  8,  7,  8,  9,  4,  9,  2,  1,  10], 
        	vec![10,  9,  8,  5,  6,  7,  8,  9,  8,  9,  2,  10], 
        	vec![10,  8,  7,  6,  7,  8,  9,  6,  7,  8,  9,  10], 
        	vec![10,  9,  8,  9,  9,  9,  6,  5,  6,  7,  8,  10], 
        	vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,  10], 
        ]
    }

    fn low_spots() -> Vec<u32> {
        vec![1, 0, 5, 5]
    }

    fn risk_level() -> u32 {
        15
    }

#[test]
    fn test_make_input_map() {
        assert_eq!(parsed_map(), make_input_map(&given_input()));
    }

#[test]
    fn test_find_low_spots() {
        assert_eq!(low_spots(), find_low_spots(&parsed_map()));
    }

#[test]
    fn test_evaluate_risk_level() {
        assert_eq!(risk_level(), evaluate_risk_level(&low_spots()));
    }
}
