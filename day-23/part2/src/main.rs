
const ROOM_SIZE: usize = 4;

#[derive(Clone)]
struct Map {
    room_a: [char; ROOM_SIZE],
    room_b: [char; ROOM_SIZE],
    room_c: [char; ROOM_SIZE],
    room_d: [char; ROOM_SIZE],
    hallway: [char; 11],
}

impl Map {
    fn new() -> Map {
        Map{
            room_a: ['.'; ROOM_SIZE],
            room_b: ['.'; ROOM_SIZE],
            room_c: ['.'; ROOM_SIZE],
            room_d: ['.'; ROOM_SIZE],
            hallway: ['.'; 11]
        }
    }

    fn get_room(&self, room: char) -> [char; ROOM_SIZE] {
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
        if self.room_a != ['a'; ROOM_SIZE] {
            return false;
        }
        if self.room_b != ['b'; ROOM_SIZE] {
            return false;
        }
        if self.room_c != ['c'; ROOM_SIZE] {
            return false;
        }
        if self.room_d != ['d'; ROOM_SIZE] {
            return false;
        }
        true
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result: String = String::new();
        result.push_str("#############\n");
        result.push_str(&format!("#{}#\n", self.hallway.iter().collect::<String>()));
        result.push_str(&format!("###{}#{}#{}#{}###\n",
            self.room_a[0], self.room_b[0], self.room_c[0], self.room_d[0]));
        for i in 1..ROOM_SIZE {
            result.push_str(&format!("  #{}#{}#{}#{}#\n", 
                self.room_a[i], self.room_b[i], self.room_c[i], self.room_d[i]));
        }
        result.push_str("  #########");
        write!(f, "{}", result)
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

    for i in 0..ROOM_SIZE {
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
        if pos_in_room == ROOM_SIZE {
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
    while pos_in_room < ROOM_SIZE {
        if map.get_in_room(name, pos_in_room).is_ascii_lowercase() {
            return None;
        }
        if map.get_in_room(name, pos_in_room) != '.' {
            break;
        }
        pos_in_room += 1;
    }
    if pos_in_room == ROOM_SIZE {
        return None;
    }

    price += 1 + pos_in_room;

    price = scale_price(price, map.get_in_room(name, pos_in_room));

    Some((price, pos_in_room))
}

fn generate_moves(input: &Map, price: usize, mut best_solution: usize, monitor: usize) -> usize {
    for pos_in_hallway in [0, 1, 3, 5, 7, 9, 10] {
        for room_name in ['A', 'B', 'C', 'D'] {
            if monitor > 0{
                let mut indent = String::new();
                for i in monitor..5 {
                    indent.push(' ');
                }
                println!("{}+{}{}", indent, pos_in_hallway, room_name);
            }
            match room_to_hallway(input, pos_in_hallway, room_name) {
                Some((gen_price, pos_in_room)) => {
                    let mut moved: Map = input.clone();
                    moved.hallway[pos_in_hallway] = moved.get_in_room(room_name, pos_in_room);
                    moved.set_in_room(room_name, pos_in_room, '.');
                    let moved_price = price + gen_price;
                    if moved_price < best_solution {
                        best_solution = generate_moves(&moved, moved_price, best_solution, monitor/2);
                    }
                }
                None => {
                }
            }
        }
    }

    for pos_in_hallway in [0, 1, 3, 5, 7, 9, 10] {
        if monitor > 0{
            let mut indent = String::new();
            for i in monitor..5 {
                indent.push(' ');
            }
            println!("{}-{}", indent, pos_in_hallway);
        }
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
                            best_solution = generate_moves(&moved, moved_price, best_solution, monitor/2);
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
    result.room_a[1] = 'D';
    result.room_a[2] = 'D';
    result.room_a[3] = lines[3][3];
    result.room_b[0] = lines[2][5];
    result.room_b[1] = 'C';
    result.room_b[2] = 'B';
    result.room_b[3] = lines[3][5];
    result.room_c[0] = lines[2][7];
    result.room_c[1] = 'B';
    result.room_c[2] = 'A';
    result.room_c[3] = lines[3][7];
    result.room_d[0] = lines[2][9];
    result.room_d[1] = 'A';
    result.room_d[2] = 'C';
    result.room_d[3] = lines[3][9];

    result
}

fn main() {
    let text = std::fs::read_to_string("input.txt").expect("cannot read input.txt");
    let input = parse_input(&text);
    println!("{}", generate_moves(&input, 0, usize::MAX, 2));
}
