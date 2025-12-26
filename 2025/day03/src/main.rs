use std::env;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Must specify the number of batteries to use in each bank");
        std::process::exit(1);
    }

    let num_batteries = args[1]
        .parse::<usize>()
        .expect("The argument must be integral");

    let mut input_iterator = io::stdin().lock().lines();

    let mut total_output_joltage = 0;

    while let Some(input_line) = input_iterator.next() {
        let line = input_line.unwrap();
        let mut bank_joltage = 0;
        let mut current_search_index = 0;

        for index in 0..num_batteries {
            let index_of_max = current_search_index
                + index_of_highest(
                    &line[current_search_index..(line.len() - num_batteries + index + 1)],
                );

            current_search_index = index_of_max + 1;

            let battery_joltage = line
                .chars()
                .nth(index_of_max)
                .unwrap()
                .to_digit(10)
                .unwrap() as u64;

            bank_joltage = bank_joltage * 10 + battery_joltage;
        }

        total_output_joltage += bank_joltage;
    }

    println!("Total output joltage is {}", total_output_joltage);
}

fn index_of_highest(s: &str) -> usize {
    s.chars()
        .rev()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(index, _)| s.len() - 1 - index)
        .unwrap()
}
