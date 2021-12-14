use std::fs;
use std::collections::HashMap;

type Polymer = Vec<String>;
type Rules = HashMap<String, [String; 2]>;

fn parse_template(text: &str) -> Polymer {
    let mut template_char = text.chars();

    let mut template = Polymer::new();
    let mut prev = template_char.next().unwrap();
    for p in template_char {
        template.push([prev, p].iter().collect::<String>());
        prev = p;
    }

    template
}

fn load_input(text: &str) -> (Polymer, Rules) {
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

        rules.insert(pattern, [value_l, value_r]);
    }


    (template, rules)
}

fn expand_polymer_by(polymer: Polymer, rules: &Rules) -> Polymer {
    let mut result = Polymer::new();

    for pair in polymer.iter() {
        result.extend(rules.get(pair).unwrap().iter().cloned());
    }

    result
}

fn count_atoms(polymer: &Polymer) -> HashMap<char, usize> {

    let mut result: HashMap<char, usize> = HashMap::new();

    result.insert(polymer[0].chars().next().unwrap(), 1);

    for atom_pair in polymer.iter() {
        let second_atom = atom_pair.chars().nth(1).unwrap();
        let current_value = *result.get(&second_atom).unwrap_or(&0);
        result.insert(second_atom, current_value + 1);
        //**result.get_mut(&second_atom).get_or_insert(&mut 0) += 1;
    }

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
    let (mut polymer, rules) = load_input(&input_text);
    for _ in 0..10 {
        polymer = expand_polymer_by(polymer, &rules);
    }
    let counts = count_atoms(&polymer);
    let (min, max) = get_count_min_max(&counts);
    println!("{}", max-min);
}
