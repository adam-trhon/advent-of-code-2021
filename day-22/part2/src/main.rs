use regex::Regex;
use std::collections::HashSet;

#[derive(Hash,Eq,PartialEq,Clone,Debug)]
struct Edge {
    start: i32,
    end: i32
}

impl Edge {
    fn new(start: i32, end: i32) -> Edge {
        Edge{start: start, end: end}
    }
    fn covers(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps_with(&self, other: &Self) -> bool {
        self.end >= other.start && self.start <= other.end   
    }

    fn cut_by(&self, other:& Self) -> Vec<Edge> {
        let mut result: Vec<Edge> = Vec::new();

        if other.covers(self) || !self.overlaps_with(other){
            result.push(self.clone());
        } else {
            let mut remaining: Edge = self.clone();
            if remaining.start < other.start {
                result.push(Edge::new(remaining.start, other.start-1));
                remaining.start = other.start;
            }

            if other.end < remaining.end {
                result.push(Edge::new(remaining.start, other.end));
                remaining.start = other.end + 1;
            }

            result.push(remaining);
        }

        result
    }

    fn size(&self) -> u64 {
        (self.end - self.start + 1) as u64
    }
}

#[cfg(test)]
mod edge_tests {
    use super::*;

#[test]
    fn test_cuts() {
        let base = Edge::new(5, 10);
        let edge = | s:i32, e:i32| Edge::new(s, e);
        assert_eq!(base.cut_by(&edge(1, 4)), vec![edge(5, 10)]);
        assert_eq!(base.cut_by(&edge(1, 5)), vec![edge(5, 5), edge(6, 10)]);
        assert_eq!(base.cut_by(&edge(1, 8)), vec![edge(5, 8), edge(9, 10)]);
        assert_eq!(base.cut_by(&edge(1, 10)), vec![edge(5,10)]);
        assert_eq!(base.cut_by(&edge(1, 15)), vec![edge(5,10)]);
        assert_eq!(base.cut_by(&edge(5, 8)), vec![edge(5, 8), edge(9, 10)]);
        assert_eq!(base.cut_by(&edge(6, 8)), vec![edge(5, 5), edge(6, 8), edge(9, 10)]);
        assert_eq!(base.cut_by(&edge(6, 10)), vec![edge(5, 5), edge(6, 10)]);
        assert_eq!(base.cut_by(&edge(6, 15)), vec![edge(5, 5), edge(6, 10)]);
        assert_eq!(base.cut_by(&edge(10, 15)), vec![edge(5, 9), edge(10, 10)]);
        assert_eq!(base.cut_by(&edge(11, 15)), vec![edge(5, 10)]);
    }
}


#[derive(Hash,Eq,PartialEq,Clone)]
struct ReactorSegment {
    x: Edge,
    y: Edge,
    z: Edge,
}

impl ReactorSegment {

    fn new(x: Edge, y: Edge, z: Edge) -> ReactorSegment {
        ReactorSegment{x: x, y: y, z: z}
    }

    fn covers(&self, other: &Self) -> bool {
        self.x.covers(&other.x) && self.y.covers(&other.y) && self.z.covers(&other.z)
    }

    fn overlaps_with(&self, other: &Self) -> bool {
        self.x.overlaps_with(&other.x) || self.y.overlaps_with(&other.y) || self.z.overlaps_with(&other.z)
    }

    fn cut_by(&self, other: &Self) -> HashSet<Self> {
        let mut result: HashSet<Self> = HashSet::new();

        let xs = self.x.cut_by(&other.x);
        let ys = self.y.cut_by(&other.y);
        let zs = self.z.cut_by(&other.z);

        for x in xs.iter() {
            for y in ys.iter() {
                for z in zs.iter() {
                    result.insert(Self::new(x.clone(), y.clone(), z.clone()));
                }
            }
        }

        result
    }

    fn remove_subsegment(self, other: &Self) -> HashSet<Self> {
        let mut result : HashSet<Self> = HashSet::new();
        if other.covers(&self) {
        } else if !self.overlaps_with(other) {
            result.insert(self);
        } else {
            let it_generated = self.cut_by(other).into_iter().filter(|s| !other.covers(&s)); 
            let mut generated = it_generated.collect::<Vec<Self>>();

            while !generated.is_empty() {
                let mut current = generated[0].clone();
                generated.remove(0);
                let mut i: usize = 0;
                while i < generated.len() {
                    match current.try_join(&generated[i]) {
                        Some(join) => {
                            current = join;
                            generated.remove(i);
                        }
                        None => {
                            i += 1;
                        }
                    }
                }
                result.insert(current);
            }

/*
            while !generated.is_empty() {
                let mut current = generated.drain().unwrap();
                result.insert(current);
            }
            */

            //result.extend(generated.drain());
        }
        result
    }

    fn size(&self) -> u64 {
        self.x.size() * self.y.size() * self.z.size()
    }

    fn try_join(&self, other: &Self) -> Option<Self> {
        let mut result: Self = self.clone();

        if result.x == other.x && result.y == other.y && result.z.end + 1 == other.z.start {
            result.z.end = other.z.end;
        } else if result.x == other.x && result.y == other.y && other.z.end + 1 == result.z.start {
            result.z.start = other.z.start;
        } else if result.x == other.x && result.y.end + 1 == other.y.start && result.z == other.z {
            result.y.end = other.y.end;
        } else if result.x == other.x && other.y.end + 1 == result.y.start && result.z == other.z {
            result.y.start = other.y.start;
        } else if result.x.end + 1 == other.x.start && result.y == other.y && result.z == other.z {
            result.x.end = other.x.end;
        } else if other.x.end + 1 == result.x.start && result.y == other.y && result.z == other.z {
            result.x.start = other.x.start;
        } else {
            return None;
        }
        Some(result)
    }

}

struct RebootStep {
    on: bool,
    segment: ReactorSegment,
}

struct Parser {
    re: Regex
}

impl Parser {
    fn new() -> Parser {
        Parser {re: Regex::new(r"(\S+) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)").unwrap()}
    }

    fn parse(&self, line:&str) -> RebootStep {
        let capt = self.re.captures(line.trim()).unwrap();
        let state = &capt[1];
        let startx = capt[2].parse::<i32>().unwrap();
        let endx = capt[3].parse::<i32>().unwrap();
        let starty = capt[4].parse::<i32>().unwrap();
        let endy = capt[5].parse::<i32>().unwrap();
        let startz = capt[6].parse::<i32>().unwrap();
        let endz = capt[7].parse::<i32>().unwrap();

        RebootStep{
            on: state == "on", 
            segment: ReactorSegment{
                x: Edge {
                    start: startx,
                    end: endx,
                },
                y: Edge {
                    start: starty,
                    end: endy,
                },
                z: Edge {
                    start: startz,
                    end: endz,
                },
            },
        }

    }
}

struct Reactor {
    segments: HashSet<ReactorSegment>
}

impl Reactor {
    fn new() -> Reactor {
        Reactor { segments: HashSet::new() }
    }

    fn turn_on(&mut self, segment: &ReactorSegment) {
        self.turn_off(&segment);
        self.segments.insert(segment.clone());
    }

    fn turn_off(&mut self, segment: &ReactorSegment) {
        let iter = self.segments.drain();
        let mut new_segments = HashSet::new();

        for current in iter {
            new_segments.extend(current.remove_subsegment(segment).into_iter());
        }

        self.segments.extend(new_segments.drain());
    }

    fn count_active(&self) -> u64 {
        self.segments.iter().map(|s| s.size()).sum()
    }
}

fn parse_input(text: &str) -> Vec<RebootStep> {
    let parser = Parser::new();
    text.split("\n").filter(|l| !l.is_empty()).map(|l| parser.parse(l)).collect::<Vec<RebootStep>>()
}

fn run_sequence(reactor: &mut Reactor, steps: Vec<RebootStep>) {
    for (i, step) in steps.iter().enumerate() {
        println!("{}/{} inputs, {} segments", i, steps.len(), reactor.segments.len());
        if step.on {
            reactor.turn_on(&step.segment);
        } else {
            reactor.turn_off(&step.segment);
        }
    }
}

fn main() {
    let text = std::fs::read_to_string("input.txt").expect("cannot read input.txt");
    let steps = parse_input(&text);
    let mut reactor = Reactor::new();
    run_sequence(&mut reactor, steps);
    println!("{}", reactor.count_active());
}

