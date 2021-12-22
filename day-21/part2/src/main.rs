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


fn advance_by(position: usize, by: usize) -> usize {
    (position - 1 + by) % 10 + 1
}

const MAX_POS: usize = 11;
const MAX_SCORE: usize = 21;
const UNIVERSE_COUNT: usize = MAX_POS * MAX_SCORE * MAX_POS * MAX_SCORE;
struct Multiverse {
    counts : [u128; UNIVERSE_COUNT]
}

impl Multiverse {
    fn new() -> Multiverse {
        Multiverse{counts: [0; UNIVERSE_COUNT]}
    }
}

fn index_of(p1_pos: usize, p1_score: usize, p2_pos: usize, p2_score: usize) -> usize{
    let mut index: usize = p1_pos;
    index = index * MAX_SCORE + p1_score;
    index = index * MAX_POS + p2_pos;
    index = index * MAX_SCORE + p2_score;
    index
}

fn generate_positions(pos: usize) -> [usize; 27] {
    let mut result: [usize; 27] = [0; 27];
    for i in 1..=3 {
        for j in 1..=3 {
            for k in 1..=3 {
                result[(i-1)*9 + (j-1)*3 + k-1] = advance_by(pos, i+j+k);
            }
        }
    }
    result
}

#[derive(Debug,PartialEq)]
struct PlayerData {
    pos: usize,
    score: usize,
    count: u128,
}

fn turn(player: PlayerData) -> (u128, Vec<PlayerData>) {
    let mut winning: u128 = 0;
    let mut continuing: Vec<PlayerData> = Vec::new();

    for pos in generate_positions(player.pos) {
        let new_player = PlayerData {
            pos: pos,
            score: player.score + pos,
            count: player.count,
        };
        if new_player.score > 20 {
            winning += new_player.count;
        } else {
            continuing.push(new_player);
        }
    }
    (winning, continuing)
}

fn run_game(p1_init: usize, p2_init: usize) -> u128 {
    let mut multiverse = Multiverse::new();
    multiverse.counts[index_of(p1_init, 0, p2_init, 0)] = 1;

    let mut p1_wins: u128 = 0;

    let mut p1_playing = true;

    loop {
        let mut next_multiverse = Multiverse::new();
        let mut game_done = true;

        for p1_pos in 1..MAX_POS {
            for p1_score in 0..MAX_SCORE {
                for p2_pos in 1..MAX_POS {
                    for p2_score in 0..MAX_SCORE {
                        let index = index_of(p1_pos, p1_score, p2_pos, p2_score);
                        let u_count = multiverse.counts[index];

                        if u_count == 0 {
                            continue;
                        }

                        if p1_playing {
                            let (win, cont) = turn(PlayerData{pos: p1_pos, score:p1_score, count: u_count});
                            p1_wins += win;
                            for c in cont {
                                game_done = false;
                                let next_index = index_of(c.pos, c.score, p2_pos, p2_score);
                                next_multiverse.counts[next_index] += c.count;
                            }
                        } else {
                            let (_, cont) = turn(PlayerData{pos: p2_pos, score:p2_score, count: u_count});
                            for c in cont {
                                game_done = false;
                                let next_index = index_of(p1_pos, p1_score, c.pos, c.score);
                                next_multiverse.counts[next_index] += c.count;
                            }
                        }

                    }
                }
            }
        }

        multiverse = next_multiverse;

        if game_done {
            break;
        }

        p1_playing = !p1_playing;
    }

    p1_wins
    
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

#[test]
    fn test_generate_positions() {
        assert_eq!(generate_positions(5), [
             8,  9, 10,
             9, 10,  1,
            10,  1,  2,
             9, 10,  1,
            10,  1,  2,
             1,  2,  3,
            10,  1,  2,
             1,  2,  3,
             2,  3,  4,
        ]);
    }
}
