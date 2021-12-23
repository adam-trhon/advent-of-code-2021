use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

const INF : u32 = u32::MAX/2; // practical "infinity" for our purposes

type Map = Vec<Vec<u32>>;
type Position = (usize, usize);

fn load_input_map(text: &str) -> Map {
    let mut map = Map::new();

    for line in text.split_whitespace() {
        map.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>());
        assert_eq!(map[map.len()-1].len(), map[0].len());
    }

    map
}

struct AStar {
    map: Map,
    open: HashSet<Position>,
    came_from: HashMap<Position, Position>,
    g_score: HashMap<Position, u32>,
    f_score: HashMap<Position, u32>,
    start: Position,
    goal: Position,
}

impl AStar {
    pub fn new(map: Map, start: &Position, goal: &Position) -> Self {
        let mut result = Self {
            map: map,
            open: HashSet::from([*start]),
            came_from: HashMap::new(),
            g_score: HashMap::from([(*start, 0)]),
            f_score: HashMap::new(),
            start: *start,
            goal: *goal,
        };

        result.f_score.insert(result.start, result.h(start));

        result
    }

    pub fn h(& self, pos: &Position) -> u32 {
        (self.goal.0-pos.0 + (self.goal.1-pos.1)) as u32
    }

    pub fn find_current(& self) -> Position {
        let mut best_pos: &Position = self.open.iter().next().expect("open set is empty");
        let mut best_score: &u32 = self.f_score.get(best_pos).unwrap_or(&INF);

        for pos in self.open.iter() {
            let score = self.f_score.get(pos).unwrap_or(&INF);
            if score < best_score {
                best_score = score;
                best_pos = pos;
            }
        }

        *best_pos
    }

    pub fn gen_neighbours(& self, position: &Position) -> Vec<Position> {
        let mut neighbours : Vec<Position> = Vec::new();
        if position.0 > 1 {
            neighbours.push((position.0-1, position.1));
        }
        if position.0 < self.map.len()-1 {
            neighbours.push((position.0+1, position.1));
        }
        if position.1 > 1 {
            neighbours.push((position.0, position.1-1));
        }
        if position.1 < self.map[0].len()-1 {
            neighbours.push((position.0, position.1+1));
        }

        neighbours
    }

    pub fn iter(&mut self) -> bool {
        let mut repeat = false;

        if ! self.open.is_empty() {
            let current = self.find_current();
            if current != self.goal {
                repeat = true;
                self.open.remove(&current);

                for neighbour in self.gen_neighbours(&current).iter() {
                    let neighbour_price = self.map[neighbour.0][neighbour.1];
                    let tentative_g_score = self.g_score.get(&current).unwrap_or(&INF) + neighbour_price;
                    if tentative_g_score < *self.g_score.get(neighbour).unwrap_or(&INF) {
                        self.came_from.insert(*neighbour, current);
                        self.g_score.insert(*neighbour, tentative_g_score);
                        self.f_score.insert(*neighbour, tentative_g_score + self.h(neighbour));
                        self.open.insert(*neighbour);
                    }
                }
            }
        }

        repeat
    }
}

fn a_star_search(map: Map, start: Position, goal: Position) -> u32 {

    let mut a_star = AStar::new(map, &start, &goal);

    while a_star.iter() {
    }

    *a_star.g_score.get(&goal).expect("search failed")
}



fn main() {
    let input_text = fs::read_to_string("input.txt").expect("cannot read input file");
    let map = load_input_map(&input_text);
    let best_path_value = a_star_search(map.clone(), (0, 0), (map.len()-1, map[0].len()-1));
    println!("{}", best_path_value);
}
