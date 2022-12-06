use crate::day06::{Input, Output};

use std::collections::HashSet;

pub fn check_window(w: &[char]) -> bool {
    let check = w.iter().copied().collect::<HashSet<char>>();
    check.len() == 14
}

pub fn solve(input: &Input) -> Output {
    let args = input.windows(14).map(check_window).collect::<Vec<bool>>();
    let mut ans = 0;
    for (i, val) in args.iter().enumerate() {
        if *val {
            ans = i + 14;
            break;
        }
    }
    Output::U32(ans as u32)
}
