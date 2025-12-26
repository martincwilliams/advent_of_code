use std::io::{self, BufRead};

fn main() {
    let mut grid = Vec::new();

    for line in io::stdin().lock().lines() {
        grid.push(line.expect("No line").into_bytes());
    }

    let mut accessible_rolls_part_1 = 0;

    let rows = grid.len();
    let cols = grid[0].len();

    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] == b'@' && count_neighbours(&grid, row, col) < 4 {
                accessible_rolls_part_1 += 1;
            }
        }
    }

    println!(
        "Part 1: There are {} accessible rolls",
        accessible_rolls_part_1
    );

    let mut accessible_rolls_part_2 = 0;
    let mut accessible_rolls_at_last_iteration = None;

    while Some(accessible_rolls_part_2) != accessible_rolls_at_last_iteration {
        accessible_rolls_at_last_iteration = Some(accessible_rolls_part_2);
        for row in 0..rows {
            for col in 0..cols {
                if grid[row][col] == b'@' && count_neighbours(&grid, row, col) < 4 {
                    accessible_rolls_part_2 += 1;
                    grid[row][col] = b'.';
                }
            }
        }
    }

    println!(
        "Part 2: There are {} accessible rolls",
        accessible_rolls_part_2
    );
}

fn count_neighbours(grid: &Vec<Vec<u8>>, row: usize, col: usize) -> u32 {
    let mut num_neighbours = 0;

    let signed_row = row as i32;
    let signed_col = col as i32;

    if grid_element(grid, signed_row - 1, signed_col - 1) == Some(b'@') {
        num_neighbours += 1
    }
    if grid_element(grid, signed_row - 1, signed_col) == Some(b'@') {
        num_neighbours += 1
    }
    if grid_element(grid, signed_row - 1, signed_col + 1) == Some(b'@') {
        num_neighbours += 1
    }
    if grid_element(grid, signed_row, signed_col - 1) == Some(b'@') {
        num_neighbours += 1
    }
    if grid_element(grid, signed_row, signed_col + 1) == Some(b'@') {
        num_neighbours += 1
    }
    if grid_element(grid, signed_row + 1, signed_col - 1) == Some(b'@') {
        num_neighbours += 1
    }
    if grid_element(grid, signed_row + 1, signed_col) == Some(b'@') {
        num_neighbours += 1
    }
    if grid_element(grid, signed_row + 1, signed_col + 1) == Some(b'@') {
        num_neighbours += 1
    }

    num_neighbours
}

fn grid_element(grid: &Vec<Vec<u8>>, row: i32, col: i32) -> Option<u8> {
    if row >= 0
        && (row as usize) < grid.len()
        && col >= 0
        && (col as usize) < grid[row as usize].len()
    {
        Some(grid[row as usize][col as usize])
    } else {
        None
    }
}
