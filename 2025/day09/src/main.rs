use indicatif::ProgressBar;
use std::cmp;
use std::io::{self, BufRead};

struct Coords {
    pub x: u64,
    pub y: u64,
}

#[derive(Clone, PartialEq)]
enum Tile {
    RedOrGreen,
    Other,
}

fn main() {
    let mut input_iterator = io::stdin().lock().lines();

    let mut red_square_positions = Vec::new();

    while let Some(line) = input_iterator.next() {
        let coordinates: Vec<_> = line
            .unwrap()
            .split(',')
            .map(|elem| elem.parse::<u64>().unwrap())
            .collect();

        red_square_positions.push(Coords {
            x: coordinates[0],
            y: coordinates[1],
        });
    }

    let mut combination_areas = Vec::new();
    for position1 in &red_square_positions {
        for position2 in &red_square_positions {
            combination_areas.push(area(position1, position2));
        }
    }

    let max_area = combination_areas.iter().max().unwrap();
    println!("Part 1: max area = {}", max_area);

    let width = red_square_positions
        .iter()
        .max_by(|position_a, position_b| position_a.x.cmp(&position_b.x))
        .unwrap()
        .x as usize
        + 1;
    let height = red_square_positions
        .iter()
        .max_by(|position_a, position_b| position_a.y.cmp(&position_b.y))
        .unwrap()
        .y as usize
        + 1;

    let mut grid = vec![vec![Tile::Other; width]; height];
    for (position1, position2) in red_square_positions
        .iter()
        .zip(red_square_positions.iter().skip(1))
    {
        fill_between(&mut grid, position1, position2);
    }
    // The line between last and first to close the outline
    fill_between(
        &mut grid,
        red_square_positions.last().unwrap(),
        red_square_positions.first().unwrap(),
    );

    fill_outline(&mut grid);

    //draw_grid(&grid);

    let bar = ProgressBar::new(red_square_positions.len().pow(2) as u64);

    let mut combination_areas_over_red_green = Vec::new();
    for position1 in &red_square_positions {
        for position2 in &red_square_positions {
            if is_over_red_or_green_tiles(&grid, position1, position2) {
                combination_areas_over_red_green.push(area(position1, position2));
            }
            bar.inc(1);
        }
    }
    bar.finish();

    let max_area_over_red_green = combination_areas_over_red_green.iter().max().unwrap();
    println!("Part 2: max area = {}", max_area_over_red_green);
}

fn area(position1: &Coords, position2: &Coords) -> u64 {
    ((position2.x as i64 - position1.x as i64).abs() as u64 + 1)
        * ((position2.y as i64 - position1.y as i64).abs() as u64 + 1)
}

fn fill_between(grid: &mut Vec<Vec<Tile>>, position1: &Coords, position2: &Coords) {
    if position1.x == position2.x {
        for y in cmp::min(position1.y, position2.y)..=cmp::max(position1.y, position2.y) {
            grid[y as usize][position1.x as usize] = Tile::RedOrGreen;
        }
    } else if position1.y == position2.y {
        for x in cmp::min(position1.x, position2.x)..=cmp::max(position1.x, position2.x) {
            grid[position1.y as usize][x as usize] = Tile::RedOrGreen;
        }
    } else {
        panic!("Not expecting cosecutive points that are not in a row or column");
    }
}

fn fill_outline(grid: &mut Vec<Vec<Tile>>) {
    for row in grid.iter_mut() {
        let mut fill_on = false;
        let mut accept_fill_change = true;
        for cell in row.iter_mut() {
            if *cell == Tile::RedOrGreen && accept_fill_change {
                fill_on = !fill_on;
                accept_fill_change = false;
            }

            if *cell == Tile::Other {
                if fill_on {
                    *cell = Tile::RedOrGreen;
                }
                accept_fill_change = true;
            }
        }
    }
}

fn draw_grid(grid: &Vec<Vec<Tile>>) {
    for row in grid {
        for cell in row {
            print!(
                "{}",
                match cell {
                    Tile::RedOrGreen => 'X',
                    Tile::Other => '.',
                }
            );
        }
        println!("");
    }
}

fn is_over_red_or_green_tiles(
    grid: &Vec<Vec<Tile>>,
    position1: &Coords,
    position2: &Coords,
) -> bool {
    // There are no holes in the red/green tile coverage => only need to check outline
    let (min_row, max_row) = (
        cmp::min(position1.y, position2.y) as usize,
        cmp::max(position1.y, position2.y) as usize,
    );
    let (min_col, max_col) = (
        cmp::min(position1.x, position2.x) as usize,
        cmp::max(position1.x, position2.x) as usize,
    );

    // Top and bottom sections of border
    for col in min_col..=max_col {
        if grid[min_row][col] != Tile::RedOrGreen || grid[max_row][col] != Tile::RedOrGreen {
            return false;
        }
    }

    // Left and right sections of border
    for row in min_row..=max_row {
        if grid[row][min_col] != Tile::RedOrGreen || grid[row][max_col] != Tile::RedOrGreen {
            return false;
        }
    }

    true
}
