const MAX_DIGIT: usize = 13;

fn digit_params(digit: usize) -> (i32, i32, i32) {
    const PARAMS: [(i32, i32, i32); MAX_DIGIT+1]  = [
        ( 1, 12,  4), // most significant digit
        ( 1, 11, 10),
        ( 1, 14, 12),
        (26, -6, 14),
        ( 1, 15,  6),
        ( 1, 12, 16),
        (26, -9,  1),
        ( 1, 14,  7),
        ( 1, 14,  8),
        (26, -5, 11),
        (26, -9,  8),
        (26, -5,  3),
        (26, -2,  1),
        (26, -7,  8), // least significant digit
    ];

    PARAMS[MAX_DIGIT-digit]
}

#[allow(dead_code)]
fn proc_raw(w: i32, params: (i32, i32, i32), mut z: i32) ->  i32 {
    let a: i32 = params.0;
    let b: i32 = params.1;
    let c: i32 = params.2;

    let mut x: i32 = 0;
    x += z;
    x = x % 26;
    z = z/a;
    x += b;
    x = (x == w) as i32;
    x = (x == 0) as i32;
    let mut y: i32 = 0;
    y += 25;
    y *= x;
    y += 1;
    z *= y;
    y *= 0;
    y += w;
    y += c;
    y *= x;
    z += y;

    z
}

#[allow(dead_code)]
fn proc_short(w: i32, a:i32, b:i32, c:i32, z1: i32) ->  i32 {
    let x = z1%26 + b;

    let z2 = z1/a;

    let z3: i32;

    if x != w {
        z3 = z2*26 + w + c;
    } else {
        z3 = z2;
    }
    z3
}

fn proc_short_form(w: i32, params: (i32, i32, i32), z1: i32) ->  i32 {

    let a: i32 = params.0;
    let b: i32 = params.1;
    let c: i32 = params.2;

    let z3: i32;

    if z1%26 != w - b {
        let z21 = z1/a;
        z3 = z21*26 + w + c;
    } else {
        let z22 = z1/a;
        z3 = z22;
    }
    z3
}

fn proc_rev(w: i32, params: (i32, i32, i32), z3: i32) -> Vec<i32> {
    let a: i32 = params.0;
    let b: i32 = params.1;
    let c: i32 = params.2;

    let mut results: Vec<i32> = Vec::new();

    if (z3 - w - c) >= 0 && (z3 - w - c) %  26 == 0 {
        let z21 = (z3 - w - c)/26;
        for z1 in z21*a..(z21+1)*a {
            if z1%26 != w-b {
                results.push(z1);
            }
        }
    }

    let z22 = z3;
    for z1 in z22*a..(z22+1)*a {
        if z1%26 == w-b {
            results.push(z1);
        }
    }

    results
}

fn search(z: i32, digit: usize, curr_val: u64, mut best_val: u64) -> u64 {
    for w in 1..=9 {
        let next_curr_val: u64 = w as u64 * 10_u64.pow(digit as u32) + curr_val;
        let next_zs = proc_rev(w, digit_params(digit), z);

        if digit == MAX_DIGIT {
            if let Some(_) = next_zs.iter().find(|&x| *x == 0) {
                if next_curr_val > best_val {
                    best_val = next_curr_val;
                }
            }
        } else {
            for next_z in next_zs {
                let next_best = search(next_z, digit+1, next_curr_val, best_val);
                if next_best > best_val {
                    best_val = next_best;
                }
            }
        }
    }

    best_val
}



fn main() {
    println!("{}", search(0, 0, 0, 0));

}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn test_undivision() {
        // z21 == z1/a
        // z1 = z21*a
        let z21 = 10;
        let a = 5;
        for z1 in z21*a..(z21+1)*a {
            assert_eq!(z1/a, z21);
        }
    }

#[allow(dead_code)]
    fn test_proc_short_form_valid() {
        for w in 0..9 {
            for z in -100000..100000 {
                for i in 0..MAX_DIGIT {
                    assert_eq!(proc_raw(w, digit_params(i), z), proc_short_form(w, digit_params(i), z));
                }
            }
        }
    }

#[test]
//#[allow(dead_code)]
    fn test_proc_rev_valid() {
        for w in 1..9 {
            for z in -10000..10000 {
                for i in 0..MAX_DIGIT {
                    let rev_zs = proc_rev(w, digit_params(i), z);
                    for rev_z in rev_zs {
                        let orig_z = proc_short_form(w, digit_params(i), rev_z);
                        if z != orig_z {
                            panic!("fail w {} i {} z {} rev_z {} orig_z {}",
                                w, i, z, rev_z, orig_z);
                        }
                    }
                }
            }
        }
    }

}


