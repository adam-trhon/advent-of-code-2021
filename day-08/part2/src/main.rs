use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;

type Digit = HashSet<char>;
type InputLine = (Vec<Digit>, Vec<Digit>);
type DetectionRules = Vec<(usize, usize, usize)>;

fn parse_input(input: &str) -> Vec<InputLine> {
    let mut result : Vec<InputLine> = Vec::new();

    for line in input.trim().split("\n") {
        let mut line_split = line.split(" | ");
        let left : &str = line_split.next().expect("failed to read left");
        let right : &str = line_split.next().expect("failed to read right");

        let mut left_vec : Vec<HashSet<char>> = Vec::new();
        left_vec.extend(left.trim().split(" ").map(|s| s.chars().collect()));
        let mut right_vec : Vec<HashSet<char>> = Vec::new();
        right_vec.extend(right.trim().split(" ").map(|s| s.chars().collect()));

        result.push((left_vec, right_vec));
    }

    result
}

fn generate_intersect_detection() -> DetectionRules {
    let mut detections : DetectionRules = Vec::new();
    let mut unknown_numbers : HashSet<usize> = HashSet::from([0, 2, 3, 5, 6, 9]);
    let mut known_numbers : HashSet<usize> = HashSet::from([1, 4, 7, 8]);
    let number_sets : Vec<HashSet<char>> = vec![
        HashSet::from(['a', 'b', 'c', 'e', 'f', 'g']),
        HashSet::from(['c', 'f']),
        HashSet::from(['a', 'c', 'd', 'e', 'g']),
        HashSet::from(['a', 'c', 'd', 'f', 'g']),
        HashSet::from(['b', 'c', 'd', 'f']),
        HashSet::from(['a', 'b', 'd', 'f', 'g']),
        HashSet::from(['a', 'b', 'd', 'e', 'f', 'g']),
        HashSet::from(['a', 'c', 'f']),
        HashSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g']),
        HashSet::from(['a', 'b', 'c', 'd', 'f', 'g']),
    ];

    loop {
        let mut new_detections : DetectionRules = Vec::new();
        for known_number in &known_numbers {
            let mut intersections : HashMap<usize, Vec<usize>> = HashMap::new();
            for unknown_number in &unknown_numbers {
                if known_number == unknown_number {
                    continue;
                }
                let intersection = number_sets[*unknown_number].intersection(&number_sets[*known_number]);
                let intersect_count = intersection.collect::<HashSet<&char>>().len();
                intersections.entry(intersect_count).or_insert(Vec::new()).push(*unknown_number);
            }
            for (count, numbers) in intersections.iter() {
                if numbers.len() == 1 {
                    new_detections.push((*known_number, *count, numbers[0]))
                }
            }

        }
        for (known_number, count, unknown_number) in &new_detections {
                if known_numbers.contains(unknown_number) {
                    continue;
                }
                known_numbers.insert(*unknown_number);
                unknown_numbers.remove(unknown_number);
                detections.push((*known_number, *count, *unknown_number));
        }
        if new_detections.len() == 0 {
            break;
        }
    }
    assert_eq!(known_numbers.len(), 10);

    detections
}

fn get_number_from_line(line: &InputLine, rules: &DetectionRules) -> usize {
    let mut unknown_digits = line.0.clone();
    let mut known_digits : Vec<Digit> = vec![Digit::new(); 10];

    // first fill the digits we can recognize by unique length
    let unique_lengths = [(2, 1), (4, 4), (3, 7), (7, 8)];
    for digit in & mut unknown_digits {
        for (len, value) in unique_lengths {
            if digit.len() == len {
                known_digits[value].extend(digit.drain());
                break;
            }
        }
    }
    for (len, value) in unique_lengths {
        assert_eq!(known_digits[value].len(), len);
    }

    // now use detection rules to recognize other digits by intersection lengths
    for (known, count, unknown) in rules {
        for digit in & mut unknown_digits {
            if known_digits[*known].intersection(&digit).collect::<HashSet<&char>>().len() == *count {
                known_digits[*unknown].extend(digit.drain());
                break;
            }
        }
        assert_ne!(known_digits[*unknown].len(), 0);
    }
    for digit in &known_digits {
        assert_ne!(digit.len(), 0);
    }

    // now we know all the digits, let's decode input number and start filling result
    let mut result: usize = 0;

    for digit in &line.1 {
        result *= 10;
        for i in 0..known_digits.len() {
            if *digit == known_digits[i] {
                result += i;
                break;
            }
        }
    }

    result
}

fn main() {
    let input_string = fs::read_to_string("input.txt").expect("failed to read input string");

    let detection_rules = generate_intersect_detection();
    for (known_number, count, unknown_number) in &detection_rules {
        println!("{} has {} intersections only with {}", known_number, count, unknown_number);
    }

    let input_data = parse_input(&input_string);
    let mut result : usize = 0;
    for input_line in input_data {
        result += get_number_from_line(&input_line, &detection_rules);
    }
    println!("{}", result);
}
