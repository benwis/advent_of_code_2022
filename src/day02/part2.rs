use crate::day02::{Input, Output};

// First char is their choice, second char is desired outcome
//A=Rock,B=Paper,C=Scissors X=lose Y=draw Z=Win
//Rock=1 Paper=2 Scissors=3
pub fn match_line(c: &str) -> u32 {
    match c {
        "A X" => 0 + 3,
        "A Y" => 3 + 1,
        "A Z" => 6 + 2,
        "B X" => 0 + 1,
        "B Y" => 3 + 2,
        "B Z" => 6 + 3,
        "C X" => 0 + 2,
        "C Y" => 3 + 3,
        "C Z" => 6 + 1,
        _ => panic!("Unknown input"),
    }
}

pub fn solve(input: &Input) -> Output {
    let sum = input.iter().map(|l| match_line(l)).sum::<u32>();
    Output::U32(sum)
}
