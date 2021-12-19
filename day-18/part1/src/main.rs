use std::fmt;
use std::fs;

#[derive(Clone)]
enum Element {
    Pair(Box<Element>, Box<Element>),
    Number(u32),
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pair(s_left, s_right) => write!(f, "[{},{}]", s_left, s_right),
            Self::Number(n) => write!(f, "{}", n),
        }
    }
}

impl fmt::Debug for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pair(s_left, s_right) => write!(f, "[{},{}]", s_left, s_right),
            Self::Number(n) => write!(f, "{}", n),
        }
    }
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Pair(s_left, s_right) => match other {
                Self::Pair(o_left, o_right) => {
                    return *s_left == *o_left && *s_right == *o_right;
                }
                _ => {
                    return false;
                }
            }
            Self::Number(s_num) => match other {
                Self::Number(o_num) => {
                    return s_num == o_num;
                }
                _ => {
                    return false;
                }
            }
        }
    }
}

fn parse_number(text: &[char]) -> (Box<Element>, usize) {
    let mut num_str: String = String::new();

    for c in text.iter() {
        if c.is_digit(10) {
            num_str.push(*c);
        } else {
            break;
        }
    }

    (Box::new(Element::Number(num_str.parse::<u32>().unwrap())), num_str.len())
}

fn parse_pair(text: &[char]) -> (Box<Element>, usize) {
    let mut pos: usize = 0;

    assert_eq!(text[pos], '[');
    pos += 1;

    let (left, left_size) = parse_element(&text[pos..]);
    pos += left_size;

    assert_eq!(text[pos], ',');
    pos += 1;

    let (right, right_size) = parse_element(&text[pos..]);
    pos += right_size;

    assert_eq!(text[pos], ']');
    pos += 1;

    (Box::new(Element::Pair(left, right)), pos)
}

fn parse_element(text: &[char]) -> (Box<Element>, usize) {
    if text[0].is_digit(10) {
        return parse_number(text);
    } else {
        assert_eq!(text[0], '[');
        return parse_pair(text);
    }
}

fn max_nesting(n: &Element) -> usize {
    match n {
        Element::Number(_) => 0,
        Element::Pair(l, r) => 1 + [max_nesting(&**l), max_nesting(&**r)].iter().max().unwrap()
    }
}

fn parse(text: &str) -> Element {
    *parse_element(&text.chars().collect::<Vec<char>>()).0
}

fn insert_fragment_left(e: &mut Element, fragment: u32) {
    match e {
        Element::Number(n) => {
            *e = Element::Number(*n + fragment);
        }
        Element::Pair(_, r) => {
            // direction left means we are inserting to the right element
            insert_fragment_left(&mut *r, fragment);
        }
    }
}

fn insert_fragment_right(e: &mut Element, fragment: u32) {
    match e {
        Element::Number(n) => {
            *e = Element::Number(*n + fragment);
        }
        Element::Pair(l, _) => {
            // direction right means we are inserting to the left element
            insert_fragment_right(&mut *l, fragment);
        }
    }
}

fn explode(e: &mut Element, height: usize) -> (bool, Option<u32>, Option<u32>) {
    match e {
        Element::Number(_) => {
            return (false, None, None);
        }
        Element::Pair(l, r) => {
            if height > 4 {
                match(&**l, &**r) {
                    (Element::Number(lv), Element::Number(rv)) => {
                        return (true, Some(*lv), Some(*rv));
                    }
                    (_, _) => {
                        panic!("Height > 4, but not two numbers");
                    }
                }
            } else {
                match explode(&mut *l, height+1) {
                    (true, None, None) => {
                        return (true, None, None);
                    }
                    (false, None, None) => {
                        // nothing happened, we are going to try to explode r
                    }
                    (true, Some(el), Some(er)) => {
                        *l = Box::new(Element::Number(0));
                        insert_fragment_right(r, er);
                        return (true, Some(el), None);
                    }
                    (true, Some(el), None) => {
                        return (true, Some(el), None);
                    }
                    (true, None, Some(er)) => {
                        insert_fragment_right(r, er);
                        return (true, None, None);
                    }
                    (false, Some(_), _)|(false, _, Some(_)) => {
                        panic!("Got fragment from nonexistent explosion at left");
                    }
                }
                match explode(&mut *r, height+1) {
                    (exploded, None, None) => {
                        // nothing happened, nothing exploded
                        return (exploded, None, None);
                    }
                    (true, Some(el), Some(er)) => {
                        *r = Box::new(Element::Number(0));
                        insert_fragment_left(l, el);
                        return (true, None, Some(er));
                    }
                    (true, Some(el), None) => {
                        insert_fragment_left(l, el);
                        return (true, None, None);
                    }
                    (true, None, Some(er)) => {
                        return (true, None, Some(er));
                    }
                    (false, Some(_), _)|(false, _, Some(_)) => {
                        panic!("Got fragment from nonexistent explosion at right");
                    }
                }
            }
        }
    }
}

fn split(e: &mut Element) -> bool {
    match e {
        Element::Number(n) => {
            if *n > 9 {
                let left: u32 = *n/2;
                let right: u32 = *n - left;
                *e = Element::Pair(Box::new(Element::Number(left)), Box::new(Element::Number(right)));
                return true;
            } else {
                return false;
            }
        }
        Element::Pair(l, r) => {
            return split(&mut *l) || split(&mut *r);
        }
    }
}

fn reduce(e: &mut Element) {
    loop {
        while explode(e, 1).0 {
        }
        if !split(e) {
            break;
        }
    }
}

fn add(l: &mut Element, r: Element) {
    *l = Element::Pair(Box::new(l.clone()), Box::new(r.clone()));
    reduce(l);
}

fn sum(elems: &[Element]) -> Element {
    let mut iter = elems.iter();
    let mut result: Element = iter.next().unwrap().clone();
    for e in iter {
        add(&mut result, e.clone());
    }
    result
}

fn magnitude(elem: &Element) -> u32 {
    match elem {
        Element::Number(n) => {
            return *n;
        }
        Element::Pair(l, r) => {
            return magnitude(&*l)*3 + magnitude(&*r)*2;
        }
    }
}

fn load_input(text: &str) -> Vec<Element> {
    let mut result: Vec<Element> = Vec::new();

    for line in text.trim().split_whitespace() {
        result.push(parse(line));
    }

    result
}

fn main() {
    let text = fs::read_to_string("input.txt").expect("cannot read input");
    let input = load_input(&text);
    let s = sum(&input);
    println!("{}", magnitude(&s));
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn test_parse_simple_pair() {
        let expected = Element::Pair(Box::new(Element::Number(1)), Box::new(Element::Number(2)));
        assert_eq!(parse("[1,2]"), expected);
    }

#[test]
    fn test_parse_pair_of_pairs() {
        let expected = Element::Pair(
            Box::new(Element::Pair(Box::new(Element::Number(1)), Box::new(Element::Number(2)))),
            Box::new(Element::Pair(Box::new(Element::Number(3)), Box::new(Element::Number(4)))),
            );
        assert_eq!(parse("[[1,2],[3,4]]"), expected);
    }

#[test]
    fn test_load_input() {
        let expected = vec![
            Element::Pair(Box::new(Element::Number(1)), Box::new(Element::Number(2))),
            Element::Pair(Box::new(Element::Number(3)), Box::new(Element::Number(4))),
            ];
        assert_eq!(load_input("[1,2]\n[3,4]\n"), expected);

    }

#[test]
    fn test_measure_nesting_simple_pair() {
        assert_eq!(max_nesting(&parse("[1,2]")), 1);
    }

#[test]
    fn test_measure_nesting_pair_of_pairs() {
        assert_eq!(max_nesting(&parse("[[1,2],[3,4]]")), 2);
    }

#[test]
    fn test_insert_fragment_left() {
        let mut input = parse("[2,[3,3]]");
        let expected = parse("[2,[3,4]]");
        insert_fragment_left(&mut input, 1);
        assert_eq!(input, expected);
    }

#[test]
    fn test_explode_simple() {
        let mut input = parse("[[[[[1,2],3],4],5],6]");
        let expected = parse("[[[[0,5],4],5],6]");
        assert_eq!(explode(&mut input, 1), (true, Some(1), None));
        assert_eq!(input, expected);
    }

#[test]
    fn test_explode_examples() {
        for (i, e) in [
            ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
            ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
            ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
            ("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"),
            ("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[7,0]]]]"),
            ("[[[[[7,7],[7,0]],[[7,7],[7,6]]],8],[0,2]]", "[[[[0,[14,0]],[[7,7],[7,6]]],8],[0,2]]"),
            ("[[[[0,[14,0]],[[7,7],[7,6]]],8],[0,2]]", "[[[[14,0],[[7,7],[7,6]]],8],[0,2]]"),
        ] {
            let mut input = parse(i);
            let expected = parse(e);
            assert_eq!(explode(&mut input, 1).0, true);
            assert_eq!(input, expected);
        }
    }

#[test]
    fn test_split_examples() {
        for (i, e) in [
            ("[11,3]", "[[5,6],3]"),
        ] {
            let mut input = parse(i);
            let expected = parse(e);
            assert_eq!(split(&mut input), true);
            assert_eq!(input, expected);
        }
    }

#[test]
    fn test_reduce() {
        for (i, e) in [
            ("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]", "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"),
            ("[[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]",
                "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"),

        ] {
            let mut input = parse(i);
            let expected = parse(e);
            reduce(&mut input);
            assert_eq!(input, expected);
        }
    }

#[test]
    fn test_add() {
        let mut l = parse("[[7,7],6]");
        let r = parse("[[[9,0],4],3]");
        let expected = parse("[[[7,7],6],[[[9,0],4],3]]");
        add(&mut l, r);
        assert_eq!(l, expected);
    }

#[test]
    fn test_add_2() {
        let mut l = parse("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]");
        let r = parse("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]");
        let expected = parse("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]");
        add(&mut l, r);
        assert_eq!(l, expected);
    }

#[test]
    fn test_magnitude() {
        for (i, m) in [
            ("[[1,2],[[3,4],5]]", 143),
            ("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384),
            ("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445),
            ("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791),
            ("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137),
            ("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]", 3488),
        ] {
            let input = parse(i);
            assert_eq!(magnitude(&input), m);
        }
    }

#[test]
    fn test_bigger_example_1() {
        let input = vec![
            parse("[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]"),
            parse("[[[5,[2,8]],4],[5,[[9,9],0]]]"),
            parse("[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]"),
            parse("[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]"),
            parse("[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]"),
            parse("[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]"),
            parse("[[[[5,4],[7,7]],8],[[8,3],8]]"),
            parse("[[9,3],[[9,9],[6,[4,9]]]]"),
            parse("[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]"),
            parse("[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"),
        ];
        let s = sum(&input);
        assert_eq!(s, parse("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]"));
        assert_eq!(magnitude(&s), 4140);
    }

#[test]
    fn test_bigger_example_2() {
        let input = vec![
            parse("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]"),
            parse("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]"),
            parse("[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]"),
            parse("[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]"),
            parse("[7,[5,[[3,8],[1,4]]]]"),
            parse("[[2,[2,2]],[8,[8,1]]]"),
            parse("[2,9]"),
            parse("[1,[[[9,3],9],[[9,0],[0,7]]]]"),
            parse("[[[5,[7,4]],7],1]"),
            parse("[[[[4,2],2],6],[8,7]]"),
        ];
        let s = sum(&input);
        assert_eq!(s, parse("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"));
    }

#[test]
    fn test_sum_1() {
        let input = vec![
            parse("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]"),
            parse("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]"),
        ];
        let s = sum(&input);
        assert_eq!(s, parse("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"));
    }

#[test]
    fn test_reduce_1() {
        for (i, e) in [
            ("[[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]",
            "[[[[4,0],[5,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]"),
            ("[[[[4,0],[5,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]",
            "[[[[4,0],[5,4]],[[0,[7,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]"),
            ("[[[[4,0],[5,4]],[[0,[7,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]",
            "[[[[4,0],[5,4]],[[7,0],[15,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]"),
            ("[[[[4,0],[5,4]],[[7,0],[15,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]",
            "[[[[4,0],[5,4]],[[7,0],[15,5]]],[10,[[0,[11,3]],[[6,3],[8,8]]]]]"),
            ("[[[[4,0],[5,4]],[[7,0],[15,5]]],[10,[[0,[11,3]],[[6,3],[8,8]]]]]",
            "[[[[4,0],[5,4]],[[7,0],[15,5]]],[10,[[11,0],[[9,3],[8,8]]]]]"),
        ] {
            let mut input = parse(i);
            assert_eq!(explode(&mut input, 1).0, true);
            assert_eq!(input, parse(e));
        }
    }

}
