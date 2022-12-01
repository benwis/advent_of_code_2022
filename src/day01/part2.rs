use crate::day01::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let mut sums_vec = input
        .iter()
        .map(|v| v.iter().sum::<u32>())
        .collect::<Vec<u32>>();
    sums_vec.sort();
    let sum_three = sums_vec.iter().rev().take(3).sum();
    Output::U32(sum_three)
}
