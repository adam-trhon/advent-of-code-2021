use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use ansi_term::Style;

const INF : u32 = u32::MAX/2; // practical "infinity" for our purposes

type Position = (usize, usize);

type MapData = Vec<Vec<u32>>;

#[derive(Clone)]
struct Map {
    data: MapData,
    multiply: usize,
}

impl Map {
    pub fn rows(& self) -> usize {
        self.data.len()*self.multiply
    }
    pub fn cols(&self) -> usize {
        self.data[0].len()*self.multiply
    }

    pub fn get(& self, pos: Position) -> u32 {
        let wrapped_row = pos.0 % self.data.len();
        let wrapped_col = pos.1 % self.data[0].len();

        let wrapping_row = (pos.0 / self.data.len()) as u32;
        let wrapping_col = (pos.1 / self.data[0].len()) as u32;

        (self.data[wrapped_row][wrapped_col] + wrapping_row + wrapping_col - 1) % 9 + 1
    }

}

fn load_input_map(text: &str) -> Map {
    let mut map = Map{data: MapData::new(), multiply: 1};

    for line in text.split_whitespace() {
        map.data.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>());
        assert_eq!(map.data[map.data.len()-1].len(), map.data[0].len());
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

    pub fn h(& self, _pos: &Position) -> u32 {
        //(self.goal.0-pos.0 + (self.goal.1-pos.1)) as u32
        0
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
        if position.0 > 0 {
            neighbours.push((position.0-1, position.1));
        }
        if position.0 < self.map.rows()-1 {
            neighbours.push((position.0+1, position.1));
        }
        if position.1 > 0 {
            neighbours.push((position.0, position.1-1));
        }
        if position.1 < self.map.cols()-1 {
            neighbours.push((position.0, position.1+1));
        }

        neighbours
    }

    pub fn iter(&mut self) -> bool {
        let mut repeat = false;

        if ! self.open.is_empty() {
            let current = self.find_current();
            // countdown
            //println!("{}", self.h(&current));
            if current != self.goal {
                repeat = true;
                self.open.remove(&current);

                for neighbour in self.gen_neighbours(&current).iter() {
                    let neighbour_price: u32 = self.map.get(*neighbour);
                    let tentative_g_score: u32 = self.g_score.get(&current).unwrap_or(&INF) + neighbour_price;
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

    pub fn evaluate(&mut self) {
        while self.iter() {
        }
    }

    pub fn get_best_path_score(& self) -> u32 {
        *self.g_score.get(&self.goal).expect("search failed")
    }
    
    pub fn pretty_print(&self) {
        let mut path: HashSet<Position> = HashSet::new();
        let mut current = self.goal;
        let mut result: u32 = 0;
        while current != self.start {
            path.insert(current);
            current = *self.came_from.get(&current).unwrap();
        }

        for i in 0..self.map.rows() {
            for j in 0..self.map.cols() {
                let val = format!("{}", self.map.get((i, j)));
                if path.contains(&(i, j)) {
                    print!("{}", Style::new().bold().paint(val));
                    result += self.map.get((i, j));
                } else {
                    print!("{}", val);
                }
            }
            println!("");
        }

        println!("{}", result);
    }
}

fn a_star_search(map: Map, start: Position, goal: Position) -> u32 {
    let mut a_star = AStar::new(map, &start, &goal);
    a_star.evaluate();
    //a_star.pretty_print();
    a_star.get_best_path_score()
}



fn main() {
    let input_text = fs::read_to_string("input.txt").expect("cannot read input file");
    let mut map = load_input_map(&input_text);
    map.multiply = 5;
    let best_path_value = a_star_search(map.clone(), (0, 0), (map.rows()-1, map.cols()-1));
    println!("{}", best_path_value);
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn test_map_example() {
        let mut map = Map{data: MapData::new(), multiply: 5};
        map.data.push(vec![8]);

        assert_eq!(map.rows(), 5);
        assert_eq!(map.cols(), 5);

        assert_eq!(map.get((0,0)), 8);

        assert_eq!(map.get((0,1)), 9);
        assert_eq!(map.get((1,0)), 9);

        assert_eq!(map.get((3,4)), 6);
        assert_eq!(map.get((4,3)), 6);

        assert_eq!(map.get((4,4)), 7);

    }

#[test]
    fn test_map_multiplying() {
        let map1 = Map{
            data: vec![
                vec![1,1,6,3,7,5,1,7,4,2],
                vec![1,3,8,1,3,7,3,6,7,2],
                vec![2,1,3,6,5,1,1,3,2,8],
                vec![3,6,9,4,9,3,1,5,6,9],
                vec![7,4,6,3,4,1,7,1,1,1],
                vec![1,3,1,9,1,2,8,1,3,7],
                vec![1,3,5,9,9,1,2,4,2,1],
                vec![3,1,2,5,4,2,1,6,3,9],
                vec![1,2,9,3,1,3,8,5,2,1],
                vec![2,3,1,1,9,4,4,5,8,1],
            ],
            multiply: 5,
        };

        let map2 = Map{
            data: vec![
            vec![1,1,6,3,7,5,1,7,4,2,2,2,7,4,8,6,2,8,5,3,3,3,8,5,9,7,3,9,6,4,4,4,9,6,1,8,4,1,7,5,5,5,1,7,2,9,5,2,8,6],
            vec![1,3,8,1,3,7,3,6,7,2,2,4,9,2,4,8,4,7,8,3,3,5,1,3,5,9,5,8,9,4,4,6,2,4,6,1,6,9,1,5,5,7,3,5,7,2,7,1,2,6],
            vec![2,1,3,6,5,1,1,3,2,8,3,2,4,7,6,2,2,4,3,9,4,3,5,8,7,3,3,5,4,1,5,4,6,9,8,4,4,6,5,2,6,5,7,1,9,5,5,7,6,3],
            vec![3,6,9,4,9,3,1,5,6,9,4,7,1,5,1,4,2,6,7,1,5,8,2,6,2,5,3,7,8,2,6,9,3,7,3,6,4,8,9,3,7,1,4,8,4,7,5,9,1,4],
            vec![7,4,6,3,4,1,7,1,1,1,8,5,7,4,5,2,8,2,2,2,9,6,8,5,6,3,9,3,3,3,1,7,9,6,7,4,1,4,4,4,2,8,1,7,8,5,2,5,5,5],
            vec![1,3,1,9,1,2,8,1,3,7,2,4,2,1,2,3,9,2,4,8,3,5,3,2,3,4,1,3,5,9,4,6,4,3,4,5,2,4,6,1,5,7,5,4,5,6,3,5,7,2],
            vec![1,3,5,9,9,1,2,4,2,1,2,4,6,1,1,2,3,5,3,2,3,5,7,2,2,3,4,6,4,3,4,6,8,3,3,4,5,7,5,4,5,7,9,4,4,5,6,8,6,5],
            vec![3,1,2,5,4,2,1,6,3,9,4,2,3,6,5,3,2,7,4,1,5,3,4,7,6,4,3,8,5,2,6,4,5,8,7,5,4,9,6,3,7,5,6,9,8,6,5,1,7,4],
            vec![1,2,9,3,1,3,8,5,2,1,2,3,1,4,2,4,9,6,3,2,3,4,2,5,3,5,1,7,4,3,4,5,3,6,4,6,2,8,5,4,5,6,4,7,5,7,3,9,6,5],
            vec![2,3,1,1,9,4,4,5,8,1,3,4,2,2,1,5,5,6,9,2,4,5,3,3,2,6,6,7,1,3,5,6,4,4,3,7,7,8,2,4,6,7,5,5,4,8,8,9,3,5],
            vec![2,2,7,4,8,6,2,8,5,3,3,3,8,5,9,7,3,9,6,4,4,4,9,6,1,8,4,1,7,5,5,5,1,7,2,9,5,2,8,6,6,6,2,8,3,1,6,3,9,7],
            vec![2,4,9,2,4,8,4,7,8,3,3,5,1,3,5,9,5,8,9,4,4,6,2,4,6,1,6,9,1,5,5,7,3,5,7,2,7,1,2,6,6,8,4,6,8,3,8,2,3,7],
            vec![3,2,4,7,6,2,2,4,3,9,4,3,5,8,7,3,3,5,4,1,5,4,6,9,8,4,4,6,5,2,6,5,7,1,9,5,5,7,6,3,7,6,8,2,1,6,6,8,7,4],
            vec![4,7,1,5,1,4,2,6,7,1,5,8,2,6,2,5,3,7,8,2,6,9,3,7,3,6,4,8,9,3,7,1,4,8,4,7,5,9,1,4,8,2,5,9,5,8,6,1,2,5],
            vec![8,5,7,4,5,2,8,2,2,2,9,6,8,5,6,3,9,3,3,3,1,7,9,6,7,4,1,4,4,4,2,8,1,7,8,5,2,5,5,5,3,9,2,8,9,6,3,6,6,6],
            vec![2,4,2,1,2,3,9,2,4,8,3,5,3,2,3,4,1,3,5,9,4,6,4,3,4,5,2,4,6,1,5,7,5,4,5,6,3,5,7,2,6,8,6,5,6,7,4,6,8,3],
            vec![2,4,6,1,1,2,3,5,3,2,3,5,7,2,2,3,4,6,4,3,4,6,8,3,3,4,5,7,5,4,5,7,9,4,4,5,6,8,6,5,6,8,1,5,5,6,7,9,7,6],
            vec![4,2,3,6,5,3,2,7,4,1,5,3,4,7,6,4,3,8,5,2,6,4,5,8,7,5,4,9,6,3,7,5,6,9,8,6,5,1,7,4,8,6,7,1,9,7,6,2,8,5],
            vec![2,3,1,4,2,4,9,6,3,2,3,4,2,5,3,5,1,7,4,3,4,5,3,6,4,6,2,8,5,4,5,6,4,7,5,7,3,9,6,5,6,7,5,8,6,8,4,1,7,6],
            vec![3,4,2,2,1,5,5,6,9,2,4,5,3,3,2,6,6,7,1,3,5,6,4,4,3,7,7,8,2,4,6,7,5,5,4,8,8,9,3,5,7,8,6,6,5,9,9,1,4,6],
            vec![3,3,8,5,9,7,3,9,6,4,4,4,9,6,1,8,4,1,7,5,5,5,1,7,2,9,5,2,8,6,6,6,2,8,3,1,6,3,9,7,7,7,3,9,4,2,7,4,1,8],
            vec![3,5,1,3,5,9,5,8,9,4,4,6,2,4,6,1,6,9,1,5,5,7,3,5,7,2,7,1,2,6,6,8,4,6,8,3,8,2,3,7,7,9,5,7,9,4,9,3,4,8],
            vec![4,3,5,8,7,3,3,5,4,1,5,4,6,9,8,4,4,6,5,2,6,5,7,1,9,5,5,7,6,3,7,6,8,2,1,6,6,8,7,4,8,7,9,3,2,7,7,9,8,5],
            vec![5,8,2,6,2,5,3,7,8,2,6,9,3,7,3,6,4,8,9,3,7,1,4,8,4,7,5,9,1,4,8,2,5,9,5,8,6,1,2,5,9,3,6,1,6,9,7,2,3,6],
            vec![9,6,8,5,6,3,9,3,3,3,1,7,9,6,7,4,1,4,4,4,2,8,1,7,8,5,2,5,5,5,3,9,2,8,9,6,3,6,6,6,4,1,3,9,1,7,4,7,7,7],
            vec![3,5,3,2,3,4,1,3,5,9,4,6,4,3,4,5,2,4,6,1,5,7,5,4,5,6,3,5,7,2,6,8,6,5,6,7,4,6,8,3,7,9,7,6,7,8,5,7,9,4],
            vec![3,5,7,2,2,3,4,6,4,3,4,6,8,3,3,4,5,7,5,4,5,7,9,4,4,5,6,8,6,5,6,8,1,5,5,6,7,9,7,6,7,9,2,6,6,7,8,1,8,7],
            vec![5,3,4,7,6,4,3,8,5,2,6,4,5,8,7,5,4,9,6,3,7,5,6,9,8,6,5,1,7,4,8,6,7,1,9,7,6,2,8,5,9,7,8,2,1,8,7,3,9,6],
            vec![3,4,2,5,3,5,1,7,4,3,4,5,3,6,4,6,2,8,5,4,5,6,4,7,5,7,3,9,6,5,6,7,5,8,6,8,4,1,7,6,7,8,6,9,7,9,5,2,8,7],
            vec![4,5,3,3,2,6,6,7,1,3,5,6,4,4,3,7,7,8,2,4,6,7,5,5,4,8,8,9,3,5,7,8,6,6,5,9,9,1,4,6,8,9,7,7,6,1,1,2,5,7],
            vec![4,4,9,6,1,8,4,1,7,5,5,5,1,7,2,9,5,2,8,6,6,6,2,8,3,1,6,3,9,7,7,7,3,9,4,2,7,4,1,8,8,8,4,1,5,3,8,5,2,9],
            vec![4,6,2,4,6,1,6,9,1,5,5,7,3,5,7,2,7,1,2,6,6,8,4,6,8,3,8,2,3,7,7,9,5,7,9,4,9,3,4,8,8,1,6,8,1,5,1,4,5,9],
            vec![5,4,6,9,8,4,4,6,5,2,6,5,7,1,9,5,5,7,6,3,7,6,8,2,1,6,6,8,7,4,8,7,9,3,2,7,7,9,8,5,9,8,1,4,3,8,8,1,9,6],
            vec![6,9,3,7,3,6,4,8,9,3,7,1,4,8,4,7,5,9,1,4,8,2,5,9,5,8,6,1,2,5,9,3,6,1,6,9,7,2,3,6,1,4,7,2,7,1,8,3,4,7],
            vec![1,7,9,6,7,4,1,4,4,4,2,8,1,7,8,5,2,5,5,5,3,9,2,8,9,6,3,6,6,6,4,1,3,9,1,7,4,7,7,7,5,2,4,1,2,8,5,8,8,8],
            vec![4,6,4,3,4,5,2,4,6,1,5,7,5,4,5,6,3,5,7,2,6,8,6,5,6,7,4,6,8,3,7,9,7,6,7,8,5,7,9,4,8,1,8,7,8,9,6,8,1,5],
            vec![4,6,8,3,3,4,5,7,5,4,5,7,9,4,4,5,6,8,6,5,6,8,1,5,5,6,7,9,7,6,7,9,2,6,6,7,8,1,8,7,8,1,3,7,7,8,9,2,9,8],
            vec![6,4,5,8,7,5,4,9,6,3,7,5,6,9,8,6,5,1,7,4,8,6,7,1,9,7,6,2,8,5,9,7,8,2,1,8,7,3,9,6,1,8,9,3,2,9,8,4,1,7],
            vec![4,5,3,6,4,6,2,8,5,4,5,6,4,7,5,7,3,9,6,5,6,7,5,8,6,8,4,1,7,6,7,8,6,9,7,9,5,2,8,7,8,9,7,1,8,1,6,3,9,8],
            vec![5,6,4,4,3,7,7,8,2,4,6,7,5,5,4,8,8,9,3,5,7,8,6,6,5,9,9,1,4,6,8,9,7,7,6,1,1,2,5,7,9,1,8,8,7,2,2,3,6,8],
            vec![5,5,1,7,2,9,5,2,8,6,6,6,2,8,3,1,6,3,9,7,7,7,3,9,4,2,7,4,1,8,8,8,4,1,5,3,8,5,2,9,9,9,5,2,6,4,9,6,3,1],
            vec![5,7,3,5,7,2,7,1,2,6,6,8,4,6,8,3,8,2,3,7,7,9,5,7,9,4,9,3,4,8,8,1,6,8,1,5,1,4,5,9,9,2,7,9,2,6,2,5,6,1],
            vec![6,5,7,1,9,5,5,7,6,3,7,6,8,2,1,6,6,8,7,4,8,7,9,3,2,7,7,9,8,5,9,8,1,4,3,8,8,1,9,6,1,9,2,5,4,9,9,2,1,7],
            vec![7,1,4,8,4,7,5,9,1,4,8,2,5,9,5,8,6,1,2,5,9,3,6,1,6,9,7,2,3,6,1,4,7,2,7,1,8,3,4,7,2,5,8,3,8,2,9,4,5,8],
            vec![2,8,1,7,8,5,2,5,5,5,3,9,2,8,9,6,3,6,6,6,4,1,3,9,1,7,4,7,7,7,5,2,4,1,2,8,5,8,8,8,6,3,5,2,3,9,6,9,9,9],
            vec![5,7,5,4,5,6,3,5,7,2,6,8,6,5,6,7,4,6,8,3,7,9,7,6,7,8,5,7,9,4,8,1,8,7,8,9,6,8,1,5,9,2,9,8,9,1,7,9,2,6],
            vec![5,7,9,4,4,5,6,8,6,5,6,8,1,5,5,6,7,9,7,6,7,9,2,6,6,7,8,1,8,7,8,1,3,7,7,8,9,2,9,8,9,2,4,8,8,9,1,3,1,9],
            vec![7,5,6,9,8,6,5,1,7,4,8,6,7,1,9,7,6,2,8,5,9,7,8,2,1,8,7,3,9,6,1,8,9,3,2,9,8,4,1,7,2,9,1,4,3,1,9,5,2,8],
            vec![5,6,4,7,5,7,3,9,6,5,6,7,5,8,6,8,4,1,7,6,7,8,6,9,7,9,5,2,8,7,8,9,7,1,8,1,6,3,9,8,9,1,8,2,9,2,7,4,1,9],
            vec![6,7,5,5,4,8,8,9,3,5,7,8,6,6,5,9,9,1,4,6,8,9,7,7,6,1,1,2,5,7,9,1,8,8,7,2,2,3,6,8,1,2,9,9,8,3,3,4,7,9],
            ],
            multiply: 1,
        };

        for i in 0..map1.rows() {
            for j in 0..map1.cols() {
                assert_eq!(map1.get((i, j)), map2.get((i, j)));
            }
        }
    }

#[test]
    fn test_map_bigger_example() {
        let map = Map{
            data: vec![
                vec![1,1,6,3,7,5,1,7,4,2],
                vec![1,3,8,1,3,7,3,6,7,2],
                vec![2,1,3,6,5,1,1,3,2,8],
                vec![3,6,9,4,9,3,1,5,6,9],
                vec![7,4,6,3,4,1,7,1,1,1],
                vec![1,3,1,9,1,2,8,1,3,7],
                vec![1,3,5,9,9,1,2,4,2,1],
                vec![3,1,2,5,4,2,1,6,3,9],
                vec![1,2,9,3,1,3,8,5,2,1],
                vec![2,3,1,1,9,4,4,5,8,1],
            ],
            multiply: 5,
        };

        assert_eq!(a_star_search(map.clone(), (0, 0), (map.rows()-1, map.cols()-1)), 315);
    }

#[test]
    fn test_map() {
        let mut map = Map{data: MapData::new(), multiply: 5};
        map.data.push(vec![8, 7]);

        assert_eq!(map.rows(), 5);
        assert_eq!(map.cols(), 10);

        assert_eq!(map.get((0,0)), 8);
        assert_eq!(map.get((0,1)), 7);

        assert_eq!(map.get((0,2)), 9);
        assert_eq!(map.get((0,3)), 8);
        assert_eq!(map.get((1,0)), 9);
        assert_eq!(map.get((1,1)), 8);

    }
}
