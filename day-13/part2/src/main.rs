use std::fs;
use std::collections::HashSet;

type Dot = (usize, usize);
type DotSet = HashSet<Dot>;
struct Input {
    dots: DotSet,
    x_folds: Vec<usize>,
    y_folds: Vec<usize>
}

fn parse_input(text: &str) -> Input {
    let mut input: Input = Input{dots: HashSet::new(), x_folds: vec![], y_folds: vec![]};

    let mut lines = text.split("\n");
    loop {
        match lines.next() {
            None => panic!("File ended too soon"),
            Some("") => break,
            Some(point) => {
                if let [x, y] = point.split(",").collect::<Vec<&str>>()[..] {
                    input.dots.insert((x.parse().unwrap(), y.parse().unwrap()));
                } else {
                    panic!("Invalid number of numbers on line");
                }
            }
        }
    }

    loop {
        match lines.next() {
            None => break,
            Some("") => continue,
            Some(fold) => {
                if let [_fold, _along, axis, pos] = fold.split(&[' ', '='][..]).collect::<Vec<&str>>()[..] {
                    let pos_val = pos.parse().unwrap();
                    if axis == "x" {
                        input.x_folds.push(pos_val);
                    } else if axis == "y" {
                        input.y_folds.push(pos_val);
                    } else {
                        panic!("invalid fold axis");
                    }
                } else {
                    panic!("Invalid fold line '{}' ", fold);
                }
            }
        }
    }

    input
}

fn fold(mut input: Input) -> Input {
    for fold in input.x_folds.iter() {
        let mut new_dots = input.dots.drain().collect::<DotSet>();
        for (mut x, y) in new_dots.drain() {
            if x > *fold {
                x = 2*fold-x;
            }
            input.dots.insert((x,y));
        }
    }

    for fold in input.y_folds.iter() {
        let mut new_dots = input.dots.drain().collect::<DotSet>();
        for (x, mut y) in new_dots.drain() {
            if y > *fold {
                y = 2*fold-y;
            }
            input.dots.insert((x,y));
        }
    }

    input
}

fn print_output(input: &Input) {
    let hx = *input.dots.iter().map(|(x, _)| x).max().unwrap() + 1;
    let hy = *input.dots.iter().map(|(_, y)| y).max().unwrap() + 1;

    let mut printed = vec![vec![' '; hx]; hy];

    for (x,y) in input.dots.iter() {
        printed[*y][*x] = '#';
    }

    for line in printed.iter() {
        for c in line.iter() {
            print!("{}", c);
        }
        print!("{}", '\n');
    }
}

fn main() {
    let text = fs::read_to_string("input.txt").expect("cannot load input file");
    let input = parse_input(&text);
    let folded_input = fold(input);
    print_output(&folded_input);
}
