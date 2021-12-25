
type MapRow = Vec<char>;
type Map = Vec<MapRow>;

fn parse_input(text: &str) -> Map {
    let mut result = Map::new();
    for line in text.split("\n").filter(|l| l.len() > 0) {
        result.push(line.chars().collect());
    }
    result
}

fn get_east(map: &Map, col: usize) -> usize {
    let rowsize = map[0].len();
    if col < rowsize - 1 {
        col + 1
    } else {
        0
    }
}

fn get_south(map: &Map, row: usize) -> usize {
    let colsize = map.len();
    if row < colsize - 1 {
        row + 1
    } else {
        0
    }
}

fn move_east(map: &mut Map) -> bool {
    let mut moving = false;
    let mut new_map = vec![vec!['.'; map[0].len()]; map.len()];

    for row in 0..map.len() {
        for col in 0..map[row].len() {
            match map[row][col] {
                '>' => {
                    let col_east = get_east(map, col);
                    if map[row][col_east] == '.' {
                        new_map[row][col_east] = '>';
                        moving = true;
                    } else {
                        new_map[row][col] = '>';
                    }
                }
                'v' => {
                    new_map[row][col] = 'v';
                }
                _ => {
                }
            }
        }
    }

    map.clear();
    map.extend(new_map.into_iter());

    moving
}

fn move_south(map: &mut Map) -> bool {
    let mut moving = false;
    let mut new_map = vec![vec!['.'; map[0].len()]; map.len()];

    for row in 0..map.len() {
        for col in 0..map[row].len() {
            match map[row][col] {
                'v' => {
                    let row_south = get_south(map, row);
                    if map[row_south][col] == '.' {
                        new_map[row_south][col] = 'v';
                        moving = true;
                    } else {
                        new_map[row][col] = 'v';
                    }

                }
                '>' => {
                    new_map[row][col] = '>';
                }
                _ => {
                }
            }
        }
    }

    map.clear();
    map.extend(new_map.into_iter());

    moving
}

fn evaluate_map(mut map: Map) -> usize {
    let mut moving = true;
    let mut move_count: usize = 0;

    while moving {
        moving = false;
        moving = move_east(&mut map) || moving;
        moving = move_south(&mut map) || moving;

        move_count += 1;
    }

    move_count

}

fn main() {
    let text = std::fs::read_to_string("input.txt").expect("failed to read input.txt");
    let map = parse_input(&text);
    println!("total move count: {}", evaluate_map(map.clone()));
}

#[cfg(test)]
mod tests 
{
    use super::*;

#[test]
    fn test_example() {
        let map = vec![
            vec!['v', '.', '.', '.', '>', '>', '.', 'v', 'v', '>'],
            vec!['.', 'v', 'v', '>', '>', '.', 'v', 'v', '.', '.'],
            vec!['>', '>', '.', '>', 'v', '>', '.', '.', '.', 'v'],
            vec!['>', '>', 'v', '>', '>', '.', '>', '.', 'v', '.'],
            vec!['v', '>', 'v', '.', 'v', 'v', '.', 'v', '.', '.'],
            vec!['>', '.', '>', '>', '.', '.', 'v', '.', '.', '.'],
            vec!['.', 'v', 'v', '.', '.', '>', '.', '>', 'v', '.'],
            vec!['v', '.', 'v', '.', '.', '>', '>', 'v', '.', 'v'],
            vec!['.', '.', '.', '.', 'v', '.', '.', 'v', '.', '>'],
        ];

        assert_eq!(evaluate_map(map.clone()), 58);

    }
}
