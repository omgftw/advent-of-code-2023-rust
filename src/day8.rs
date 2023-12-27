use std::collections::HashMap;
use std::fs;
use num_integer::lcm;

pub(crate) async fn day8(data: Option<String>, part2: bool) -> i64 {
    let data = data.unwrap_or_else(|| fs::read_to_string("src/data/8.txt").unwrap());

    let parts = data.split("\n\n").collect::<Vec<&str>>();
    let instructions = parts[0]
        .replace('L', "0")
        .replace('R', "1");
    let instructions = instructions
        .chars()
        .map(|c| c.to_digit(2).unwrap() as usize);

    let mut nodes = HashMap::new();
    for node_item in parts[1].lines() {
        let node_parts = node_item.split(" = ").collect::<Vec<&str>>();
        let node_name = node_parts[0];
        let node_instr_parts = node_parts[1].split(", ").collect::<Vec<&str>>();
        let node_left = node_instr_parts[0].replace('(', "");
        let node_right = node_instr_parts[1].replace(')', "");
        nodes.insert(node_name, [node_left, node_right]);
    }

    if !part2 {
        let mut iterations = 0;
        let mut cur_node = "AAA";
        'top: loop {
            for instruction in instructions.clone() {
                iterations += 1;
                cur_node = &nodes.get(cur_node).unwrap()[instruction];
                if cur_node == "ZZZ" {
                    break 'top;
                }
            }
        }
        return iterations;
    }

    let mut cur_nodes = vec![];
    for node in nodes.keys() {
        if node.ends_with('A') {
            cur_nodes.push(*node);
        }
    }

    let mut iterations = Vec::new();
    for mut node in cur_nodes {
        let mut node_iter = 0;
        'outer: loop {
            for instruction in instructions.clone() {
                node_iter += 1;
                let next_node = &nodes.get(node).unwrap()[instruction];
                if next_node.ends_with('Z') {
                    iterations.push(node_iter);
                    break 'outer;
                }
                node = next_node;
            }
        }
    }

    iterations.iter().fold(1, |acc, x| lcm(acc, *x))
}