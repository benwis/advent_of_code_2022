use crate::day03::{Input, Output};

pub fn solve(input: &Input) -> Output {
    let priority: u32 = input
        .iter()
        .map(|l| {
            let mid: usize = l.len() / 2;
            (l[..mid].to_string(), l[mid..].to_string())
        })
        .filter_map(|(s1, s2)| {
            for c1 in s1.chars() {
                for c2 in s2.chars() {
                    if c1 == c2 {
                        return Some(c1);
                    }
                }
            }
            None
        })
        // map each char to points
        .filter_map(|c| {
            if c.is_lowercase() {
                Some(1 + (c as u32) - ('a' as u32))
            } else if c.is_uppercase() {
                Some(27 + (c as u32) - ('A' as u32))
            } else {
                None
            }
        })
        .sum();
    Output::U32(priority)
}
