
use std::fs;

fn main()  {
    let contents = fs::read_to_string("input.txt").expect("Input file read failed");
    let mut lines = contents.split("\n");
    let mut increases = 0;
    let mut previous_depth: u32 = lines.next().unwrap().parse().unwrap();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let current_depth: u32 = line.parse().unwrap();
        if current_depth > previous_depth {
            increases = increases + 1;
        }
        previous_depth = current_depth;
    }
    println!("{}", increases);
}
