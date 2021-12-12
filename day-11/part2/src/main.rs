use std::fs;

type Tile = u32;
type MapRow = Vec<Tile>;
type Map = Vec<MapRow>;
type Coords = (usize, usize);

fn parse_input_line(input_line: &str) -> MapRow {
    input_line.chars().map(|c| c.to_digit(10).unwrap()).collect::<MapRow>()
}

fn parse_input(input: &String) -> Map {
    input.split_whitespace().map(|line| parse_input_line(&line)).collect::<Map>()
}

fn charge_octopi(map: & mut Map) {
    for row in map {
        for tile in row {
            *tile += 1
        }
    }
}

fn get_neighbours(tile: Coords, bottom_right_tile: Coords) -> Vec<Coords> {
    let mut result = Vec::<Coords>::new();
    let row = tile.0;
    let col = tile.1;

    if row > 0 {
        if col > 0 {
            result.push((row-1, col-1));
        }
        result.push((row-1, col));
        if col < bottom_right_tile.1 {
            result.push((row-1, col + 1));
        }
    }

    if col > 0 {
        result.push((row, col-1));
    }

    if col < bottom_right_tile.1 {
        result.push((row, col+1));
    }

    if row < bottom_right_tile.0 {
        if col > 0 {
            result.push((row+1, col-1));
        }
        result.push((row+1, col));
        if col < bottom_right_tile.1 {
            result.push((row+1, col + 1));
        }
    }

    result
}

fn flash_octopi(map: &mut Map) -> Vec<Coords> {
    let mut flashes = Vec::<Coords>::new();

    let bottom_right_tile = (map.len()-1, map[0].len()-1);

    for row in 0..=bottom_right_tile.0 {
        for col in 0..=bottom_right_tile.1 {
            if map[row][col] > 9 {
                flashes.push((row, col));
                map[row][col] = 0;
                for (r, c) in get_neighbours((row, col), bottom_right_tile) {
                    map[r][c] += 1;
                }
            }
        }
    }

    flashes
}

fn simulate_step(map: &mut Map) -> Vec<Coords> {
    let mut total_flashes = Vec::<Coords>::new();

    charge_octopi(map);

    loop {
        let mut new_flashes = flash_octopi(map);
        if new_flashes.is_empty() {
            break;
        }
        total_flashes.extend(new_flashes.drain(..));
    }

    for (row, col) in &total_flashes {
        map[*row][*col] = 0;
    }

    total_flashes
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to load input file");
    let mut map = parse_input(&input);

    let octopi_count = map.len() * map[0].len();
    let mut steps_to_all = 1;

    while simulate_step(&mut map).len() < octopi_count {
        steps_to_all += 1;
    }

    println!("{}", steps_to_all);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn state_0() -> Map {
        vec![
            vec![1, 1, 1, 1, 1],
            vec![1, 9, 9, 9, 1],
            vec![1, 9, 1, 9, 1],
            vec![1, 9, 9, 9, 1],
            vec![1, 1, 1, 1, 1],
        ]
    }

    fn state_1() -> Map {
        vec![
            vec![3, 4, 5, 4, 3],
            vec![4, 0, 0, 0, 4],
            vec![5, 0, 0, 0, 5],
            vec![4, 0, 0, 0, 4],
            vec![3, 4, 5, 4, 3],
        ]
    }

#[test]
    fn test_simulate_step_1() {
        let mut map = state_0();
        assert_eq!(simulate_step(&mut map).len(), 9);
        assert_eq!(map, state_1());
    }


    fn state_example_0() -> Map {
        vec![
            vec![5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
            vec![2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
            vec![5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
            vec![6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
            vec![6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
            vec![4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
            vec![2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
            vec![6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
            vec![4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
            vec![5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
        ]
    }


#[test]
    fn test_simulate_10_example_steps() {
        assert_eq!(simulate_steps(&mut state_example_0(), 10).len(), 204);
    }

}
