use utils;
// use regex::Regex;
// use std::cmp;


fn main() {
    let raw_input = utils::lines_from_file("day09");
    let p1: i64 = calc_p1(&raw_input);
    let p2: i64 = calc_p2(&raw_input);
    println!("p1: {}", p1);
    println!("p2: {}", p2);
}


fn calc_p1(raw_input: &Vec<String>) -> i64 {
    let mut p1: i64 = 0;
    for row in raw_input {
        let pyramid_rows: Vec<Vec<i64>> = Vec::new();
        let mut nums: Vec<i64> = row.split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        let mut pyramid_rows: Vec<Vec<i64>> = Vec::new();
        pyramid_rows.push(nums.clone());
        while nums.iter().any(|&x| x != 0) {
            let mut new_nums: Vec<i64> = Vec::new();
            for i in 0..nums.len() - 1 {
                new_nums.push(nums[i + 1] - nums[i]);
            }
            nums = new_nums;
            pyramid_rows.push(nums.clone());
        }
        let mut end_element: i64 = 0;
        while pyramid_rows.len() > 0 {
            end_element = end_element + pyramid_rows.pop().unwrap().pop().unwrap();
        }
        p1 += end_element;
    }
    p1
}

fn calc_p2(raw_input: &Vec<String>) -> i64 {
    let mut p2: i64 = 0;
    for row in raw_input {
        let pyramid_rows: Vec<Vec<i64>> = Vec::new();
        let mut nums: Vec<i64> = row.split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        let mut pyramid_rows: Vec<Vec<i64>> = Vec::new();
        pyramid_rows.push(nums.clone());
        while nums.iter().any(|&x| x != 0) {
            let mut new_nums: Vec<i64> = Vec::new();
            for i in 0..nums.len() - 1 {
                new_nums.push(nums[i + 1] - nums[i]);
            }
            nums = new_nums;
            pyramid_rows.push(nums.clone());
        }
        let mut start_element: i64 = 0;
        while pyramid_rows.len() > 0 {
            start_element = pyramid_rows.pop().unwrap().into_iter().nth(0).unwrap() - start_element;
        }
        p2 += start_element;
    }
    p2
}


fn get_example_input() -> Vec<String> {
    let test_input: Vec<String> = vec![
        "0 3 6 9 12 15".to_string(),
        "1 3 6 10 15 21".to_string(),
        "10 13 16 21 30 45".to_string()
    ];
    test_input
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_p1() {
        assert_eq!(calc_p1(&vec!["0 3 6 9 12 15".to_string()]), 18);
        assert_eq!(calc_p1(&vec!["1 3 6 10 15 21".to_string()]), 28);
        assert_eq!(calc_p1(&vec!["10 13 16 21 30 45".to_string()]), 68);
        assert_eq!(calc_p1(&get_example_input()), 114);
    }

    #[test]
    fn test_calc_p2() {
        assert_eq!(calc_p2(&vec!["0 3 6 9 12 15".to_string()]), -3);
        assert_eq!(calc_p2(&vec!["1 3 6 10 15 21".to_string()]), 0);
        assert_eq!(calc_p2(&vec!["10 13 16 21 30 45".to_string()]), 5);
        assert_eq!(calc_p2(&get_example_input()), 2);
    }
}
