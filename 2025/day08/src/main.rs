use std::collections::HashSet;
use std::io::{self, BufRead};

struct Coords {
    pub x: u64,
    pub y: u64,
    pub z: u64,
}

fn main() {
    let mut input_iterator = io::stdin().lock().lines();

    let mut junction_boxes = Vec::new();

    while let Some(line) = input_iterator.next() {
        let coordinates: Vec<_> = line
            .unwrap()
            .split(',')
            .map(|elem| elem.parse::<u64>().unwrap())
            .collect();

        junction_boxes.push(Coords {
            x: coordinates[0],
            y: coordinates[1],
            z: coordinates[2],
        });
    }

    let mut distances = calculate_distances(&junction_boxes);

    distances.sort_by(|((_, _), a), ((_, _), b)| a.partial_cmp(b).unwrap());

    /*
    for dist in &distances {
        let ((source, destination), d) = dist;
        println!(
            "From {},{},{} to {},{},{} dist is {}",
            junction_boxes[*source].x,
            junction_boxes[*source].y,
            junction_boxes[*source].z,
            junction_boxes[*destination].x,
            junction_boxes[*destination].y,
            junction_boxes[*destination].z,
            d
        );
    }
    */

    let num_connections = 1000;
    println!(
        "Part 1: {}",
        part_1(&junction_boxes, &distances, num_connections)
    );

    println!("Part 2: {}", part_2(&junction_boxes, &distances));
}

fn part_1(
    junction_boxes: &Vec<Coords>,
    distances: &Vec<((usize, usize), f64)>,
    num_connections: usize,
) -> usize {
    let mut groups = Vec::new();
    for box_index in 0..junction_boxes.len() {
        groups.push(HashSet::from([box_index]));
    }

    for connection_index in 0..num_connections {
        let (source, destination) = distances[connection_index].0;
        println!(
            "Connect {},{},{} to {},{},{}",
            junction_boxes[source].x,
            junction_boxes[source].y,
            junction_boxes[source].z,
            junction_boxes[destination].x,
            junction_boxes[destination].y,
            junction_boxes[destination].z
        );
        groups = merge_groups(groups, source, destination);
    }

    let mut group_sizes: Vec<_> = groups.iter().map(|group| group.len()).collect();
    group_sizes.sort_by(|a, b| b.cmp(a));

    group_sizes[0] * group_sizes[1] * group_sizes[2]
}

fn part_2(junction_boxes: &Vec<Coords>, distances: &Vec<((usize, usize), f64)>) -> u64 {
    let mut groups = Vec::new();
    for box_index in 0..junction_boxes.len() {
        groups.push(HashSet::from([box_index]));
    }

    let mut connection_index = 0;
    while groups.len() > 1 {
        let (source, destination) = distances[connection_index].0;
        println!(
            "Connect {},{},{} to {},{},{}",
            junction_boxes[source].x,
            junction_boxes[source].y,
            junction_boxes[source].z,
            junction_boxes[destination].x,
            junction_boxes[destination].y,
            junction_boxes[destination].z
        );
        groups = merge_groups(groups, source, destination);
        connection_index += 1;
    }

    let (source, destination) = distances[connection_index - 1].0;
    junction_boxes[source].x * junction_boxes[destination].x
}

fn merge_groups(
    input_groups: Vec<HashSet<usize>>,
    element1: usize,
    element2: usize,
) -> Vec<HashSet<usize>> {
    // This is a hugely inefficient implementation but for the provided data it works quickly
    // enough. For uses on larger data sets, a union-find structure might be a good bet.
    let mut output_groups = Vec::new();
    let mut groups_left = input_groups.len();
    let mut elem1_index = None;
    let mut elem2_index = None;
    for index in 0..input_groups.len() {
        if !input_groups[index].contains(&element1) && !input_groups[index].contains(&element2) {
            output_groups.push(input_groups[index].clone());
            groups_left -= 1;
        } else {
            if input_groups[index].contains(&element1) {
                elem1_index = Some(index);
            }
            if input_groups[index].contains(&element2) {
                elem2_index = Some(index);
            }
        }
    }

    if groups_left == 1 {
        // The elements are already combined
        output_groups.push(input_groups[elem1_index.unwrap()].clone());
    } else if groups_left == 2 {
        let union: HashSet<_> = input_groups[elem1_index.unwrap()]
            .union(&input_groups[elem2_index.unwrap()])
            .cloned()
            .collect();
        output_groups.push(union);
    } else {
        panic!("What! There are {} groups left!", groups_left);
    }

    output_groups
}

fn calculate_distances(junction_boxes: &Vec<Coords>) -> Vec<((usize, usize), f64)> {
    let mut distances = Vec::new();

    for source in 0..junction_boxes.len() {
        for destination in source + 1..junction_boxes.len() {
            distances.push((
                (source, destination),
                distance(&junction_boxes[source], &junction_boxes[destination]),
            ));
        }
    }

    distances
}

fn distance(coords_a: &Coords, coords_b: &Coords) -> f64 {
    ((coords_b.x as f64 - coords_a.x as f64).powf(2.0)
        + (coords_b.y as f64 - coords_a.y as f64).powf(2.0)
        + (coords_b.z as f64 - coords_a.z as f64).powf(2.0))
    .sqrt()
}
