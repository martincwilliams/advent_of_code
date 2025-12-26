use std::io::{self, BufRead};

fn main() {
    let mut grid = read_grid();

    show_grid(&grid);

    let start_position = find_start(&grid);

    let width = grid[0].len();

    for current_row in start_position.0 + 1..grid.len() {
        for col in 0..width {
            grid[current_row][col] = match grid[current_row][col] {
                b'.' => {
                    if is_tachyon_beam_at(&grid, current_row, col, -1, 0) {
                        // Tachyon beam falls
                        b'|'
                    } else if is_splitter_at(&grid, current_row, col, 0, 1)
                        && is_tachyon_beam_at(&grid, current_row, col, -1, 1)
                    {
                        b'|'
                    } else if is_splitter_at(&grid, current_row, col, 0, -1)
                        && is_tachyon_beam_at(&grid, current_row, col, -1, -1)
                    {
                        b'|'
                    } else {
                        b'.'
                    }
                }
                x => x,
            };
        }
        show_grid(&grid);
    }

    let num_splits = find_splits(&grid);

    show_grid(&grid);

    println!("Part 1: num splits = {}", num_splits);

    let mut timeline_grid = vec![vec![0; width]; grid.len()];
    timeline_grid[start_position.0][start_position.1] = 1;
    for current_row in start_position.0 + 1..grid.len() {
        for col in 0..width {
            if grid[current_row][col] == b'|' {
                let mut increase = 0;
                if is_tachyon_beam_at(&grid, current_row, col, -1, 0) {
                    increase +=
                        timeline_grid_at(&timeline_grid, current_row, col, -1, 0).unwrap_or(0);
                }
                if is_splitter_at(&grid, current_row, col, 0, 1)
                    && is_tachyon_beam_at(&grid, current_row, col, -1, 1)
                {
                    increase +=
                        timeline_grid_at(&timeline_grid, current_row, col, -1, 1).unwrap_or(0);
                }
                if is_splitter_at(&grid, current_row, col, 0, -1)
                    && is_tachyon_beam_at(&grid, current_row, col, -1, -1)
                {
                    increase +=
                        timeline_grid_at(&timeline_grid, current_row, col, -1, -1).unwrap_or(0);
                }
                timeline_grid[current_row][col] += increase;
            }
        }
    }

    let total_timelines: u64 = timeline_grid.last().unwrap().iter().sum();
    println!("Part 2: num timelines = {}", total_timelines);
}

fn read_grid() -> Vec<Vec<u8>> {
    let mut grid = Vec::new();

    let mut input_iterator = io::stdin().lock().lines();
    while let Some(line) = input_iterator.next() {
        grid.push(line.expect("No line").into_bytes());
    }

    grid
}

fn show_grid(grid: &Vec<Vec<u8>>) {
    for row in grid {
        for elem in row {
            print!("{}", *elem as char);
        }
        println!("");
    }
    println!("");
}

fn find_start(grid: &Vec<Vec<u8>>) -> (usize, usize) {
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == b'S' {
                return (row, col);
            }
        }
    }

    panic!("Could not find start position");
}

fn find_splits(grid: &Vec<Vec<u8>>) -> u32 {
    let mut num_splits = 0;

    for row in 1..grid.len() {
        for col in 1..(grid[row].len() - 1) {
            if is_splitter_at(grid, row, col, 0, 0) {
                // There is a split if there is a tachyon beam comin+
                // g down on the splitter
                if is_tachyon_beam_at(grid, row, col, -1, 0) {
                    num_splits += 1
                }
            }
        }
    }

    num_splits
}

fn is_tachyon_beam_at(
    grid: &Vec<Vec<u8>>,
    row: usize,
    col: usize,
    row_offset: i32,
    col_offset: i32,
) -> bool {
    let potential_at = at(grid, row, col, row_offset, col_offset);
    potential_at == Some(b'S') || potential_at == Some(b'|')
}

fn is_splitter_at(
    grid: &Vec<Vec<u8>>,
    row: usize,
    col: usize,
    row_offset: i32,
    col_offset: i32,
) -> bool {
    at(grid, row, col, row_offset, col_offset) == Some(b'^')
}

fn at(grid: &Vec<Vec<u8>>, row: usize, col: usize, row_offset: i32, col_offset: i32) -> Option<u8> {
    let at_row = row as i32 + row_offset;
    let at_col = col as i32 + col_offset;

    if at_row < 0 || at_row >= grid.len() as i32 || at_col < 0 || at_col >= grid[0].len() as i32 {
        None
    } else {
        Some(grid[at_row as usize][at_col as usize])
    }
}

fn timeline_grid_at(
    timeline_grid: &Vec<Vec<u64>>,
    row: usize,
    col: usize,
    row_offset: i32,
    col_offset: i32,
) -> Option<u64> {
    let at_row = row as i32 + row_offset;
    let at_col = col as i32 + col_offset;

    if at_row < 0
        || at_row >= timeline_grid.len() as i32
        || at_col < 0
        || at_col >= timeline_grid[0].len() as i32
    {
        None
    } else {
        Some(timeline_grid[at_row as usize][at_col as usize])
    }
}
