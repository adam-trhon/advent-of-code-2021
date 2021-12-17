use std::fs;
use regex::Regex;
use std::collections::HashSet;

#[derive(Copy, Clone)]
struct TargetArea {
    beg_x: i32,
    end_x: i32,
    beg_y: i32,
    end_y: i32,
}

fn parse_input(text: &str) -> TargetArea {

    let re = Regex::new(r"target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)").unwrap();
    let m = re.captures(text).unwrap();

    TargetArea {
        beg_x: m[1].parse().unwrap(),
        end_x: m[2].parse().unwrap(),
        beg_y: m[3].parse().unwrap(),
        end_y: m[4].parse().unwrap(),
    }
}

fn shoot(mut dx: i32, mut dy: i32, target: &TargetArea) -> bool {
    let mut pos_x: i32 = 0;
    let mut pos_y: i32 = 0;

    loop {
        pos_x += dx;
        pos_y += dy;

        if pos_x > target.end_x {
            return false;
        }
        if pos_y < target.beg_y {
            return false;
        }
        if pos_x >= target.beg_x && pos_y <= target.end_y {
            return true;
        }

        if dx > 0 {
            dx -= 1;
        }
        dy -= 1;
    }

}

fn count_shots(target: &TargetArea) -> usize {
    gen_succ_shots(target).len()
}

fn gen_succ_shots(target: &TargetArea) -> HashSet<(i32, i32)> {
    let mut result: HashSet<(i32, i32)> = HashSet::new();

    for x in 0..target.end_x + 1 {
        for y in target.beg_y..-target.beg_y {
            if shoot(x, y, &target) {
                result.insert((x, y));
            }
        }
    }

    result
}

fn main() {
    let text = fs::read_to_string("input.txt").expect("cannot read input.txt");
    let target_area = parse_input(&text);
    println!("{}", gen_succ_shots(&target_area).len());
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn test_examples() {
        let target = parse_input("target area: x=20..30, y=-10..-5");
        assert_eq!(shoot(7, 2, &target), true);
        assert_eq!(shoot(6, 3, &target), true);
        assert_eq!(shoot(9, 0, &target), true);
        assert_eq!(shoot(17, -4, &target), false);
    }

#[test]
    fn test_random_miss() {
        let target = parse_input("target area: x=20..30, y=-10..-5");
        assert_eq!(shoot(21, -2, &target), false);
    }

#[test]
    fn test_gen_shots() {
        let target = parse_input("target area: x=20..30, y=-10..-5");

        let expected: HashSet<(i32, i32)> = HashSet::from([
            (23,-10),  (25,-9),   (27,-5),   (29,-6),   (22,-6),   (21,-7),   (9,0),     (27,-7),   (24,-5),
            (25,-7),   (26,-6),   (25,-5),   (6,8),     (11,-2),   (20,-5),   (29,-10),  (6,3),     (28,-7),
            (8,0),     (30,-6),   (29,-8),   (20,-10),  (6,7),     (6,4),     (6,1),     (14,-4),   (21,-6),
            (26,-10),  (7,-1),    (7,7),     (8,-1),    (21,-9),   (6,2),     (20,-7),   (30,-10),  (14,-3),
            (20,-8),   (13,-2),   (7,3),     (28,-8),   (29,-9),   (15,-3),   (22,-5),   (26,-8),   (25,-8),
            (25,-6),   (15,-4),   (9,-2),    (15,-2),   (12,-2),   (28,-9),   (12,-3),   (24,-6),   (23,-7),
            (25,-10),  (7,8),     (11,-3),   (26,-7),   (7,1),     (23,-9),   (6,0),     (22,-10),  (27,-6),
            (8,1),     (22,-8),   (13,-4),   (7,6),     (28,-6),   (11,-4),   (12,-4),   (26,-9),   (7,4),
            (24,-10),  (23,-8),   (30,-8),   (7,0),     (9,-1),    (10,-1),   (26,-5),   (22,-9),   (6,5),
            (7,5),     (23,-6),   (28,-10),  (10,-2),   (11,-1),   (20,-9),   (14,-2),   (29,-7),   (13,-3),
            (23,-5),   (24,-8),   (27,-9),   (30,-7),   (28,-5),   (21,-10),  (7,9),     (6,6),     (21,-5),
            (27,-10),  (7,2),     (30,-9),   (21,-8),   (22,-7),   (24,-9),   (20,-6),   (6,9),     (29,-5),
            (8,-2),    (27,-8),   (30,-5),   (24,-7),
        ]);

        assert_eq!(gen_succ_shots(&target).difference(&expected).collect::<HashSet<&(i32,i32)>>(), HashSet::new());

    }

#[test]
    fn test_count_example() {
        let target = parse_input("target area: x=20..30, y=-10..-5");
        assert_eq!(count_shots(&target), 112);
    }
}
