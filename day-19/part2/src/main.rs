use std::fs;
use std::collections::HashSet;

type Scan = HashSet<Vec<i32>>;

fn combine(l: [[i32; 3]; 3], r: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut result: [[i32; 3]; 3] = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                result[i][j] += l[i][k] * r[k][j];
            }
        }
    }
    result
}

fn rotation(n: usize) -> [[i32; 3]; 3] {
    match n {
        0 => [ [ 1,  0,  0], [ 0,  1,  0], [ 0,  0,  1] ],
        1 => [ [ 0,  1,  0], [-1,  0,  0], [ 0,  0,  1] ],
        2 => combine(rotation(1), rotation(1)),
        3 => combine(rotation(2), rotation(1)),
        4 => [ [ 1,  0,  0], [ 0,  0, -1], [ 0,  1,  0] ],
        r @ 5..=7 => combine(rotation(r-1), rotation(1)),
        8 => combine(rotation(4), rotation(4)),
        r @ 9..=11 => combine(rotation(r-1), rotation(1)),
        12 => combine(rotation(8), rotation(4)),
        r @ 13..=15 => combine(rotation(r-1), rotation(1)),
        16 => [ [ 0,  0, -1], [ 0,  1,  0], [ 1,  0,  0] ],
        r @ 17..=19 => combine(rotation(r-1), rotation(1)),
        20 => [ [ 0,  0,  1], [ 0,  1,  0], [-1,  0,  0] ],
        r @ 21..=23 => combine(rotation(r-1), rotation(1)),
        _ => panic!("not supported")
    }
}

fn apply(transformation: &[[i32; 3]; 3], vec: &[i32]) -> Vec<i32> {
    assert_eq!(vec.len(), 3);
    let mut result: Vec<i32> = Vec::new();
    for i in 0..3 {
        let mut new_val: i32 = 0;
        for j in 0..3 {
            new_val += vec[j] * transformation[j][i];
        }
        result.push(new_val);
    }
    result
}

fn load_input(text: &str) -> Vec<Scan>  {
    let mut result: Vec<Scan> = Vec::new();
    let mut current: Option<Scan> = None;

    for line in text.split("\n") {
        if line.is_empty() {
            continue;
        }
        if &line[..3] == "---" {
            match current.replace(Scan::new()) {
                None => (),
                Some(scan) => result.push(scan),
            }
        } else {
            match current {
                None => panic!("Option is none"),
                Some(ref mut scan) => {
                    scan.insert(line.split(",").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>());
                }
            }
        }
    }
    match current {
        None => (),
        Some(scan) => result.push(scan),
    }

    result
}

fn add_vects(l: &Vec<i32>, r: &Vec<i32>) -> Vec<i32> {
    l.iter().zip(r.iter()).map(|(lv, rv)| lv+rv).collect::<Vec<i32>>()
}

fn random_move_to(scan: Scan, reference: &Scan) -> (Vec<i32>, Scan) {
    let scan_i = rand::random::<usize>() % scan.len();
    let reference_i = rand::random::<usize>() % reference.len();

    let scan_ref = scan.iter().nth(scan_i).unwrap();
    let reference_ref = reference.iter().nth(reference_i).unwrap();

    let diff = scan_ref.iter().zip(reference_ref.iter()).map(|(s, r)| r - s).collect::<Vec<i32>>();
    let diff_result = diff.clone();

    let shift = |e: Vec<i32>| e.iter().zip(diff.iter()).map(|(l, r)| l + r).collect::<Vec<i32>>();

    (diff_result, scan.into_iter().map(|e| shift(e)).collect::<Scan>())
}

fn count_overlap(l: &Scan, r: &Scan) -> usize {
    l.intersection(r).collect::<Vec<&Vec<i32>>>().len()
}

fn random_match(scan: &Scan, reference: &Scan) -> Option<(Vec<i32>, Scan)> {
    for i in 0..24 {
        let rotated: Scan = scan.iter().map(|s| apply(&rotation(i), &s)).collect();
        for _ in 0..20 {
            let (diff, moved) = random_move_to(rotated.clone(), reference);
            if count_overlap(&moved, reference) >= 12 {
                return Some((diff, moved));
            }
        }
    }
    return None;
}

fn unify_scans(mut scans: Vec<Scan>) -> Vec<(Vec<i32>, Scan)> {
    let mut known: Vec<(Vec<i32>, Scan)> = Vec::new();
    let mut scan_iter = scans.into_iter();
    known.push((vec![0, 0, 0], scan_iter.next().unwrap()));
    scans = scan_iter.collect();

    while !scans.is_empty() {
        println!("New major loop, {} remaining", scans.len());
        scan_iter = scans.into_iter();
        scans = Vec::new();
        'scans: for scan in scan_iter {
            for (pos, k) in known.iter() {
                match random_match(&scan, k) {
                    Some((diff, moved)) => {
                        println!("Inserted a scan!");
                        known.push((diff, moved));
                        continue 'scans;
                    }
                    None => (),
                }
            }
            scans.push(scan);
        }
    }

    known
}

fn manhattan_distance(l: &Vec<i32>, r: &Vec<i32>) -> i32 {
    l.iter().zip(r.iter()).map(|(l, r)| (l-r).abs()).sum()
}

fn max_pos_distance(scans: &Vec<(Vec<i32>, Scan)> ) -> i32 {
    let mut max_distance: i32 = 0;
    for i in 0..scans.len() {
        for j in 0..scans.len() {
            let distance = manhattan_distance(&scans[i].0, &scans[j].0);
            if distance > max_distance {
                max_distance = distance;
            }
        }
    }
    max_distance
}

fn main() {
    let text = fs::read_to_string("input.txt").expect("cannot read input.txt");
    let input = load_input(&text);
    let known = unify_scans(input);
    println!("{}", max_pos_distance(&known));
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn test_transform() {
        // z to +z
        assert_eq!(apply(&rotation( 0), &[1, 2, 3]), [ 1,  2,  3]);
        assert_eq!(apply(&rotation( 1), &[1, 2, 3]), [-2,  1,  3]);
        assert_eq!(apply(&rotation( 2), &[1, 2, 3]), [-1, -2,  3]);
        assert_eq!(apply(&rotation( 3), &[1, 2, 3]), [ 2, -1,  3]);

        // z to +y
        assert_eq!(apply(&rotation( 4), &[1, 2, 3]), [ 1,  3, -2]);
        assert_eq!(apply(&rotation( 5), &[1, 2, 3]), [-3,  1, -2]);
        assert_eq!(apply(&rotation( 6), &[1, 2, 3]), [-1, -3, -2]);
        assert_eq!(apply(&rotation( 7), &[1, 2, 3]), [ 3, -1, -2]);

        // z to -z
        assert_eq!(apply(&rotation( 8), &[1, 2, 3]), [ 1, -2, -3]);
        assert_eq!(apply(&rotation( 9), &[1, 2, 3]), [ 2,  1, -3]);
        assert_eq!(apply(&rotation(10), &[1, 2, 3]), [-1,  2, -3]);
        assert_eq!(apply(&rotation(11), &[1, 2, 3]), [-2, -1, -3]);

        // z to -y
        assert_eq!(apply(&rotation(12), &[1, 2, 3]), [ 1, -3,  2]);
        assert_eq!(apply(&rotation(13), &[1, 2, 3]), [ 3,  1,  2]);
        assert_eq!(apply(&rotation(14), &[1, 2, 3]), [-1,  3,  2]);
        assert_eq!(apply(&rotation(15), &[1, 2, 3]), [-3, -1,  2]);

        // z to +x
        assert_eq!(apply(&rotation(16), &[1, 2, 3]), [ 3,  2, -1]);
        assert_eq!(apply(&rotation(17), &[1, 2, 3]), [-2,  3, -1]);
        assert_eq!(apply(&rotation(18), &[1, 2, 3]), [-3, -2, -1]);
        assert_eq!(apply(&rotation(19), &[1, 2, 3]), [ 2, -3, -1]);

        // z to -x
        assert_eq!(apply(&rotation(20), &[1, 2, 3]), [-3,  2,  1]);
        assert_eq!(apply(&rotation(21), &[1, 2, 3]), [-2, -3,  1]);
        assert_eq!(apply(&rotation(22), &[1, 2, 3]), [ 3, -2,  1]);
        assert_eq!(apply(&rotation(23), &[1, 2, 3]), [ 2,  3,  1]);
    }

#[test]
    fn test_load_input() {
        let text = vec![
            "--- scanner 0 ---",
            "404,-588,-901",
            "528,-643,409",
            "-838,591,734",
            "390,-675,-793",
            "-537,-823,-458",
            "-485,-357,347",
            "-345,-311,381",
            "-661,-816,-575",
            "-876,649,763",
            "-618,-824,-621",
            "553,345,-567",
            "474,580,667",
            "-447,-329,318",
            "-584,868,-557",
            "544,-627,-890",
            "564,392,-477",
            "455,729,728",
            "-892,524,684",
            "-689,845,-530",
            "423,-701,434",
            "7,-33,-71",
            "630,319,-379",
            "443,580,662",
            "-789,900,-551",
            "459,-707,401",
            "",
            "--- scanner 1 ---",
            "686,422,578",
            "605,423,415",
            "515,917,-361",
            "-336,658,858",
            "95,138,22",
            "-476,619,847",
            "-340,-569,-846",
            "567,-361,727",
            "-460,603,-452",
            "669,-402,600",
            "729,430,532",
            "-500,-761,534",
            "-322,571,750",
            "-466,-666,-811",
            "-429,-592,574",
            "-355,545,-477",
            "703,-491,-529",
            "-328,-685,520",
            "413,935,-424",
            "-391,539,-444",
            "586,-435,557",
            "-364,-763,-893",
            "807,-499,-711",
            "755,-354,-619",
            "553,889,-390",
            "",
        ].iter().map(|s| s.to_string()).collect::<Vec<String>>().join("\n");

        let input = load_input(&text);

        let scan0 = vec![
            vec![404,-588,-901],
            vec![528,-643,409],
            vec![-838,591,734],
            vec![390,-675,-793],
            vec![-537,-823,-458],
            vec![-485,-357,347],
            vec![-345,-311,381],
            vec![-661,-816,-575],
            vec![-876,649,763],
            vec![-618,-824,-621],
            vec![553,345,-567],
            vec![474,580,667],
            vec![-447,-329,318],
            vec![-584,868,-557],
            vec![544,-627,-890],
            vec![564,392,-477],
            vec![455,729,728],
            vec![-892,524,684],
            vec![-689,845,-530],
            vec![423,-701,434],
            vec![7,-33,-71],
            vec![630,319,-379],
            vec![443,580,662],
            vec![-789,900,-551],
            vec![459,-707,401],
        ].into_iter().collect::<Scan>();
        assert_eq!(input[0], scan0);

        let scan1 = vec![
            vec![686,422,578],
            vec![605,423,415],
            vec![515,917,-361],
            vec![-336,658,858],
            vec![95,138,22],
            vec![-476,619,847],
            vec![-340,-569,-846],
            vec![567,-361,727],
            vec![-460,603,-452],
            vec![669,-402,600],
            vec![729,430,532],
            vec![-500,-761,534],
            vec![-322,571,750],
            vec![-466,-666,-811],
            vec![-429,-592,574],
            vec![-355,545,-477],
            vec![703,-491,-529],
            vec![-328,-685,520],
            vec![413,935,-424],
            vec![-391,539,-444],
            vec![586,-435,557],
            vec![-364,-763,-893],
            vec![807,-499,-711],
            vec![755,-354,-619],
            vec![553,889,-390],
        ].into_iter().collect::<Scan>();
        assert_eq!(input[1], scan1);
    }
}
