pub mod input;
pub mod part1;
pub mod part2;

use crate::{Output, Part};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct File {
    pub size: u32,
}

#[derive(Debug, Clone)]
pub enum Command {
    LS,
    CD(String),
}

#[derive(Debug, Clone)]
pub enum Line {
    Command(Command),
    File(File),
    Directory(String),
}

// Create a recursive Tree struct, to details about the Directory at each level
#[derive(Debug, Default)]
pub struct DirectoryTree {
    subdirs: HashMap<String, DirectoryTree>,
    files: Vec<File>,
    size: Option<u32>,
}

impl DirectoryTree {
    // Recursive function to populate the file sizes in the tree, starting from the bottom up
    fn fill_sizes(&mut self) {
        for dir in self.subdirs.values_mut() {
            dir.fill_sizes()
        }
        // Get the size of all files in the current dir + the stored sizes of all the files in the subdir
        self.size = Some(
            self.files.iter().map(|file| file.size).sum::<u32>()
                + self
                    .subdirs
                    .values()
                    .map(|subdir| subdir.size.unwrap())
                    .sum::<u32>(),
        )
    }
}

pub type Input = DirectoryTree;

pub fn run(part: Part) -> Output {
    let mut input = input::read();
    match part {
        Part::One => part1::solve(&mut input),
        Part::Two => part2::solve(&mut input),
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
