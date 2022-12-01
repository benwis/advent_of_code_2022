use crate::day01::Input;

const INPUT: &str = include_str!("../../input/01/input.txt");

pub fn read() -> Input {
    let mut split_vec = INPUT
        .split("\n\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    split_vec
        .iter()
        .map(|s| {
            s.split('\n')
                .map(|i| i.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}
