use crate::day07::{Input, Output};

use super::DirectoryTree;

/// Find smallest dir that has the needed free space
pub fn find_smallest_dir_get_free_space(tree: &DirectoryTree) -> u32 {
    let space_needed = 30_000_000 - (70_000_000 - tree.size.unwrap());
    let mut dirs = Vec::from_iter(tree.subdirs.values());
    let mut min = 70_000_000;
    while let Some(dir) = dirs.pop() {
        let size = dir.size.unwrap();
        if size < min && size >= space_needed {
            min = size;
        }
        dirs.extend(dir.subdirs.values());
    }
    min
}

pub fn solve(input: &mut Input) -> Output {
    input.fill_sizes();
    let size = find_smallest_dir_get_free_space(input);
    Output::U32(size)
}
