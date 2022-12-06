pub mod input;
pub mod part1;
pub mod part2;

use crate::{Output, Part};
#[derive(Debug)]
pub struct Stack<T> {
    pub stack: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { stack: Vec::new() }
    }
    fn append(&mut self, item: &mut Vec<T>) {
        self.stack.append(item)
    }
    fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    fn push(&mut self, item: T) {
        self.stack.push(item)
    }
    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
    fn len(&self) -> usize {
        self.stack.len()
    }
    fn peek(&self) -> Option<&T> {
        self.stack.last()
    }
}

pub type Input = Vec<Vec<String>>;

pub fn run(part: Part) -> Output {
    let input = input::read();
    match part {
        Part::One => part1::solve(&input),
        Part::Two => part2::solve(&input),
    }
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
