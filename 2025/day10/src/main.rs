use itertools::Itertools;
use lp_modeler::constraint;
use lp_modeler::dsl::*;
use lp_modeler::solvers::{CbcSolver, SolverTrait};
use std::io::{self, BufRead};

struct Button {
    pub wiring: Vec<usize>,
}

struct Machine {
    pub indicators_for_start: Vec<bool>,
    pub buttons: Vec<Button>,
    pub joltage_levels: Vec<usize>,
}

fn main() {
    let machines = parse_input();

    for machine in &machines {
        print_machine(machine);
    }

    println!(
        "Part 1: total_button_presses = {}",
        total_button_presses_for_indicators(&machines)
    );

    println!(
        "Part 2: total_button_presses = {}",
        total_button_presses_for_joltages(&machines)
    );
}

fn total_button_presses_for_indicators(machines: &Vec<Machine>) -> usize {
    machines.iter().fold(0, |acc, machine| {
        acc + least_button_presses_for_indicators(&machine)
    })
}

fn least_button_presses_for_indicators(machine: &Machine) -> usize {
    let num_buttons = machine.buttons.len();
    let mut num_button_presses = 1;

    loop {
        let buttons: Vec<usize> = (0..num_buttons).collect();

        for combo in buttons.iter().combinations(num_button_presses) {
            let mut distribution = vec![0; num_buttons];
            for button in combo {
                distribution[*button] = 1;
            }
            if check_combination(machine, &distribution) {
                return num_button_presses;
            }
        }
        num_button_presses += 1;
    }
}

fn check_combination(machine: &Machine, button_presses: &Vec<usize>) -> bool {
    let mut indicator_lights = vec![false; machine.indicators_for_start.len()];

    for button_index in 0..button_presses.len() {
        for connection in &machine.buttons[button_index].wiring {
            indicator_lights[*connection] =
                indicator_lights[*connection] != (button_presses[button_index] % 2 == 1);
        }
    }

    indicator_lights == machine.indicators_for_start
}

fn total_button_presses_for_joltages(machines: &Vec<Machine>) -> usize {
    machines.iter().fold(0, |acc, machine| {
        acc + least_button_presses_for_joltages(&machine)
    })
}

fn least_button_presses_for_joltages(machine: &Machine) -> usize {
    let mut problem = LpProblem::new("button_presses", LpObjective::Minimize);

    let mut variables: Vec<LpInteger> = Vec::new();
    let mut objective_function: LpExpression = 0.into();
    for button_index in 0..machine.buttons.len() {
        let var_name = format!("x{}", button_index);
        variables.push(LpInteger::new(&var_name));
        objective_function += &variables[button_index];
    }
    problem += objective_function;

    for variable in &variables {
        problem += constraint!(variable >= 0);
    }

    let mut coefficients = vec![vec![0; machine.buttons.len()]; machine.joltage_levels.len()];

    for button_index in 0..machine.buttons.len() {
        for connection in &machine.buttons[button_index].wiring {
            coefficients[*connection][button_index] = 1;
        }
    }

    for (row, rhs) in coefficients.iter().zip(machine.joltage_levels.iter()) {
        let mut expr: LpExpression = 0.into();

        for (coef, var) in row.iter().zip(variables.iter()) {
            expr += *coef * var;
        }
        problem += expr.equal(*rhs as i32);
    }

    let solver = CbcSolver::new();
    match solver.run(&problem) {
        Ok(solution) => solution.results.values().sum::<f32>() as usize,
        Err(msg) => panic!("{msg}"),
    }
}

fn parse_input() -> Vec<Machine> {
    let mut machines = Vec::new();

    let mut input_iterator = io::stdin().lock().lines();
    while let Some(input_line) = input_iterator.next() {
        let line = input_line.unwrap();
        let line_elements: Vec<_> = line.split_whitespace().collect();

        machines.push(Machine {
            indicators_for_start: parse_indicators(&line_elements[0]),
            buttons: parse_buttons(
                &mut line_elements
                    .iter()
                    .skip(1)
                    .take(line_elements.len() - 2)
                    .copied(),
            ),
            joltage_levels: parse_joltage_levels(&line_elements[line_elements.len() - 1]),
        });
    }

    machines
}

fn parse_indicators(s: &str) -> Vec<bool> {
    if s.is_empty() || s.chars().nth(0).unwrap() != '[' || s.chars().last().unwrap() != ']' {
        panic!("Unexpected indicator input: \"{}\"", s);
    }

    s.chars()
        .skip(1)
        .take(s.len() - 2)
        .map(|indicator_char| indicator_char == '#')
        .collect::<Vec<_>>()
}

fn parse_buttons<'a>(buttons_input_iter: impl Iterator<Item = &'a str>) -> Vec<Button> {
    buttons_input_iter
        .map(|input| {
            let inner = &input[1..input.len() - 1];

            Button {
                wiring: inner
                    .split(',')
                    .map(|indicator_id| indicator_id.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
            }
        })
        .collect()
}

fn parse_joltage_levels(s: &str) -> Vec<usize> {
    if s.is_empty() || s.chars().nth(0).unwrap() != '{' || s.chars().last().unwrap() != '}' {
        panic!("Unexpected joltage level input: \"{}\"", s);
    }

    s[1..s.len() - 1]
        .split(',')
        .map(|level_str| level_str.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
}

fn print_machine(machine: &Machine) {
    print!("[");
    for indicator in &machine.indicators_for_start {
        print!(
            "{}",
            match indicator {
                true => '#',
                false => '.',
            }
        );
    }
    print!("] ");
    for button in &machine.buttons {
        print!("{:?} ", button.wiring);
    }
    println!("");
}
