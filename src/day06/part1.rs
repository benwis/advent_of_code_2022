use std::collections::HashSet;

use crate::day06::{Input, Output};

pub fn check_window(w: &[char]) -> bool {
    let check = w.iter().copied().collect::<HashSet<char>>();
    check.len() == 4
}

pub fn solve(input: &Input) -> Output {
    let args = input.windows(4).map(check_window).collect::<Vec<bool>>();
    let mut ans = 0;
    for (i, val) in args.iter().enumerate() {
        if *val {
            ans = i + 4;
            break;
        }
    }
    Output::U32(ans as u32)
}
