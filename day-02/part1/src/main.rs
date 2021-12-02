use std::fs;

fn main()  {
        let contents = fs::read_to_string("input.txt").expect("Input file read failed");
        let lines = contents.split("\n");
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
            } else if instruction == "down" {
                depth += value;
            } else if instruction == "up" {
                depth -= value;
            } else {
                panic!("unknown instruction '{}'", instruction);
            }
        }
        println!("{}", dist);
        println!("{}", depth);
        println!("{}", dist*depth);
}
