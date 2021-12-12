use std::fs;

type Map = multimap::MultiMap<String, String>;
type Path = Vec<String>;
type PathList = Vec<Path>;

fn load_input_map(input: &str) -> Map {
    let mut result = Map::new();

    for line in input.split_whitespace() {
        if let [l, r] = &line.split("-").map(|s| s.to_string()).collect::<Vec<String>>()[..] {
            result.insert(l.clone(), r.clone());
            result.insert(r.clone(), l.clone());
        } else {
            panic!("wrong number of path elements in '{}'", line)
        }
    }

    result
}

fn new_path_list() -> PathList {
    vec![vec!["start".to_string()]]
}

fn generate_once(map: &Map, paths: &mut PathList) -> PathList {
    let mut finished_paths = PathList::new();
    let mut new_paths = PathList::new();
    for path in paths.drain(..) {
        for next in map.get_vec(&path[path.len()-1]).unwrap().iter() {
            if next.chars().next().unwrap().is_lowercase() && path.contains(next) {
                continue;
            }
            let mut new_path = path.clone();
            new_path.push(next.clone());
            if next == "end" {
                finished_paths.push(new_path);
            } else {
                new_paths.push(new_path);
            }
        }
    }
    paths.extend(new_paths.drain(..));

    finished_paths
}

fn generate_all(map: & Map) -> PathList {
    let mut current_paths = new_path_list();
    let mut finished_paths = PathList::new();

    while ! current_paths.is_empty() {
        finished_paths.extend(generate_once(&map, & mut current_paths).drain(..));
    }

    finished_paths
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("cannot read input file");
    let input_map = load_input_map(&input);
    println!("{}", generate_all(&input_map).len());

}

#[cfg(test)]
mod tests {
    use super::*;

    fn parsed_input_map() -> Map {
        let mut result = Map::new();
        result.insert("start".to_string(), "A".to_string());
        result.insert("A".to_string(), "start".to_string());
        result.insert("start".to_string(), "b".to_string());
        result.insert("b".to_string(), "start".to_string());
        result.insert("A".to_string(), "c".to_string());
        result.insert("c".to_string(), "A".to_string());
        result.insert("A".to_string(), "b".to_string());
        result.insert("b".to_string(), "A".to_string());
        result.insert("b".to_string(), "d".to_string());
        result.insert("d".to_string(), "b".to_string());
        result.insert("A".to_string(), "end".to_string());
        result.insert("end".to_string(), "A".to_string());
        result.insert("b".to_string(), "end".to_string());
        result.insert("end".to_string(), "b".to_string());
        result
    }

#[test]
    fn test_generate_once() {
        let map = parsed_input_map();
        let mut paths = new_path_list();
        
        let closed_paths = generate_once(&map, & mut paths);
        assert_eq!(closed_paths.len(), 0);
        assert_eq!(paths, vec![vec!["start", "A"], vec!["start", "b"]]);
    }

#[test]
    fn test_generate_all() {
        let map = parsed_input_map();
        assert_eq!(generate_all(&map).len(), 10);
    }
}
