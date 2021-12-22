use std::fs;
use regex::Regex;

fn load_input(text: &str) -> (usize, usize) {
    let mut result: Vec<usize> = Vec::new();

    let line_re = Regex::new(r"Player (\d+) starting position: (\d+)").unwrap();
    for line in text.split("\n") {
        if line.len() == 0 {
            continue;
        }

        let capts = line_re.captures(line).expect("cannot match the line");
        result.push(capts[2].parse().unwrap());

    }

    (result[0], result[1])
}

struct Dice {
    current: usize,
    throws: usize,
}

impl Dice {
    fn new() -> Dice {
        Dice{current: 1, throws: 0}
    }

    fn throw(&mut self) -> usize {
        let result = self.current;
        if self.current == 100 {
            self.current = 1;
        } else {
            self.current += 1;
        }
        self.throws += 1;

        result
    }

    fn throw_n_times(&mut self, n: usize) -> usize {
        let mut result: usize = 0;
        for _ in 0..n {
            result += self.throw();
        }
        result
    }

}

fn advance_by(position: usize, by: usize) -> usize {
    (position - 1 + by) % 10 + 1
}

struct Player {
    position: usize,
    score: usize,
}

impl Player {
    fn new(position: usize) -> Player {
        Player{position: position, score: 0}
    }

    fn turn(&mut self, dice: &mut Dice) {
        self.position = advance_by(self.position, dice.throw_n_times(3));
        self.score += self.position;
    }
}

fn run_game(p1_pos: usize, p2_pos: usize) -> usize {
    let mut p1: Player = Player::new(p1_pos);
    let mut p2: Player = Player::new(p2_pos);
    let mut dice: Dice = Dice::new();

    let mut result: usize;

    loop {
        p1.turn(&mut dice);
        if p1.score > 1000 {
            result = p2.score * dice.throws;
            break;
        }
        p2.turn(&mut dice);
        if p2.score > 1000 {
            result = p1.score * dice.throws;
            break;
        }
    }

    result
}

fn main() {
    let text = fs::read_to_string("input.txt").expect("cannot read input file");
    let input = load_input(&text);
    println!("{}", run_game(input.0, input.1));
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn test_advance_by() {
        assert_eq!(advance_by(7, 5), 2);
        assert_eq!(advance_by(7, 15), 2);
    }
}
