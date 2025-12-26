use std::io::{self, BufRead};

#[derive(Debug)]
enum Operation {
    Multiply,
    Add,
}

fn main() {
    /*
    let (table, operations) = read_table_part_1();

    let mut total = 0;

    for col in 0..operations.len() {
        total += match &operations[col] {
            Operation::Multiply => {
                let mut subtotal = 1;
                for row in &table {
                    subtotal *= row[col];
                }
                subtotal
            }
            Operation::Add => {
                let mut subtotal = 0;
                for row in &table {
                    subtotal += row[col];
                }
                subtotal
            }
        }
    }

    println!("Part 1: total = {}", total);
    */

    let (table, operations) = read_table_part_2();

    let mut total = 0;

    let mut subtotal = 0;

    let mut current_operation = None;

    for operation_line_index in 0..operations.len() {
        if let Some(operation) = byte_to_operator(&operations[operation_line_index]) {
            println!("Subtotal = {subtotal}");
            total += subtotal;
            subtotal = match operation {
                Operation::Multiply => 1,
                Operation::Add => 0,
            };
            println!("Next operation = {:?}", operation);
            current_operation = Some(operation);
        }

        if let Some(number) = number_in_column(&table, operation_line_index) {
            println!("Number = {}", number);
            subtotal = match current_operation {
                Some(Operation::Multiply) => subtotal * number,
                Some(Operation::Add) => subtotal + number,
                _ => panic!("No current operation!"),
            };
        }
    }

    // Add the final operation's subtotal
    println!("Subtotal = {subtotal}");
    total += subtotal;

    println!("Part 2: total = {}", total);
}

fn read_table_part_1() -> (Vec<Vec<u64>>, Vec<Operation>) {
    let mut table = Vec::new();
    let mut operations = Vec::new();

    let mut input_iterator = io::stdin().lock().lines();
    while let Some(input_line) = input_iterator.next() {
        let line = input_line.unwrap();

        if let Ok(row) = line
            .split_whitespace()
            .map(|elem| elem.parse::<u64>())
            .collect()
        {
            table.push(row);
        } else {
            operations = line
                .split_whitespace()
                .map(|elem| string_to_operator(&elem))
                .collect();
        }
    }

    (table, operations)
}

fn read_table_part_2() -> (Vec<Vec<u8>>, Vec<u8>) {
    let mut table = Vec::new();
    let mut operations = Vec::new();

    let mut input_iterator = io::stdin().lock().lines();
    while let Some(input_line) = input_iterator.next() {
        let line = input_line.expect("No line");
        let line_as_bytes = line.into_bytes();

        if byte_to_operator(&line_as_bytes[0]).is_some() {
            operations = line_as_bytes;
        } else {
            table.push(line_as_bytes);
        }
    }

    (table, operations)
}

fn string_to_operator(s: &str) -> Operation {
    match s {
        "*" => Operation::Multiply,
        "+" => Operation::Add,
        op => panic!("Unsupported operator {op}"),
    }
}

fn byte_to_operator(ch: &u8) -> Option<Operation> {
    match ch {
        b'*' => Some(Operation::Multiply),
        b'+' => Some(Operation::Add),
        _ => None,
    }
}

fn number_in_column(table: &Vec<Vec<u8>>, column_index: usize) -> Option<u64> {
    let mut column_value = 0;
    let mut found_digit = false;
    for row in table {
        if let Some(digit) = get_digit(&row[column_index]) {
            column_value = column_value * 10 + digit;
            found_digit = true;
        }
    }

    if found_digit {
        Some(column_value)
    } else {
        None
    }
}

fn get_digit(ch: &u8) -> Option<u64> {
    if *ch >= b'0' && *ch <= b'9' {
        Some((ch - b'0') as u64)
    } else {
        None
    }
}
