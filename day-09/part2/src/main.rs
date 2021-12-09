use std::fs;

type Coords = (usize, usize);
type InputMapRow = Vec<u32>;
type InputMap = Vec<InputMapRow>;

fn make_input_map(input: &str) -> InputMap {
    let mut result = InputMap::new();

    for line in input.split_whitespace() {
        let mut row = InputMapRow::new();
        row.push(9);
        for height in line.chars() {
            row.push(height.to_digit(10).unwrap());
        }
        row.push(9);
        if result.is_empty() {
            result.push(vec![9; row.len()]);
        }
        assert_eq!(row.len(), result[0].len());
        result.push(row);
    }
    result.push(vec![9; result[0].len()]);

    result
}

fn get_neighbours(tile: Coords) -> Vec<Coords> {
    vec![
        (tile.0-1, tile.1),
        (tile.0+1, tile.1),
        (tile.0, tile.1-1),
        (tile.0, tile.1+1)
    ]
}

fn find_low_spots(map: &InputMap) -> Vec<(usize, usize)> {
    let mut low_spots = Vec::<(usize, usize)>::new();
    for i in 1..map.len()-1 {
        'point: for j in 1..map[i].len()-1 {
            for n in get_neighbours((i, j)) {
                if map[n.0][n.1] <= map[i][j] {
                    continue 'point;
                }
            }
            low_spots.push((i, j));
        }
    }
    low_spots
}

fn measure_basin_impl(map: & mut InputMap, pos: Coords) -> u32 {
    let mut result : u32 = 0;
    if map[pos.0][pos.1] != 9 {
        map[pos.0][pos.1] = 9;
        result = 1;
        for n in get_neighbours(pos) {
            result += measure_basin_impl(map, n);
        }
    }
    result
}

fn measure_basin(mut map: InputMap, low_spot: Coords) -> u32 {
    measure_basin_impl(&mut map, low_spot)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to read input file");
    let input_map = make_input_map(&input);
    let low_spots = find_low_spots(&input_map);
    let mut sizes : Vec<u32> = low_spots.iter().map(|t| measure_basin(input_map.clone(), *t)).collect();
    sizes.sort_unstable();
    sizes.reverse();
    let mut result : u32 = 1;
    for s in sizes[..3].iter() {
        result *= s;
    }
    println!("{}", result);
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
        	vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9], 
        	vec![9, 2, 1, 9, 9, 9, 4, 3, 2, 1, 0, 9], 
        	vec![9, 3, 9, 8, 7, 8, 9, 4, 9, 2, 1, 9], 
        	vec![9, 9, 8, 5, 6, 7, 8, 9, 8, 9, 2, 9], 
        	vec![9, 8, 7, 6, 7, 8, 9, 6, 7, 8, 9, 9], 
        	vec![9, 9, 8, 9, 9, 9, 6, 5, 6, 7, 8, 9], 
        	vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9], 
        ]
    }

    fn low_spots() -> Vec<(usize, usize)> {
        vec![(1, 2), (1, 10), (3, 3), (5, 7)]
    }

    fn risk_level() -> u32 {
        15
    }

    fn basin_size(low_spot: Coords) -> u32 {
        if low_spot == (1, 2) {
            3
        } else if low_spot == (1, 10) {
            9
        } else if low_spot == (3, 3) {
            14
        } else if low_spot == (5, 7) {
            9
        } else {
            panic!("unknown low spot");
        }
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
    fn test_measure_basin_size() {
        for low_spot in low_spots() {
            assert_eq!(basin_size(low_spot), measure_basin(parsed_map(), low_spot)) 
        }
    }
}
