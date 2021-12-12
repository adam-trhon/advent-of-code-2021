use std::fs;

type Map = multimap::MultiMap<String, String>;
#[derive(Clone, Debug)]
struct Path {
    points: Vec<String>,
    visited_twice: bool
}
impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.points == other.points && self.visited_twice == other.visited_twice
    }
}
impl Eq for Path { }
type PathList = Vec<Path>;

fn make_path(points: &[&str], visited_twice: bool) -> Path {
    Path{points: points.iter().map(|p| p.to_string()).collect::<Vec<String>>(), visited_twice: visited_twice}
}

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
    vec![Path{points: vec!["start".to_string()], visited_twice: false}]
}

fn add_point_to_path(path: &Path, point: &String) -> Option<Path> {
    let mut result : Option<Path> = None;

    let mut adding_possible = true;
    let mut visiting_twice = false;

    if point.chars().next().unwrap().is_lowercase() && path.points.contains(point) {
        if path.visited_twice || point == "start" {
            adding_possible = false;
        } else {
            visiting_twice = true;
        }
    }

    if adding_possible {
        let mut new_path = path.clone();
        new_path.points.push(point.clone());
        new_path.visited_twice = new_path.visited_twice || visiting_twice;
        result = Some(new_path);
    }

    result
}

fn generate_once(map: &Map, paths: &mut PathList) -> PathList {
    let mut finished_paths = PathList::new();
    let mut new_paths = PathList::new();
    for path in paths.drain(..) {
        for next in map.get_vec(&path.points[path.points.len()-1]).unwrap().iter() {
            match add_point_to_path(&path, &next) {
                Some(p) => {
                    if next == "end" {
                        finished_paths.push(p);
                    } else {
                        new_paths.push(p);
                    }
                }
                None => {
                }
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
        //assert_eq!(paths, vec![vec!["start", "A"], vec!["start", "b"]]);
    }

#[test]
    fn test_add_point_to_path_third_time() {
        let mut path = Path{points: vec![], visited_twice: false };
        match add_point_to_path(&path, &"foo".to_string()) {
            Some(p) => {
                assert_eq!(p.points.len(), 1);
                assert_eq!(p.visited_twice, false);
                path = p;
            }
            None => {
                panic!("path should not be none yet");
            }
        }

        match add_point_to_path(&path, &"foo".to_string()) {
            Some(p) => {
                assert_eq!(p.points.len(), 2);
                assert_eq!(p.visited_twice, true);
                path = p;
            }
            None => {
                panic!("path should not be none yet");
            }
        }

        match add_point_to_path(&path, &"foo".to_string()) {
            Some(_) => {
                panic!("path should be none now");
            }
            None => {
            }
        }


    }

#[test]
    fn test_generate_paths_one_by_one() {
        let map = parsed_input_map();
        let mut paths = new_path_list();

        generate_once(&map, &mut paths);
        let mut expected_paths = vec![
            make_path(&["start", "A"], false),
            make_path(&["start", "b"], false),
        ];
        assert_eq!(paths, expected_paths);

        generate_once(&map, &mut paths);
        expected_paths = vec![
            make_path(&["start", "A", "c"], false),
            make_path(&["start", "A", "b"], false),
            make_path(&["start", "b", "A"], false),
            make_path(&["start", "b", "d"], false),
        ];
        assert_eq!(paths, expected_paths);

        generate_once(&map, &mut paths);
        expected_paths = vec![
            make_path(&["start", "A", "c", "A"], false),
            make_path(&["start", "A", "b", "A"], false),
            make_path(&["start", "A", "b", "d"], false),
            make_path(&["start", "b", "A", "c"], false),
            make_path(&["start", "b", "A", "b"], true),
            make_path(&["start", "b", "d", "b"], true),
        ];
        assert_eq!(paths, expected_paths);

        generate_once(&map, &mut paths);
        expected_paths = vec![
            make_path(&["start", "A", "c", "A", "c"], true),
            make_path(&["start", "A", "c", "A", "b"], false),
            make_path(&["start", "A", "b", "A", "c"], false),
            make_path(&["start", "A", "b", "A", "b"], true),
            make_path(&["start", "A", "b", "d", "b"], true),
            make_path(&["start", "b", "A", "c", "A"], false),
            make_path(&["start", "b", "A", "b", "A"], true),
            make_path(&["start", "b", "A", "b", "d"], true),
            make_path(&["start", "b", "d", "b", "A"], true),
        ];
        assert_eq!(paths, expected_paths);
    }

#[test]
    fn test_generate_all() {
        let map = parsed_input_map();
        assert_eq!(generate_all(&map).len(), 36);
    }
}
