use std::fs;

fn main()  {
        let contents = fs::read_to_string("input.txt").expect("Input file read failed");
        let lines = contents.split("\n");
        let mut ones_count : [u32; 12] = [0; 12]; 
        let mut zeros_count : [u32; 12] = [0; 12];

        for line in lines {
            if line.is_empty() {
                continue;
            }
            for (i, c) in line.chars().enumerate() {
                if c == '1' {
                    ones_count[i] += 1;
                } else {
                    zeros_count[i] += 1;
                }
            }
        }

        let mut gamma_rate : u32 = 0;
        let mut epsilon_rate : u32 = 0;

        for (&c0, c1) in zeros_count.iter().zip(ones_count) {
            gamma_rate *= 2;
            epsilon_rate *= 2;

            if c0 > c1 {
                gamma_rate += 1;
            } else {
                epsilon_rate += 1;
            }
        }

        println!("{}", gamma_rate*epsilon_rate);
}
