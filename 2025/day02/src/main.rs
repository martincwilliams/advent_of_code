use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut total = (0, 0);

    for item in input.trim().split(',') {
        println!("Item: {}", item.trim());
        let parts: Vec<&str> = item.split('-').collect();
        let (first_id, last_id) = (
            parts[0].parse::<u64>().unwrap(),
            parts[1].parse::<u64>().unwrap(),
        );
        let subtotal = search_id_range_for_invalid(first_id, last_id);
        total = (total.0 + subtotal.0, total.1 + subtotal.1);
    }

    println!("Part 1: total = {}", total.0);
    println!("Part 2: total = {}", total.1);
}

fn search_id_range_for_invalid(first_id: u64, last_id: u64) -> (u64, u64) {
    let mut total_of_invalid_ids = (0, 0);

    for id in first_id..=last_id {
        if is_id_invalid_part_1(id) {
            // println!("{}", id);
            total_of_invalid_ids.0 += id;
        }
        if is_id_invalid_part_2(id) {
            //println!("{}", id);
            total_of_invalid_ids.1 += id;
        }
    }

    total_of_invalid_ids
}

fn is_id_invalid_part_1(id: u64) -> bool {
    let half_num_digits: u32 = (num_digits_in_id(id) / 2) as u32;
    id % 10u64.pow(half_num_digits) == id / 10u64.pow(half_num_digits)
}

fn is_id_invalid_part_2(id: u64) -> bool {
    let num_digits = num_digits_in_id(id) as u32;

    let id_as_string = id.to_string();

    for num_splits in 2..=num_digits {
        let group_size = (num_digits / num_splits) as u32;

        if group_size * num_splits != num_digits {
            // Not even divisible by in num_splits parts
            continue;
        }

        let search_pattern: &str = &id_as_string[..group_size as usize];

        if are_repetitions(&id_as_string, search_pattern) {
            return true;
        }
    }

    false
}

fn num_digits_in_id(num: u64) -> usize {
    num.to_string().len()
}

fn are_repetitions(s: &str, pattern: &str) -> bool {
    let group_size = pattern.len();
    let num_groups = s.len() / group_size;

    for i in 1..num_groups {
        if &s[(i * group_size)..((i + 1) * group_size)] != pattern {
            return false;
        }
    }

    true
}
