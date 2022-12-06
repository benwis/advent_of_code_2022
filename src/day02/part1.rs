use crate::day02::{Input, Output};

pub fn match_line(c: &str) -> u32 {
    match c {
        "A X" => 3 + 1,
        "A Y" => 6 + 2,
        "A Z" => 3,
        "B X" => 1,
        "B Y" => 3 + 2,
        "B Z" => 6 + 3,
        "C X" => 6 + 1,
        "C Y" => 2,
        "C Z" => 3 + 3,
        _ => panic!("Unknown input"),
    }
}

pub fn solve(input: &Input) -> Output {
    let sum = input.iter().map(|l| match_line(l)).sum::<u32>();

    Output::U32(sum)
}
