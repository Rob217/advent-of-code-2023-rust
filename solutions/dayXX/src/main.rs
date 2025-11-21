use utils;
use regex::Regex;
use std::cmp;


fn main() {
    let raw_input = utils::lines_from_file("dayXX");
    let p1: usize = calc_p1(&raw_input);
    let p2: usize = calc_p2(&raw_input);
    println!("p1: {}", p1);
    println!("p2: {}", p2);
}


fn calc_p1(raw_input: &Vec<String>) -> usize {
    let mut p1: usize = 0;
    p1
}

fn calc_p2(raw_input: &Vec<String>) -> usize {
    let mut p2: usize = 0;
    p2
}


fn get_example_input() -> Vec<String> {
    let test_input: Vec<String> = vec![
        "foo bar".to_string(),
        "bar foo".to_string()
    ];
    test_input
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_p1() {
        let example_input = get_example_input();
        assert_eq!(calc_p1(&example_input), 35);
    }

    #[test]
    fn test_calc_p2() {
        let example_input = get_example_input();
        assert_eq!(calc_p2(&example_input), 0);
    }
}
