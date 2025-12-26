use regex::Regex;
use std::io::{self, BufRead};

fn main() {
    let mut input_iterator = io::stdin().lock().lines();

    let mut pointer = 50;
    let mut part_1_total_zeros = 0;
    let mut part_2_crossing_zeros = 0;

    let re = Regex::new(r"([L|R])([0-9]+)").unwrap();
    while let Some(line) = input_iterator.next() {
        for (_, [direction, distance_str]) in re.captures_iter(&line.unwrap()).map(|c| c.extract())
        {
            let distance = distance_str.parse::<i32>().unwrap();
            println!("{} {}", direction, distance);
            for _ in 0..distance {
                pointer = if direction == "R" {
                    (pointer + 1) % 100
                } else {
                    (pointer - 1 + 100) % 100
                };

                if pointer == 0 {
                    part_2_crossing_zeros += 1;
                }
            }

            if pointer == 0 {
                part_1_total_zeros += 1;
            }
        }
    }

    println!("Part 1: total zeros = {}", part_1_total_zeros);
    println!("Part 2: total zero crossings = {}", part_2_crossing_zeros);
}
