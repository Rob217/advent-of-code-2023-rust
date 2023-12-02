use utils;
use regex::Regex;
use std::collections::HashMap;
use std::cmp;


fn main() {
    let raw_input = utils::lines_from_file("day02");
    let game_number_re = Regex::new(r"Game (\d+)").unwrap();
    let mut p1: usize = 0;
    let mut p2: usize = 0;
    for game in raw_input {
        let game_number: &usize = &game_number_re.find(&game).unwrap().as_str()[5..].parse().unwrap();
        if is_valid_game_p1(&game) {
            p1 += game_number;
        }
        p2 += game_power_p2(&game);
    }
    println!("p1: {}", p1);
    println!("p2: {}", p2);
}


fn is_valid_game_p1(game: &str) -> bool {
    let mut n_cubes = HashMap::new();
    n_cubes.insert(String::from("red"), 12);
    n_cubes.insert(String::from("green"), 13);
    n_cubes.insert(String::from("blue"), 14);

    let mut is_valid_game: bool = true;
    for draw in game.split(":").nth(1).unwrap().split(";") {
        let mut is_valid_draw: bool = true;

        for color in vec!["red", "blue", "green"] {
            let color_re = Regex::new(&format!(r"(\d+) {}", color)).unwrap();
            if color_re.find(&draw).is_none() {
                continue;
            }
            let color_number: &usize = &color_re.find(&draw).unwrap().as_str().split_whitespace().nth(0).unwrap().parse().unwrap();
            if color_number > n_cubes.get(color).unwrap() {
                is_valid_draw = false;
            }
        }
        is_valid_game = is_valid_game && is_valid_draw;
    }
    is_valid_game
}


fn game_power_p2(game: &str) -> usize {
    let mut n_cubes = HashMap::new();
    n_cubes.insert(String::from("red"), 0);
    n_cubes.insert(String::from("green"), 0);
    n_cubes.insert(String::from("blue"), 0);

    let red_re = Regex::new(r"(\d+) red").unwrap();
    let green_re = Regex::new(r"(\d+) green").unwrap();
    let blue_re = Regex::new(r"(\d+) blue").unwrap();

    for draw in game.split(":").nth(1).unwrap().split(";") {
        for color in vec!["red", "blue", "green"] {
            let color_re = Regex::new(&format!(r"(\d+) {}", color)).unwrap();
            if color_re.find(&draw).is_none() {
                continue;
            }
            let color_number: &usize = &color_re.find(&draw).unwrap().as_str().split_whitespace().nth(0).unwrap().parse().unwrap();
            n_cubes.insert(String::from(color), *cmp::max(n_cubes.get(color).unwrap(), color_number));
        }
    }
    n_cubes.get("red").unwrap() * n_cubes.get("green").unwrap() * n_cubes.get("blue").unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_game_p1() {
        assert_eq!(is_valid_game_p1("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"), true);
        assert_eq!(is_valid_game_p1("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"), true);
        assert_eq!(is_valid_game_p1("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"), false);
        assert_eq!(is_valid_game_p1("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"), false);
        assert_eq!(is_valid_game_p1("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"), true);
    }

    #[test]
    fn test_game_power_p2() {
        assert_eq!(game_power_p2("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"), 48);
        assert_eq!(game_power_p2("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"), 12);
        assert_eq!(game_power_p2("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"), 1560);
        assert_eq!(game_power_p2("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"), 630);
        assert_eq!(game_power_p2("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"), 36);
    }
}
