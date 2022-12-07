use crate::day07::Input;

use super::{Command, DirectoryTree, File, Line};

const INPUT: &str = include_str!("../../input/07/input.txt");

pub fn parse_line(l: &str) -> Line {
    if let Some(com) = l.strip_prefix("$ ") {
        if let Some(dir) = com.trim().strip_prefix("cd ") {
            Line::Command(Command::CD(dir.to_string()))
        } else if let Some(list) = com.strip_prefix("ls") {
            Line::Command(Command::LS)
        } else {
            panic!("Unrecognized command!")
        }
    } else if let Some(s) = l.strip_prefix("dir ") {
        // This is a Directory in the current directory
        Line::Directory(s.to_string())
    } else {
        let (size, _) = l.split_once(' ').unwrap();
        Line::File(File {
            size: size.parse().unwrap(),
        })
    }
}
/// Create a new nested working tree with subdirs populated correctly for the current path
fn nest_working_tree<'a>(tree: &'a mut DirectoryTree, path: &'_ [String]) -> &'a mut DirectoryTree {
    let mut working_tree = tree;
    for part in path.iter().skip(1) {
        // Skip the root
        working_tree = working_tree.subdirs.get_mut(part).unwrap();
    }
    working_tree
}
// Create the Directory tree from an iterator of Line items
pub fn make_dir_tree(line_iter: impl Iterator<Item = Line>) -> DirectoryTree {
    let mut dir_tree = DirectoryTree::default();
    let mut path = Vec::new();
    let mut working_tree = &mut dir_tree;
    for line in line_iter {
        match line {
            Line::Command(Command::CD(dir)) => {
                if &dir == ".." {
                    path.pop();
                } else {
                    path.push(dir)
                }
                working_tree = nest_working_tree(&mut dir_tree, &path);
            }
            Line::Command(Command::LS) => (), //Skipping LS as Dir and File handle the rest,
            Line::Directory(dir) => {
                working_tree.subdirs.insert(dir, Default::default());
            }
            Line::File(file) => {
                working_tree.files.push(file);
            }
        }
    }
    dir_tree
}
pub fn read() -> Input {
    let lines = INPUT.lines().map(parse_line);
    make_dir_tree(lines)
}
