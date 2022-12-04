use super::expand_assignment;
use crate::day04::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let ans = input
        .iter()
        .map(|s| {
            let sectors = s
                .split(',')
                .map(expand_assignment)
                .collect::<Vec<Vec<u32>>>();

            sectors
        })
        .filter(|s| {
            if s[1].len() >= s[0].len() {
                s[0].iter().all(|item| s[1].contains(item))
            } else {
                s[1].iter().all(|item| s[0].contains(item))
            }
        })
        .count();

    Output::U32(ans as u32)
}
