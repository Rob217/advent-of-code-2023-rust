use utils;
use std::collections::HashMap;
use std::collections::HashSet;


fn main() {
    let raw_input = utils::lines_from_file("day10");
    let p1: usize = calc_p1(&raw_input);
    let p2: usize = calc_p2(&raw_input);
    println!("p1: {}", p1);
    println!("p2: {}", p2);
}


fn calc_p1(raw_input: &Vec<String>) -> usize {
    let mut p1: usize = 0;

    // convert input into a hash map of nodes
    let mut grid = Grid::from(raw_input);
    println!("{:?}", grid);

    // find the starting node 'S'
    let mut start_node = grid
        .nodes
        .iter_mut()
        .find(|(_, node)| node.name == 'S')
        .unwrap();
    // let mut start_node =
    // start_node.1.visited = true;
    println!("start_node: {:?}", start_node);

    // store visited locations in a separate set to avoid having to modify the nodes
    let mut visited = HashSet::new();
    visited.insert(start_node.0);

    // let mut to_visit = vec![start_node.1.children.0, start_node.1.children.1];
    // println!("to_visit: {:?}", to_visit);

    // this assumes that S is not on edge of grid
    let mut nodes_to_visit: Vec<(usize, usize)> = Vec::new();
    let (x_start, y_start) = start_node.0;
    // match grid.nodes.get(&(x_start + 1, y_start)).unwrap().name {
    //     '-' => nodes_to_visit.push((x_start + 1, y_start)),
    //     'L' => nodes_to_visit.push((x_start + 1, y_start)),
    //     'J' => nodes_to_visit.push((x_start + 1, y_start)),
    //     'F' => nodes_to_visit.push((x_start + 1, y_start)),
    //     _ => ()
    // }
    // if grid.nodes.get(&(x_start, y_start - 1)).unwrap().name == '|' {
    //     nodes_to_visit.push((x_start, y_start - 1));
    // }
    println!("{:?}, {:?}", x_start, y_start);
    grid.nodes.get(&(*x_start, *y_start)).unwrap().name;



    // THOUGHT: store visited locations in a separate set to avoid having to modify the nodes

    // loop around the nodes in both directions, counting the number of steps taken until pointers reach each other
    // let mut foo = grid
    //     .nodes
    //     .get_mut(&(1, 1))
    //     .unwrap();
    // foo.visited = true;

    // println!("{:?}", start_node);

    // let mut left_node: &mut Node = grid.nodes.get_mut(&start_node.1.children.0).unwrap();
    // let mut left_node: &mut Node = grid.nodes.get_mut(&(0, 0)).unwrap();
    // println!({"{:?}", left_node});

    // let mut left_node: &mut Node = grid.nodes.get(&start_node.1.children.0).unwrap();
    // let mut right_node: &mut Node = grid.nodes.get(&start_node.1.children.1).unwrap();
    // left_node.visited = true;
    // right_node.visited = true;
    // while true {
    //     for child in vec![left_node.children.0, left_node.children.1] {

    //     }
    // }

    // return the number of steps taken, halved

    p1
}

fn calc_p2(raw_input: &Vec<String>) -> usize {
    let mut p2: usize = 0;
    p2
}


///////////////////
// supporting methods/objects
///////////////////

#[derive(Debug)]
struct Node {
    name: char,
    pos: (usize, usize),
    visited: bool,
    children: ((usize, usize), (usize, usize))
}

#[derive(Debug)]
struct Grid {
    nodes: HashMap<(usize, usize), Node>
}

impl From<&Vec<String>> for Grid {
    fn from(input: &Vec<String>) -> Self {
        let mut grid = HashMap::new();
        for (y, row) in input.iter().enumerate() {
            for (x, node) in row.chars().enumerate() {
                let xm1: usize = match x {
                    0 => 0,
                    _ => x - 1
                };
                let ym1: usize = match y {
                    0 => 0,
                    _ => y - 1
                };
                let children: ((usize, usize), (usize, usize)) = match node {
                    '|' => ((x, ym1), (x, y + 1)),
                    '-' => ((xm1, y), (x + 1, y)),
                    'L' => ((x, ym1), (x + 1, y)),
                    'J' => ((x, ym1), (xm1, y)),
                    '7' => ((x, y + 1), (xm1, y)),
                    'F' => ((x, y + 1), (x + 1, y)),
                    '.' => ((x, y), (x, y)),
                    'S' => ((x, y), (x, y)),
                    _ => panic!("Invalid node type: {}", node)
                };
                // let node = Node {
                //     name: node,
                //     pos: (x, y),
                //     visited: false,
                //     children: children
                // };
                grid.insert(
                    (x, y),
                    Node {
                        name: node,
                        pos: (x, y),
                        visited: false,
                        children: children
                    }
                );
            }
        }
        Grid { nodes: grid }
    }
}


///////////////////
// testing
///////////////////

fn get_example_input() -> Vec<String> {
    let test_input: Vec<String> = vec![
        "-L|F7".to_string(),
        "7S-7|".to_string(),
        "L|7||".to_string(),
        "-L-J|".to_string(),
        "L|-JF".to_string()
    ];
    test_input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_p1() {
        let example_input = get_example_input();
        assert_eq!(calc_p1(&example_input), 8);
    }

    #[test]
    fn test_calc_p2() {
        let example_input = get_example_input();
        assert_eq!(calc_p2(&example_input), 0);
    }
}
