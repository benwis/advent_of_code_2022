pub mod input;
pub mod part1;
pub mod part2;

use crate::{Output, Part};

pub type Input = Vec<String>;

pub fn run(part: Part) -> Output {
    let input = input::read();
    match part {
        Part::One => part1::solve(&input),
        Part::Two => part2::solve(&input),
    }
}

pub fn expand_assignment(sec: &str) -> Vec<u32> {
    let nums = sec
        .split('-')
        .map(|l| l.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    // Silly non inclusive upper bound
    (nums[0]..(nums[1] + 1)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_answer_one() {
        let result = run(Part::One);
        println!("{result}");
    }

    #[test]
    fn check_answer_two() {
        let result = run(Part::Two);
        println!("{result}");
    }
}
