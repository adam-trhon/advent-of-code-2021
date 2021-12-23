
#[derive(Clone)]
struct Map {
    room_a: [char; 2],
    room_b: [char; 2],
    room_c: [char; 2],
    room_d: [char; 2],
    hallway: [char; 11],
}

impl Map {
    fn new() -> Map {
        Map{room_a: ['.'; 2], room_b: ['.'; 2], room_c: ['.'; 2], room_d: ['.'; 2], hallway: ['.'; 11]}
    }

    fn get_room(&self, room: char) -> [char; 2] {
        match room {
            'A' => self.room_a.clone(),
            'B' => self.room_b.clone(),
            'C' => self.room_c.clone(),
            'D' => self.room_d.clone(),
            _ => panic!("invalid room")
        }
    }

    fn get_in_room(&self, room: char, i: usize) -> char {
        match room {
            'A' => self.room_a[i],
            'B' => self.room_b[i],
            'C' => self.room_c[i],
            'D' => self.room_d[i],
            _ => panic!("invalid room")
        }
    }

    fn set_in_room(&mut self, room: char, i: usize, val: char) {
        match room {
            'A' => self.room_a[i] = val,
            'B' => self.room_b[i] = val,
            'C' => self.room_c[i] = val,
            'D' => self.room_d[i] = val,
            _ => panic!("invalid room")
        }
    }

    fn is_solved(&self) -> bool {
        if self.room_a != ['a'; 2] {
            return false;
        }
        if self.room_b != ['b'; 2] {
            return false;
        }
        if self.room_c != ['c'; 2] {
            return false;
        }
        if self.room_d != ['d'; 2] {
            return false;
        }
        true
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#############\n#{}#\n###{}#{}#{}#{}###\n  #{}#{}#{}#{}#\n  #########",
            self.hallway.iter().collect::<String>(),
            self.room_a[0], self.room_b[0], self.room_c[0], self.room_d[0],
            self.room_a[1], self.room_b[1], self.room_c[1], self.room_d[1],
        )
    }
}

fn abs_dist(l: usize, r:usize) -> usize {
    std::cmp::max(l, r) - std::cmp::min(l, r)
}

fn room_position(room: char) -> usize {
    match room {
        'A' => 2,
        'B' => 4,
        'C' => 6,
        'D' => 8,
        _ => panic!("invalid room")
    }
}

fn hallway_room_distance(hallway_pos: usize, room: char) -> usize {
    abs_dist(room_position(room), hallway_pos)
}

fn gen_range(a: usize, b: usize) -> std::ops::Range<usize> {
    std::ops::Range{start: std::cmp::min(a, b), end: std::cmp::max(a, b) + 1}
}

fn scale_price(price: usize, name: char) -> usize {
    match name {
        'A' => price,
        'B' => price*10,
        'C' => price*100,
        'D' => price*1000,
        _ => panic!("invalid room")
    }
}

fn hallway_to_room(map: &Map, hallway_pos: usize, name: char) -> Option<(usize, usize)> {
    let room_pos = room_position(name);
    for i in gen_range(hallway_pos, room_pos) {
        if map.hallway[i] != '.' {
            return None;
        }
    }

    let mut price = hallway_room_distance(hallway_pos, name);

    for i in 0..2 {
        if map.get_room(name)[i] == name {
            continue;
        }
        if map.get_room(name)[i] == name.to_ascii_lowercase() {
            continue;
        }
        if map.get_room(name)[i] == '.' {
            continue;
        }
        return None;
    }

    let mut pos_in_room: usize = 0;
    while map.get_in_room(name, pos_in_room).to_ascii_uppercase() != name {
        pos_in_room += 1;
        if pos_in_room == 2 {
            break;
        }
    }

    pos_in_room -= 1;

    price += 1 + pos_in_room;

    price = scale_price(price, name);

    Some((price, pos_in_room))
}

fn room_to_hallway(map: &Map, hallway_pos: usize, name: char) -> Option<(usize, usize)> {
    let room_pos = room_position(name);
    for i in gen_range(hallway_pos, room_pos) {
        if map.hallway[i] != '.' {
            return None;
        }
    }

    let mut price = hallway_room_distance(hallway_pos, name);

    let mut pos_in_room: usize = 0;
    while pos_in_room < 2 {
        if map.get_in_room(name, pos_in_room).is_ascii_lowercase() {
            return None;
        }
        if map.get_in_room(name, pos_in_room) != '.' {
            break;
        }
        pos_in_room += 1;
    }
    if pos_in_room == 2 {
        return None;
    }

    price += 1 + pos_in_room;

    price = scale_price(price, map.get_in_room(name, pos_in_room));

    Some((price, pos_in_room))
}

fn generate_moves(input: &Map, price: usize, mut best_solution: usize, monitor: bool) -> usize {
    for pos_in_hallway in [0, 1, 3, 5, 7, 9, 10] {
        for room_name in ['A', 'B', 'C', 'D'] {
            if monitor {
                println!("{}{}", pos_in_hallway, room_name);
            }
            match room_to_hallway(input, pos_in_hallway, room_name) {
                Some((gen_price, pos_in_room)) => {
                    let mut moved: Map = input.clone();
                    moved.hallway[pos_in_hallway] = moved.get_in_room(room_name, pos_in_room);
                    moved.set_in_room(room_name, pos_in_room, '.');
                    let moved_price = price + gen_price;
                    if moved_price < best_solution {
                        best_solution = generate_moves(&moved, moved_price, best_solution, false);
                    }
                }
                None => {
                }
            }
        }
    }

    for pos_in_hallway in [0, 1, 3, 5, 7, 9, 10] {
        let name = input.hallway[pos_in_hallway];
        if name != '.' {
            let mut moved = input.clone();
            moved.hallway[pos_in_hallway] = '.';
            match hallway_to_room(&moved, pos_in_hallway, name) {
                Some((gen_price, pos_in_room)) => {
                    moved.set_in_room(name, pos_in_room, name.to_ascii_lowercase());
                    let moved_price = price + gen_price;
                    if moved_price < best_solution {
                        if moved.is_solved() {
                            println!("found solution, price {}", moved_price);
                            best_solution = moved_price;
                        } else {
                            best_solution = generate_moves(&moved, moved_price, best_solution, false);
                        }
                    }
                }
                None => {
                }
            }
        }
    }

    best_solution
}

fn parse_input(text: &str) -> Map {
    let lines = text.split("\n").filter(|l| !l.is_empty())
        .map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let mut result = Map::new();
    result.room_a[0] = lines[2][3];
    result.room_a[1] = lines[3][3];
    result.room_b[0] = lines[2][5];
    result.room_b[1] = lines[3][5];
    result.room_c[0] = lines[2][7];
    result.room_c[1] = lines[3][7];
    result.room_d[0] = lines[2][9];
    result.room_d[1] = lines[3][9];

    result
}

fn main() {
    let text = std::fs::read_to_string("input.txt").expect("cannot read input.txt");
    let input = parse_input(&text);
    generate_moves(&input, 0, usize::MAX, true);
}

#[cfg(test)]
mod test {
    use super::*;

#[test]
    fn test_hallway_to_room() {
        //#CD...B.A..A#
        //###.#.#.#.###
        //  #.#b#c#d#
        let mut initial = Map{
            room_a: ['.', '.'],
            room_b: ['.', 'b'],
            room_c: ['.', 'c'],
            room_d: ['.', 'd'],
            hallway: ['C', 'D', '.', '.', '.', 'B', '.', 'A', '.', '.', 'A']
        };

        initial.hallway[5] = '.';
        assert_eq!(hallway_to_room(&initial, 5, 'B'), Some((20, 0)));
    }
}
