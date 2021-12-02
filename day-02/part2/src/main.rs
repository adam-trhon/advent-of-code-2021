use std::fs;

fn main()  {
        let contents = fs::read_to_string("input.txt").expect("Input file read failed");
        let lines = contents.split("\n");
        let mut aim : u32 = 0;
        let mut depth : u32 = 0;
        let mut dist : u32 = 0;
        for line in lines {
            if line.is_empty() {
                continue;
            }
            let mut parsed = line.split(" ");
            let instruction : &str = parsed.next().unwrap();
            let value : u32 = parsed.next().unwrap().parse().unwrap();

            if instruction == "forward" {
                dist += value;
                depth += aim*value;
            } else if instruction == "down" {
                aim += value;
            } else if instruction == "up" {
                aim -= value;
            } else {
                panic!("unknown instruction '{}'", instruction);
            }
        }
        println!("{}", dist);
        println!("{}", depth);
        println!("{}", dist*depth);
}
