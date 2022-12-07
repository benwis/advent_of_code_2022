use crate::day07::{Input, Output};

use super::DirectoryTree;

pub fn find_small_dirs_size(tree: &DirectoryTree) -> u32 {
    let subdir_total: u32 = tree.subdirs.values().map(find_small_dirs_size).sum();
    subdir_total + tree.size.filter(|&d| d <= 100_000).unwrap_or_default()
}

pub fn solve(input: &mut Input) -> Output {
    input.fill_sizes();
    let ans = find_small_dirs_size(input);
    println!("ANS: {ans}");
    Output::U32(ans)
}
