use crate::day03::Input;

const INPUT: &str = include_str!("../../input/03/input.txt");

pub fn read() -> Input {
    INPUT
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}
