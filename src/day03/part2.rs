use std::collections::HashSet;

use crate::day03::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let priostr = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let ans = input
        .chunks(3)
        .map(|lines| {
            priostr
                .find(
                    lines
                        .iter()
                        .map(|line| line.chars().collect::<HashSet<char>>())
                        .reduce(|accset, set| accset.intersection(&set).copied().collect())
                        .and_then(|set| set.into_iter().next())
                        .unwrap(),
                )
                .unwrap()
                + 1
        })
        .sum::<usize>();
    Output::U32(ans as u32)
}
