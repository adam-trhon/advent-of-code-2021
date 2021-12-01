
use std::fs;

fn main()  {
    let contents = fs::read_to_string("input.txt").expect("Input file read failed");
    let mut lines = contents.split("\n");
    let mut increases = 0;


    const SLIDING_WINDOW_LENGTH: usize = 3;
    let mut window: [u32; SLIDING_WINDOW_LENGTH] = [0; SLIDING_WINDOW_LENGTH];
    let mut window_total_prev = 0;
    for i in 0..SLIDING_WINDOW_LENGTH {
        window[i] = lines.next().unwrap().parse().unwrap();
        window_total_prev += window[i];
    }

    let mut i = 0;
    for line in lines {
        if line.is_empty() {
            continue;
        }
        window[i] = line.parse().unwrap();
        let mut window_total = 0;
        for item in window {
            window_total += item;
        }

        if window_total > window_total_prev {
            increases += 1;
        }

        window_total_prev = window_total;
        i = (i + 1) % SLIDING_WINDOW_LENGTH;
    }
    println!("{}", increases);
}
