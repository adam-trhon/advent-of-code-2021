use std::fs;

fn parse_input(text: &str) -> Vec<char> {
    let mut result: Vec<char> = Vec::new();

    for c in text.trim().chars() {
        result.extend(format!("{:04b}", c.to_digit(16).unwrap()).chars());
    }

    result
}

fn bits_to_u128(bits: &[char]) -> u128 {
    let mut result : u128 = 0;

    for b in bits.iter() {
        result *= 2;
        result += b.to_digit(2).unwrap() as u128;
    }

    result
}

fn get_packet_version(packet: &[char]) -> (u128, usize) {
    (bits_to_u128(&packet[0..3]), 3)
}

fn get_packet_id(packet: &[char]) -> (u128, usize) {
    (bits_to_u128(&packet[0..3]), 3)
}

fn parse_literal(packet: &[char]) -> (u128, usize) {
    let mut pos: usize = 0;
    let mut result_bits: Vec<char> = Vec::new();

    loop {
        result_bits.extend(packet[pos+1..pos+5].iter());
        if packet[pos] == '0' {
            break;
        }
        pos += 5;
    }
    pos += 5;

    (bits_to_u128(&result_bits), pos)
}

fn parse_operator(op: u128, packet: &[char]) -> (u128, usize) {
    let mut pos: usize = 1;
    let mut sub_results: Vec<u128> = Vec::new();
    let result: u128;

    if packet[0] == '0' {
        pos += 15;
        let subpacket_total_length = bits_to_u128(&packet[1..pos]) as usize;
        let mut subpacket_pos: usize = 0;
        while subpacket_pos < subpacket_total_length {
            let (subpacket_val, subpacket_length) = process_packet(&packet[pos..]);
            subpacket_pos += subpacket_length;
            pos += subpacket_length;
            sub_results.push(subpacket_val);
        }
    } else {
        pos += 11;
        let subpacket_count = bits_to_u128(&packet[1..pos]) as usize;
        for _ in 0..subpacket_count {
            let (subpacket_val, subpacket_length) = process_packet(&packet[pos..]);
            pos += subpacket_length;
            sub_results.push(subpacket_val);
        }
    }

    match op {
        0 => result = sub_results.iter().sum(),
        1 => result = sub_results.iter().product(),
        2 => result = *sub_results.iter().min().unwrap(),
        3 => result = *sub_results.iter().max().unwrap(),
        5 => result = (sub_results[0] > sub_results[1]) as u128,
        6 => result = (sub_results[0] < sub_results[1]) as u128,
        7 => result = (sub_results[0] == sub_results[1]) as u128,
        _ => panic!("unsupported operation"),
    }

    (result, pos)
}

fn process_packet(packet: &[char]) -> (u128, usize) {
    let mut pos: usize = 0;
    let mut result: u128 = 0;

    let (_, step) = get_packet_version(&packet[pos..]);
    pos += step;
    let (packet_id, step) = get_packet_id(&packet[pos..]);
    pos += step;

    match packet_id {
        4 => {
            let (literal_value,step) = parse_literal(&packet[pos..]);
            result = literal_value;
            pos += step;
        }
        _ => {
            let (operator_result, step) = parse_operator(packet_id, &packet[pos..]);
            pos += step;
            result += operator_result;
        }
    }

    (result,  pos)
}

fn main() {
    let input_text = fs::read_to_string("input.txt").expect("cannot read input.txt");
    let input = parse_input(&input_text);
    println!("{}", process_packet(&input).0);
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn test_parse_input() {
        let expected = vec!['1','1','0','1','0','0','1','0','1','1','1','1','1',
            '1','1','0','0','0','1','0','1','0','0','0',];
        assert_eq!(parse_input("D2FE28"), expected);
    }

#[test]
    fn test_get_packet_version() {
        assert_eq!(get_packet_version(&parse_input("D2FE28")), (6, 3));
    }

#[test]
    fn test_get_packet_id() {
        assert_eq!(get_packet_id(&parse_input("D2FE28")[3..]), (4, 3));
    }

#[test]
    fn test_parse_literal() {
        assert_eq!(parse_literal(&parse_input("D2FE28")[6..]), (2021, 15));
    }

#[test]
    fn test_process_packet_literal() {
        assert_eq!(process_packet(&parse_input("D2FE28")), (2021, 21));
    }

#[test]
    fn test_parse_operator_1() {
        assert_eq!(process_packet(&parse_input("C200B40A82")).0, 3);
    }

#[test]
    fn test_parse_operator_2() {
        assert_eq!(process_packet(&parse_input("04005AC33890")).0, 54);
    }

#[test]
    fn test_parse_operator_3() {
        assert_eq!(process_packet(&parse_input("880086C3E88112")).0, 7);
    }

#[test]
    fn test_parse_operator_4() {
        assert_eq!(process_packet(&parse_input("CE00C43D881120")).0, 9);
    }
}
