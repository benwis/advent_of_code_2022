use crate::day02::Input;

const INPUT: &str = include_str!("../../input/02/input.txt");

pub fn read() -> Input {
    INPUT
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}
