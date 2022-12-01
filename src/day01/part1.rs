use crate::day01::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let max = input
        .iter()
        .map(|v| v.iter().sum::<u32>())
        .fold(std::u32::MIN, |a, b| a.max(b));
    println!("MAX: {}", max);
    Output::U32(max)
}
