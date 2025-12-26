use regex::Regex;
use std::io::{self, BufRead};

struct Shape {
    pub form: [[bool; 3]; 3],
}

struct Region {
    pub width: usize,
    pub length: usize,
    pub quantities: Vec<usize>,
}

fn main() {
    let (shapes, regions) = parse_input();

    let mut fit_count = 0;
    for region in regions {
        println!("{:?}", region.quantities);
        let total_shape_area = region
            .quantities
            .iter()
            .enumerate()
            .map(|(shape_index, count)| count * shape_area(&shapes[shape_index]))
            .sum();
        if region_area(&region) >= total_shape_area {
            fit_count += 1;
        }
    }

    println!("Part 1: Number of regions that fit presents = {fit_count}");
}

fn shape_area(shape: &Shape) -> usize {
    shape.form.iter().flatten().filter(|&&elem| elem).count()
}

fn region_area(region: &Region) -> usize {
    region.width * region.length
}

fn parse_input() -> (Vec<Shape>, Vec<Region>) {
    let mut shapes = Vec::new();
    let mut regions = Vec::new();

    let mut input_iterator = io::stdin().lock().lines();

    while let Some(input_line) = input_iterator.next() {
        let line = input_line.unwrap();
        if !line.is_empty() {
            if line[line.len() - 1..line.len()] == *":" {
                shapes.push(parse_shape([
                    &input_iterator.next().unwrap().unwrap(),
                    &input_iterator.next().unwrap().unwrap(),
                    &input_iterator.next().unwrap().unwrap(),
                ]));
            } else {
                regions.push(parse_region(&line));
            }
        }
    }

    (shapes, regions)
}

fn parse_shape(lines: [&str; 3]) -> Shape {
    Shape {
        form: [
            parse_shape_line(lines[0]),
            parse_shape_line(lines[1]),
            parse_shape_line(lines[2]),
        ],
    }
}

fn parse_shape_line(line: &str) -> [bool; 3] {
    let chars = line[0..3].as_bytes();
    [
        is_part_of_shape(chars[0]),
        is_part_of_shape(chars[1]),
        is_part_of_shape(chars[2]),
    ]
}

fn is_part_of_shape(ch: u8) -> bool {
    ch == b'#'
}

fn parse_region(line: &str) -> Region {
    let fields: Vec<&str> = line.split_whitespace().collect();

    let (width, length) = parse_width_length(fields[0]);

    Region {
        width,
        length,
        quantities: fields
            .iter()
            .skip(1)
            .map(|field| field.parse::<usize>().unwrap())
            .collect(),
    }
}

fn parse_width_length(field: &str) -> (usize, usize) {
    let re = Regex::new(r"([0-9]+)x([0-9]+):").unwrap();

    for (_, [width, length]) in re.captures_iter(field).map(|c| c.extract()) {
        return (
            width.parse::<usize>().unwrap(),
            length.parse::<usize>().unwrap(),
        );
    }

    panic!("Couldn't parse width and length");
}
