use std::cmp;
use std::io::{self, BufRead};

fn main() {
    let mut ranges = Vec::new();
    let mut fresh_ingredient_count = 0;

    let mut input_iterator = io::stdin().lock().lines();
    let mut processing_ranges = true;
    while let Some(input_line) = input_iterator.next() {
        let line = input_line.unwrap();

        if line.len() == 0 {
            processing_ranges = false;
        } else if processing_ranges {
            let parts: Vec<&str> = line.split('-').collect();
            ranges.push((
                parts[0].parse::<u64>().unwrap(),
                parts[1].parse::<u64>().unwrap(),
            ));
        } else {
            let ingredient = line.parse::<u64>().unwrap();
            if is_fresh(&ranges, ingredient) {
                fresh_ingredient_count += 1;
            }
        }
    }

    println!(
        "Part 1: Fresh ingredient count is {}",
        fresh_ingredient_count
    );

    let collapsed_ranges = collapse_ranges(ranges);

    let mut total_fresh_ingredients = 0;
    for range in collapsed_ranges {
        total_fresh_ingredients += range.1 - range.0 + 1;
    }

    println!(
        "Part 2: Fresh ingredient count is {}",
        total_fresh_ingredients
    );
}

fn is_fresh(ranges: &Vec<(u64, u64)>, ingredient: u64) -> bool {
    ranges
        .iter()
        .any(|(min, max)| *min <= ingredient && ingredient <= *max)
}

fn collapse_ranges(ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut processed_ranges: Vec<Option<(u64, u64)>> = Vec::new();

    for range in ranges {
        processed_ranges.push(Some(range));
    }

    let mut last_size = None;

    while Some(count_non_optionals(&processed_ranges)) != last_size {
        last_size = Some(count_non_optionals(&processed_ranges));
        for range_a_index in 0..processed_ranges.len() {
            for range_b_index in 0..processed_ranges.len() {
                if range_a_index == range_b_index {
                    continue;
                }
                if let Some(range_a) = &processed_ranges[range_a_index]
                    && let Some(range_b) = &processed_ranges[range_b_index]
                {
                    if a_superset_of_b(range_a, range_b) {
                        processed_ranges[range_b_index] = None;
                    } else if a_subset_of_b(range_a, range_b) {
                        processed_ranges[range_a_index] = None;
                    } else if a_overlaps_b(range_a, range_b) {
                        processed_ranges[range_a_index] = Some((
                            cmp::min(range_a.0, range_b.0),
                            cmp::max(range_a.1, range_b.1),
                        ));
                        processed_ranges[range_b_index] = None;
                    }
                }
            }
        }
    }

    let mut collapsed_ranges = Vec::new();
    for maybe_range in processed_ranges {
        if let Some(range) = maybe_range {
            collapsed_ranges.push(range);
        }
    }

    collapsed_ranges
}

fn a_subset_of_b(a: &(u64, u64), b: &(u64, u64)) -> bool {
    a.0 >= b.0 && a.1 <= b.1
}

fn a_superset_of_b(a: &(u64, u64), b: &(u64, u64)) -> bool {
    a_subset_of_b(b, a)
}

fn a_overlaps_b(a: &(u64, u64), b: &(u64, u64)) -> bool {
    (a.0 <= b.0 && b.0 <= a.1) || (a.0 <= b.1 && b.1 <= a.1)
}

fn count_non_optionals<T>(container: &Vec<Option<T>>) -> usize {
    let mut non_optionals = 0;
    for item in container {
        if item.is_some() {
            non_optionals += 1;
        }
    }

    non_optionals
}
