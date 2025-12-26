use std::collections::BTreeSet;
use std::collections::HashMap;
use std::env;
use std::io::{self, BufRead};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Missing arguments: specify the origin device, and then all devices that must be passed through");
        process::exit(1);
    }

    let origin_device_name = &args[1];
    let mut must_pass_by = BTreeSet::new();
    for arg in args.iter().skip(2) {
        must_pass_by.insert(arg.as_str());
    }

    println!(
        "Starting from '{}', passing through {:?}",
        origin_device_name, must_pass_by
    );

    let graph = parse_input();

    println!(
        "Different paths = {}",
        depth_first_find_paths_to_goal(&graph, origin_device_name, "out", &must_pass_by)
    );
}

fn depth_first_find_paths_to_goal(
    g: &Graph,
    origin: &str,
    goal: &str,
    must_pass_by: &BTreeSet<&str>,
) -> u64 {
    let mut memo = HashMap::new();
    depth_first_find_paths_to_goal_with_memoisation(g, origin, goal, must_pass_by, &mut memo)
}

fn depth_first_find_paths_to_goal_with_memoisation<'a>(
    g: &'a Graph,
    origin: &'a str,
    goal: &str,
    must_pass_by: &BTreeSet<&'a str>,
    memo: &mut HashMap<(&'a str, BTreeSet<&'a str>), u64>,
) -> u64 {
    let current = g.nodes.get(origin).unwrap();

    if origin == goal {
        return if must_pass_by.is_empty() { 1 } else { 0 };
    }

    // Check whether we have the current state in the memo
    if let Some(paths) = memo.get(&(origin, must_pass_by.clone())) {
        return *paths;
    }

    let mut must_pass_by_updated = must_pass_by.clone();
    must_pass_by_updated.remove(current.label.as_str());

    let mut paths_to_out_found = 0;
    for edge in &current.edges {
        paths_to_out_found += depth_first_find_paths_to_goal_with_memoisation(
            g,
            &edge,
            goal,
            &must_pass_by_updated,
            memo,
        );
    }

    // Memoise this result
    memo.insert((origin, must_pass_by.clone()), paths_to_out_found);

    paths_to_out_found
}

fn parse_input() -> Graph {
    let mut graph = Graph::new();

    let mut input_iterator = io::stdin().lock().lines();
    while let Some(input_line) = input_iterator.next() {
        let line = input_line.unwrap();

        let fields: Vec<&str> = line.split_whitespace().collect();

        let device_name = &fields[0][0..fields[0].len() - 1];
        let mut device = Node::new(&device_name);
        for output in fields.iter().skip(1) {
            device.add_edge(output);
        }
        graph.add_node(device);
    }

    // The "out" node is not explicitly defined in the input
    graph.add_node(Node::new("out"));

    graph
}

struct Node {
    pub label: String,
    pub edges: Vec<String>,
}

impl Node {
    pub fn new(label: &str) -> Node {
        Node {
            label: String::from(label),
            edges: Vec::new(),
        }
    }

    pub fn add_edge(&mut self, edge: &str) {
        self.edges.push(String::from(edge));
    }
}

struct Graph {
    pub nodes: HashMap<String, Node>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            nodes: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.label.clone(), node);
    }
}
