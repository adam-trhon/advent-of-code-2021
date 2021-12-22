use std::fs;
use std::collections::HashSet;

type Algorithm = Vec<bool>;

struct Image {
    data: HashSet<(i32, i32)>,
    inverted: bool,
}

impl Image {
    fn new() -> Image {
        Image {data: HashSet::<(i32, i32)>::new(), inverted: false}
    }
}

fn load_input(text: &str) -> (Algorithm, Image) {
    let mut line_iter = text.split("\n");
    let algorithm = line_iter.next().unwrap().chars().map(|c| c == '#').collect::<Algorithm>();

    line_iter.next();

    let mut image: Image = Image::new();
    for (i, line) in line_iter.enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                image.data.insert((i as i32,j as i32));
            }
        }
    }

    (algorithm, image)
} 

fn get_bound(image: &Image) -> (std::ops::Range<i32>, std::ops::Range<i32>) {
    let (mut min_row, mut min_col) = image.data.iter().next().unwrap();
    let (mut max_row, mut max_col) = image.data.iter().next().unwrap();

    for (i, j) in image.data.iter() {
        if i < &min_row {
            min_row = *i;
        }
        if i > &max_row {
            max_row = *i;
        }
        if j < &min_col {
            min_col = *j;
        }
        if j > &max_col {
            max_col = *j;
        }
    }

    ((min_row-2..max_row+2), (min_col-2..max_col+2))
}

fn process_pixel(image: &Image, pos: &(i32, i32), algo: &Algorithm, invert: bool) -> bool {
    let mut result: usize = 0;

    for i in -1i32..=1 {
        for j in -1i32..=1 {
            result *= 2;
            if image.data.contains(&(pos.0+i, pos.1+j)) ^ image.inverted {
                result += 1;
            }
        }
    }
    algo[result] ^ invert
}

fn apply_algorithm(a: &Algorithm, img: &Image) -> Image {
    let (row_range, col_range) = get_bound(&img);

    let mut result = Image::new();
    result.inverted = a[0] ^ img.inverted;

    for i  in row_range {
        for j in col_range.clone() {
            if process_pixel(img, &(i, j), a, result.inverted) {
                result.data.insert((i, j));
            }
        }
    }
    result
}

fn main() {
    let text = fs::read_to_string("input.txt").expect("cannot read input.txt");
    let (algorithm, image) = load_input(&text);
    let output_image = apply_algorithm(&algorithm, &image);
    let output_image_2 = apply_algorithm(&algorithm, &output_image);
    println!("{}", output_image_2.data.len());
}
