use utils;
use regex::Regex;
use std::cmp;


fn main() {
    let raw_input = utils::lines_from_file("day03");
    let p1: usize = calc_p1(&raw_input);
    let p2: u64 = calc_p2(&raw_input);
    println!("p1: {}", p1);
    println!("p2: {}", p2);
}


fn calc_p1(raw_input: &Vec<String>) -> usize {
    let mut p1: usize = 0;
    let re = Regex::new(r"(\d+)").unwrap();
    let n_rows = raw_input.len();
    let n_cols = raw_input[0].len();
    for (i_row, row) in raw_input.clone().into_iter().enumerate() {
        let matches: regex::Matches = re.find_iter(&row);

        for m in matches {
            let i_above = find_i_minus_one(i_row);
            let i_below = cmp::min(i_row + 1, n_rows - 1);
            let j_left = find_i_minus_one(m.start());
            let j_right = cmp::min(m.end(), n_cols - 1);
            let mut is_part_number: bool = false;
            for i in i_above..i_below+1 {
                is_part_number = is_part_number || contains_non_digits(&raw_input[i].get(j_left..j_right+1).unwrap());
            }
            if is_part_number {
                p1 += m.as_str().parse::<usize>().unwrap();
            }
        }
    }
    p1
}


fn find_i_minus_one(i: usize) -> usize {
    if i > 0 {
        i - 1
    } else {
        0
    }
}


fn contains_non_digits(s: &str) -> bool {
    let re = Regex::new(r"[^\d.]").unwrap();
    re.is_match(s)
}


fn calc_p2(raw_input: &Vec<String>) -> u64 {
    let mut p2: u64 = 0;
    let n_rows = raw_input.len();
    let n_cols = raw_input[0].len();
    let re = Regex::new(r"(\d+)").unwrap();
    for i_row in 0..n_rows {
        for i_col in 0..n_cols {
            if raw_input[i_row].chars().nth(i_col).unwrap() != '*' {
                continue;
            }
            let i_above = find_i_minus_one(i_row);
            let i_below = cmp::min(i_row + 1, n_rows - 1);
            let j_left = find_i_minus_one(i_col);
            let j_right = cmp::min(i_col + 1, n_cols - 1);
            let mut n_neighbours: usize = 0;
            let mut gear_power: u64 = 1;
            for i in i_above..i_below+1 {
                let matches: regex::Matches = re.find_iter(&raw_input[i]);
                for m in matches {
                    if m.start() <= j_right && m.end() - 1 >= j_left {
                        let power = m.as_str().parse::<u64>().unwrap();
                        n_neighbours += 1;
                        gear_power *= power;
                    }
                }
            }
            if n_neighbours == 2 {
                p2 += gear_power;
            }
        }
    }
    p2
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_p2() {
        let raw_input = vec![
            String::from("..."),
            String::from("..."),
            String::from("..."),
        ];
        assert_eq!(calc_p2(&raw_input), 0);

        let raw_input = vec![
            String::from("467..114.."),
            String::from("...*......"),
            String::from("..35..633."),
            String::from("......#..."),
            String::from("617*......"),
            String::from(".....+.58."),
            String::from("..592....."),
            String::from("......755."),
            String::from("...$.*...."),
            String::from(".664.598..")
        ];
        assert_eq!(calc_p2(&raw_input), 467835);
    }
}
