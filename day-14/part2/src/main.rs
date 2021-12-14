use std::fs;
use std::collections::HashMap;

struct PolymerStats {
    atom_pairs: HashMap<String, usize>,
    first_pair: String,
    last_pair: String,
}

impl PolymerStats {
    pub fn new() -> Self {
        Self{ atom_pairs: HashMap::new(), first_pair: "".to_string(), last_pair: "".to_string()}
    }

    pub fn inc_middle_pair(&mut self, pair: String) {
        self.inc_middle_pair_by(pair, 1);
    }

    pub fn inc_middle_pair_by(&mut self, pair: String, count: usize) {
        let current_value = *self.atom_pairs.get(&pair).unwrap_or(&0);
        self.atom_pairs.insert(pair, current_value + count);
    }
}

type Rules = HashMap<String, (String, String)>;

fn parse_template(text: &str) -> PolymerStats {
    let template_chars = text.chars().collect::<Vec<char>>();

    let mut stats = PolymerStats::new();

    for i in 0..template_chars.len()-1 {
        let pair = [template_chars[i], template_chars[i+1]].iter().collect::<String>();
        if i == 0 {
            stats.first_pair = pair;
        } else if i == template_chars.len()-2 {
            stats.last_pair = pair;
        } else {
            stats.inc_middle_pair(pair);
        }
    }

    stats
}

fn load_input(text: &str) -> (PolymerStats, Rules) {
    let mut lines = text.split("\n");

    let template = parse_template(lines.next().unwrap());

    lines.next();

    let mut rules = Rules::new();
    for line in lines {
        if line.len() == 0 {
            continue;
        }
        let line_chars = line.chars().collect::<Vec<char>>();

        let pattern: String = [line_chars[0], line_chars[1]].iter().collect::<String>(); 
        let value_l: String = [line_chars[0], line_chars[6]].iter().collect::<String>();
        let value_r: String = [line_chars[6], line_chars[1]].iter().collect::<String>();

        rules.insert(pattern, (value_l, value_r));
    }


    (template, rules)
}

fn expand_polymer_by(polymer: PolymerStats, rules: &Rules) -> PolymerStats {
    let mut result = PolymerStats::new();

    for (pair, count) in polymer.atom_pairs.iter() {
        match rules.get(pair) {
            None => result.inc_middle_pair_by(pair.clone(), *count),
            Some((l, r)) => {
                result.inc_middle_pair_by(l.clone(), *count);
                result.inc_middle_pair_by(r.clone(), *count);
            }
        }
    }

    match rules.get(&polymer.first_pair) {
        None => result.first_pair = polymer.first_pair,
        Some((l, r)) => {
            result.inc_middle_pair(r.clone());
            result.first_pair = l.clone();
        }
    }

    match rules.get(&polymer.last_pair) {
        None => result.last_pair = polymer.last_pair,
        Some((l, r)) => {
            result.inc_middle_pair(l.clone());
            result.last_pair = r.clone();
        }
    }

    result
}

fn count_atoms(polymer: &PolymerStats) -> HashMap<char, usize> {

    let mut result: HashMap<char, usize> = HashMap::new();

    for (atom_pair, count) in polymer.atom_pairs.iter() {
        for atom in atom_pair.chars() {
            *result.entry(atom).or_insert(0) += count; 
        }
    }

    *result.entry(polymer.first_pair.chars().nth(1).unwrap()).or_insert(0) += 1;
    *result.entry(polymer.last_pair.chars().nth(0).unwrap()).or_insert(0) += 1;

    for (_, count) in result.iter_mut() {
        assert_eq!(*count % 2, 0);
        *count = *count / 2;
    }

    *result.entry(polymer.first_pair.chars().nth(0).unwrap()).or_insert(0) += 1;
    *result.entry(polymer.last_pair.chars().nth(1).unwrap()).or_insert(0) += 1;

    result
}

fn get_count_min_max(counts: &HashMap<char, usize>) -> (usize, usize) {
    let mut min : (&char, &usize) = counts.iter().next().unwrap();
    let mut max : (&char, &usize) = counts.iter().next().unwrap();

    for (atom, count) in counts.iter() {
        if count < min.1 {
            min = (atom, count);
        }
        if count > max.1 {
            max = (atom, count);
        }
    }

    (*min.1, *max.1)
}

fn main() {
    let input_text = fs::read_to_string("input.txt").expect("failed to read input file");
    let (mut polymer_stats, rules) = load_input(&input_text);
    for i in 0..40 {
        println!("iteration {}", i);
        polymer_stats = expand_polymer_by(polymer_stats, &rules);
    }
    let counts = count_atoms(&polymer_stats);
    let (min, max) = get_count_min_max(&counts);
    println!("{}", max-min);
}
