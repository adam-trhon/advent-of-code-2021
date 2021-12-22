use regex::Regex;
use std::ops::Range;

struct RebootStep {
    on: bool,
    startx: i32,
    endx: i32,
    starty: i32,
    endy: i32,
    startz: i32,
    endz: i32
}

fn parse_line(line: &str) -> RebootStep {
    let re = Regex::new(r"(\S+) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)").unwrap();
    let capt = re.captures(line.trim()).unwrap();
    let state = &capt[1];
    let startx = capt[2].parse::<i32>().unwrap();
    let endx = capt[3].parse::<i32>().unwrap();
    let starty = capt[4].parse::<i32>().unwrap();
    let endy = capt[5].parse::<i32>().unwrap();
    let startz = capt[6].parse::<i32>().unwrap();
    let endz = capt[7].parse::<i32>().unwrap();

    RebootStep{
        on: state == "on", 
        startx: startx,
        endx: endx,
        starty: starty,
        endy: endy,
        startz: startz,
        endz: endz,
    }
}

fn parse_input(text: &str) -> Vec<RebootStep> {
    text.split("\n").filter(|l| !l.is_empty()).map(|l| parse_line(l)).collect::<Vec<RebootStep>>()
}

fn step_in_range(step: &RebootStep) -> bool {
    let check = |n: i32| n >= -50 && n <= 50;
    let _ = check(step.startx) || return false;
    let _ = check(step.endx) || return false;
    let _ = check(step.starty) || return false;
    let _ = check(step.endy) || return false;
    let _ = check(step.startz) || return false;
    let _ = check(step.endz) || return false;
    true
}

const REACTOR_SIDE: usize = 101;
const REACTOR_SIZE: usize = REACTOR_SIDE * REACTOR_SIDE * REACTOR_SIDE;
struct Reactor {
    cubes: [bool; REACTOR_SIZE]
}

impl Reactor {
    fn new() -> Reactor {
        Reactor{cubes: [false; REACTOR_SIZE] } 
    }

    fn apply_step( &mut self, step: &RebootStep) {
        let to_range = |s: i32, e: i32| Range::<usize>{start: (s + 50) as usize, end: (e + 51) as usize};
        for x in to_range(step.startx, step.endx) {
            for y in to_range(step.starty, step.endy) {
                for z in to_range(step.startz, step.endz) {
                    let mut index = x;
                    index = index * REACTOR_SIDE + y;
                    index = index * REACTOR_SIDE + z;
                    self.cubes[index] = step.on;
                }
            }
        }
    }

    fn count_on(&self) -> usize {
        self.cubes.iter().filter(|v| **v).count()
    }
}

fn main() {
    let text = std::fs::read_to_string("input.txt").expect("cannot read input.txt");
    let steps = parse_input(&text).into_iter().filter(|s| step_in_range(s)).collect::<Vec<RebootStep>>();
    let mut reactor = Reactor::new();
    for step in steps.iter() {
        reactor.apply_step(step);
    }
    println!("{}", reactor.count_on());
}
