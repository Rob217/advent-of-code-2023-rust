use utils;
use regex::Regex;
use std::iter::FromIterator;


fn main() {
    let raw_input = utils::lines_from_file("day01");
    println!("p1: {}", calc_p1(raw_input.clone()));
    println!("p2: {}", calc_p2(raw_input));
}


fn calc_p1(raw_input: Vec<String>) -> u32 {
    let mut p1: u32 = 0;
    for row in raw_input.into_iter() {
        let mut first_digit = ' ';
        let mut second_digit = ' ';
        for character in row.chars() {
            if character.is_digit(10) {
                if first_digit == ' ' {
                    first_digit = character;
                }
                second_digit = character;
            }
        }
        let result: u32 = 10 * first_digit.to_digit(10).unwrap() + second_digit.to_digit(10).unwrap();
        p1 += result;
    }
    p1
}


fn calc_p2(raw_input: Vec<String>) -> usize {
    let mut p2: usize = 0;
    let re_forward = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine)|[1-9]").unwrap();
    let re_backwards = Regex::new(r"(eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)|[1-9]").unwrap();

    for row in raw_input.into_iter() {
        let first_match: &str = re_forward.find(&row).unwrap().as_str();
        let first_digit: usize = convert_str_to_digit(&first_match[..]);
        let reversed_row: String = String::from_iter(row.chars().rev());
        let second_match: &str = re_backwards.find(&reversed_row).unwrap().as_str();
        let reveresed_second_match: String = second_match.chars().rev().collect::<String>();
        let second_digit: usize = convert_str_to_digit(&reveresed_second_match[..]);
        let result: usize = 10 * first_digit + second_digit;
        p2 += result;
    }
    p2
}


fn convert_str_to_digit(input_string: &str) -> usize {
    match &input_string[..] {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        _ => 0
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p2() {
        assert_eq!(calc_p2(vec!["two1nine".to_string()]), 29);
        assert_eq!(calc_p2(vec!["eightwothree".to_string()]), 83);
        assert_eq!(calc_p2(vec!["abcone2threexyz".to_string()]), 13);
        assert_eq!(calc_p2(vec!["xtwone3four".to_string()]), 24);
        assert_eq!(calc_p2(vec!["4nineeightseven2".to_string()]), 42);
        assert_eq!(calc_p2(vec!["zoneight234".to_string()]), 14);
        assert_eq!(calc_p2(vec!["7pqrstsixteen".to_string()]), 76);
        assert_eq!(calc_p2(vec!["twone".to_string()]), 21);
        let all_inputs = vec![
            "two1nine".to_string(),
            "eightwothree".to_string(),
            "abcone2threexyz".to_string(),
            "xtwone3four".to_string(),
            "4nineeightseven2".to_string(),
            "zoneight234".to_string(),
            "7pqrstsixteen".to_string()
        ];
        assert_eq!(calc_p2(all_inputs), 281);
    }
}