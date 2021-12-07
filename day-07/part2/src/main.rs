use std::fs;

fn parse_positions(input: &str) -> Vec<usize> {
    let mut result : Vec<usize> = Vec::new();
    result.extend(input.split(",").map(|s| s.parse::<usize>().unwrap()));
    result
}

fn find_available_range(positions: &Vec<usize>) -> (usize, usize) {
    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();

    (min, max-min)
}

fn get_fuel_for_distance(distance: usize) -> usize {
    distance * (distance+1) / 2
}

fn get_fuel_required_for_position(submarines: &Vec<usize>, position: usize) -> usize {
    let mut result : usize = 0;
    for submarine in submarines {
        result += get_fuel_for_distance(((position as i32) - (*submarine as i32)).abs() as usize);
    }

    result
}

fn main() {
    let input = String::from(fs::read_to_string("input.txt").expect("failed to read input").trim());
    let positions = parse_positions(&input);

    let (range_min, range_size) = find_available_range(&positions);

    let mut best_price = get_fuel_required_for_position(&positions, range_min);

    for position in range_min..range_min+range_size {
        let price = get_fuel_required_for_position(&positions, position);
        if price < best_price {
            best_price = price;
        }
    }

    println!("{}", best_price);
}


#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn test_get_fuel_required_for_position() {
        let submarines = vec![4,3,4,5,2,1,1,5,5];
        let position = 2;
        let expected_result : usize = 16;
        let actual_result = get_fuel_required_for_position(&submarines, position);
        assert_eq!(actual_result, expected_result);
    }
}
