use utils;
use regex::Regex;
use std::cmp;
use std::collections::HashMap;

fn main() {
    let raw_input = utils::lines_from_file("day08");
    let p1: usize = calc_p1(&raw_input);
    let p2: u64 = calc_p2(&raw_input);
    println!("p1: {}", p1);
    println!("p2: {}", p2);
}


fn calc_p1(raw_input: &Vec<String>) -> usize {
    // get instructions from first line of raw_input
    let instructions: Vec<char> = raw_input[0].chars().collect();

    // build mapping from third line onwards of raw_input
    let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
    let mut mapping: HashMap<String, Children> = HashMap::new();
    for line in raw_input[2..].iter() {
        let caps = re.captures(line).unwrap();
        let parent = caps.get(1).unwrap().as_str().to_string();
        let left = caps.get(2).unwrap().as_str().to_string();
        let right = caps.get(3).unwrap().as_str().to_string();
        mapping.insert(parent, Children{left: left, right: right});
    }

    let mut current_node = "AAA".to_string();
    let mut n_steps: usize = 0;
    let mut instruction_index: usize = 0;
    while current_node != "ZZZ".to_string() {
        let children = mapping.get(&current_node).unwrap();
        if instructions[instruction_index] == 'R' {
            current_node = children.right.clone();
        } else {
            current_node = children.left.clone();
        }
        n_steps += 1;
        instruction_index = (instruction_index + 1) % instructions.len();
    }
    n_steps
}

fn calc_p2(raw_input: &Vec<String>) -> u64 {
    // get instructions from first line of raw_input
    let instructions: Vec<char> = raw_input[0].chars().collect();

    // build mapping from third line onwards of raw_input
    let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
    let mut mapping: HashMap<String, Children> = HashMap::new();
    for line in raw_input[2..].iter() {
        let caps = re.captures(line).unwrap();
        let parent = caps.get(1).unwrap().as_str().to_string();
        let left = caps.get(2).unwrap().as_str().to_string();
        let right = caps.get(3).unwrap().as_str().to_string();
        mapping.insert(parent, Children{left: left, right: right});
    }

    // find all nodes with "A" in
    let mut nodes_with_a: Vec<String> = Vec::new();
    for (node, children) in mapping.iter() {
        if node.contains("A") {
            nodes_with_a.push(node.clone());
        }
    }
    println!("nodes_with_a: {:?}", nodes_with_a);



    for current_node in nodes_with_a.iter() {
        let mut n_steps: u64 = 0;
        let mut current_node = current_node.clone();
        let mut instruction_index: usize = 0;
        while current_node != "ZZZ".to_string() {
            let children = mapping.get(&current_node).unwrap();
            if instructions[instruction_index] == 'R' {
                current_node = children.right.clone();
            } else {
                current_node = children.left.clone();
            }
            n_steps += 1;
            instruction_index = (instruction_index + 1) % instructions.len();
        }
    }
    // let mut instruction_index: usize = 0;
    // while current_node != "ZZZ".to_string() {
    //     let children = mapping.get(&current_node).unwrap();
    //     if instructions[instruction_index] == 'R' {
    //         current_node = children.right.clone();
    //     } else {
    //         current_node = children.left.clone();
    //     }
    //     n_steps += 1;
    //     instruction_index = (instruction_index + 1) % instructions.len();
    // }
    n_steps
}


#[derive(Debug)]
struct Children {
    left: String,
    right: String
}


fn get_example_input_p1() -> Vec<String> {
    let test_input: Vec<String> = vec![
        "RL".to_string(),
        "".to_string(),
        "AAA = (BBB, CCC)".to_string(),
        "BBB = (DDD, EEE)".to_string(),
        "CCC = (ZZZ, GGG)".to_string(),
        "DDD = (DDD, DDD)".to_string(),
        "EEE = (EEE, EEE)".to_string(),
        "GGG = (GGG, GGG)".to_string(),
        "ZZZ = (ZZZ, ZZZ)".to_string()
    ];
    test_input
}


fn get_example_input_p2() -> Vec<String> {
    let test_input: Vec<String> = vec![
        "LR".to_string(),
        "".to_string(),
        "11A = (11B, XXX)".to_string(),
        "11B = (XXX, 11Z)".to_string(),
        "11Z = (11B, XXX)".to_string(),
        "22A = (22B, XXX)".to_string(),
        "22B = (22C, 22C)".to_string(),
        "22C = (22Z, 22Z)".to_string(),
        "22Z = (22B, 22B)".to_string(),
        "XXX = (XXX, XXX)".to_string()
    ];
    test_input
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_p1() {
        let example_input = get_example_input_p1();
        assert_eq!(calc_p1(&example_input), 2);
    }

    #[test]
    fn test_calc_p2() {
        let example_input = get_example_input_p2();
        assert_eq!(calc_p2(&example_input), 6);
    }
}
