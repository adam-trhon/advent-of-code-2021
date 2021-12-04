use std::fs;

fn get_nth(string: &str, pos: usize) -> char {
    string.chars().nth(pos).unwrap()
}

fn count_zeroes_at(lines: &[&str], position: usize) -> usize {
    let mut count : usize = 0;
    for line in lines {
        if get_nth(line, position) == '0' {
            count += 1;
        } else {
            break;
        }
    }
    count
}

fn get_o2_generator_rating(lines: &[&str]) -> u32 {
        let mut o2_generator_rating : u32 = 0;

        let mut slice_size = lines.len();
        let mut slice_pos : usize = 0;
        let mut zero_count : usize = 0;

        let width = lines[0].len();

        for i in 0 .. width {
            o2_generator_rating *= 2;
            zero_count = count_zeroes_at(&lines[slice_pos .. slice_pos + slice_size], i);
            let one_count = slice_size - zero_count;

            if one_count >= zero_count {
                o2_generator_rating += 1;
                slice_pos += zero_count;
                slice_size = one_count;
            } else {
                slice_size = zero_count;
            }
        }

        o2_generator_rating
}

fn get_co2_scrubber_rating(lines: &[&str]) -> u32 {
        let mut co2_scrubber_rating : u32 = 0;

        let mut slice_size = lines.len();
        let mut slice_pos : usize = 0;
        let mut zero_count : usize = 0;

        let width = lines[0].len();

        for i in 0 .. width {
            co2_scrubber_rating *= 2;
            zero_count = count_zeroes_at(&lines[slice_pos .. slice_pos + slice_size], i);
            let one_count = slice_size - zero_count;

            if (one_count > 0 && one_count < zero_count) || (zero_count == 0) {
                co2_scrubber_rating += 1;
                slice_pos += zero_count;
                slice_size = one_count;
            } else {
                slice_size = zero_count;
            }
        }

        co2_scrubber_rating
}

fn main()  {
        let contents = fs::read_to_string("input.txt").expect("Input file read failed");
        let mut lines : Vec<&str> = Vec::new();
        lines.extend(contents.split("\n").filter(|l| ! l.is_empty()));
        lines.sort();

        println!("{}", get_o2_generator_rating(&lines[..])*get_co2_scrubber_rating(&lines[..]));
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn test_count_zeroes_at() {
        let mut data : Vec<&str> = Vec::new();
        data.push("00001");
        data.push("00011");
        data.push("00101");
        data.push("00111");
        data.push("01001");
        data.push("01011");
        data.push("01101");
        data.push("01111");

        assert_eq!(count_zeroes_at(&data[0 .. 7], 0), 7);
        assert_eq!(count_zeroes_at(&data[0 .. 7], 1), 4);
        assert_eq!(count_zeroes_at(&data[0 .. 7], 4), 0);
    }

#[test]
    fn test_o2_generator_rating() {
        let mut data : Vec<&str> = Vec::new();
        data.push("00100");
        data.push("11110");
        data.push("10110");
        data.push("10111");
        data.push("10101");
        data.push("01111");
        data.push("00111");
        data.push("11100");
        data.push("10000");
        data.push("11001");
        data.push("00010");
        data.push("01010");

        data.sort();

        assert_eq!(get_o2_generator_rating(&data[..]), 23)
    }

#[test]
    fn test_co2_scrubber_rating_0_0() {
        let mut data : Vec<&str> = Vec::new();
        data.push("0");
        data.push("0");
        data.push("0");
        data.push("0");

        assert_eq!(get_co2_scrubber_rating(&data[..]), 0)
    }

#[test]
    fn test_co2_scrubber_rating_0_1() {
        let mut data : Vec<&str> = Vec::new();
        data.push("1");
        data.push("1");
        data.push("1");
        data.push("1");

        assert_eq!(get_co2_scrubber_rating(&data[..]), 1)
    }

#[test]
    fn test_co2_scrubber_rating_1() {
        let mut data : Vec<&str> = Vec::new();
        data.push("0");
        data.push("1");
        data.push("1");

        assert_eq!(get_co2_scrubber_rating(&data[..]), 0)
    }

#[test]
    fn test_co2_scrubber_rating_2() {
        let mut data : Vec<&str> = Vec::new();
        data.push("0");
        data.push("0");
        data.push("1");

        assert_eq!(get_co2_scrubber_rating(&data[..]), 1)
    }

#[test]
    fn test_co2_scrubber_rating_3() {
        let mut data : Vec<&str> = Vec::new();
        data.push("0");
        data.push("0");
        data.push("1");
        data.push("1");

        assert_eq!(get_co2_scrubber_rating(&data[..]), 0)
    }

#[test]
    fn test_co2_scrubber_rating_upstream_example() {
        let mut data : Vec<&str> = Vec::new();
        data.push("00100");
        data.push("11110");
        data.push("10110");
        data.push("10111");
        data.push("10101");
        data.push("01111");
        data.push("00111");
        data.push("11100");
        data.push("10000");
        data.push("11001");
        data.push("00010");
        data.push("01010");

        data.sort();

        assert_eq!(get_co2_scrubber_rating(&data[..]), 10)
    }

}
