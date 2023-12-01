use std::{
    fs::File,
    io::{prelude::*, BufReader},
    env,
    path::PathBuf
};

pub fn lines_from_file(day: &str) -> Vec<String> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let inputs_dir = manifest_dir.join("../../inputs");
    let filename = format!("{}/{}.in", inputs_dir.display(), day);
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn lines_from_file_as_usize(day: &str) -> Vec<usize> {
    let lines = lines_from_file(day);
    lines.iter().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>()
}

pub fn get_char_at_index(input_string: &str, index: usize) -> char {
    input_string.chars().nth(index).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_char_at_index() {
        assert_eq!(get_char_at_index("abcde", 0), 'a');
        assert_eq!(get_char_at_index("abcde", 1), 'b');
        assert_eq!(get_char_at_index("abcde", 2), 'c');
        assert_eq!(get_char_at_index("abcde", 3), 'd');
        assert_eq!(get_char_at_index("abcde", 4), 'e');
    }
}
