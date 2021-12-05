use std::fs;
use std::collections::HashMap;

fn parse_input_lines(lines: &Vec<&str>) -> Vec<Vec<u32>> {
    let mut result : Vec<Vec<u32>> = Vec::new();
    for line in lines {
        let mut line_result : Vec<u32> = Vec::new();
        for point in line.split(" -> ") {
            for coord in point.split(",") {
                line_result.push(coord.parse().unwrap());
            }
        }
        result.push(line_result);
    }
    result
}

fn gen_coords_between(a: u32, b: u32) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();

    let mut i : u32 = a;
    while i != b {
        result.push(i);
        if a > b {
            i -= 1;
        } else {
            i += 1;
        }
    }
    result.push(b);
    
    result
}

fn generate_line_points(lines: &Vec<Vec<u32>>) -> Vec<(u32, u32)> {
    let mut line_points: Vec<(u32, u32)> = Vec::new();

    for line in lines {
        if let [mut ax, mut ay, mut bx, mut by] = line[..] {
            let mut coords_x = gen_coords_between(ax, bx);
            let mut coords_y = gen_coords_between(ay, by);

            while coords_x.len() < coords_y.len() {
                coords_x.push(coords_x[0]);
            }
            while coords_y.len() < coords_x.len() {
                coords_y.push(coords_y[0]);
            }

            for (x,y) in coords_x.iter().zip(coords_y.iter()) {
                line_points.push((*x, *y));
            }
        }
    }

    line_points
}

fn count_line_points(line_points: &Vec<(u32, u32)>) -> HashMap<(u32, u32), u32> {
    let mut result: HashMap<(u32, u32), u32> = HashMap::new();

    for line_point in line_points {
        *result.entry(*line_point).or_insert(0) += 1;
    }

    result
}

fn count_overlaps(points: &HashMap<(u32, u32), u32>) -> usize {
    let mut result: usize = 0; 

    for (_, coverage) in points {
        if *coverage >= 2 {
            result += 1;
        }
    }

    result
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Input file read failed");
    let mut lines : Vec<&str> = Vec::new();
    lines.extend(contents.split("\n").filter(|l| ! l.is_empty()));

    let input_lines = parse_input_lines(&lines);
    let line_points = generate_line_points(&input_lines);
    let point_counts = count_line_points(&line_points);
    let overlaps = count_overlaps(&point_counts);

    println!("{}", overlaps);

}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn test_line_downwards() {

        let input_lines : Vec<Vec<u32>> = vec![
            vec![9, 4, 3, 4], 
        ];

        let line_points = generate_line_points(&input_lines);
        let line_points_expected : Vec<(u32, u32)> = vec![
            (3, 4), (4, 4), (5, 4), (6, 4), (7, 4), (8, 4), (9, 4)
        ];

        assert_eq!(line_points, line_points_expected);


    }

#[test]
    fn test_example() {

        let lines: Vec<&str> = vec![
            "0,9 -> 5,9",
            "8,0 -> 0,8",
            "9,4 -> 3,4",
            "2,2 -> 2,1",
            "7,0 -> 7,4",
            "6,4 -> 2,0",
            "0,9 -> 2,9",
            "3,4 -> 1,4",
            "0,0 -> 8,8",
            "5,5 -> 8,2",
        ];

        let input_lines = parse_input_lines(&lines);
        let input_lines_expected : Vec<Vec<u32>> = vec![
            vec![0, 9, 5, 9], 
            vec![8, 0, 0, 8], 
            vec![9, 4, 3, 4], 
            vec![2, 2, 2, 1], 
            vec![7, 0, 7, 4], 
            vec![6, 4, 2, 0], 
            vec![0, 9, 2, 9], 
            vec![3, 4, 1, 4], 
            vec![0, 0, 8, 8], 
            vec![5, 5, 8, 2], 
        ];

        assert_eq!(input_lines, input_lines_expected);

        let line_points = generate_line_points(&input_lines);
        let point_counts = count_line_points(&line_points);
        let overlaps = count_overlaps(&point_counts);

        assert_eq!(overlaps ,5);
    }
}
