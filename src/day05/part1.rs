use crate::day05::{Input, Output};

use super::Stack;

pub fn process_commands(pile: &mut [Stack<String>], commands: Vec<Vec<u8>>) {
    let ans = commands
        .iter()
        .map(|c| {
            let mut all_popped = Vec::new();
            for j in (0..c[0]) {
                let popped = pile[(c[1] as usize - 1)].pop().unwrap();
                all_popped.push(popped.clone());
                pile[(c[2] as usize - 1)].push(popped.clone());
            }
            all_popped
        })
        .collect::<Vec<Vec<String>>>();
}

pub fn create_stacks(boxes: &[Vec<&str>]) -> Vec<Stack<String>> {
    let num_stacks = boxes[0].len();
    let mut pile: Vec<Stack<String>> = Vec::new();
    // Instantiate the pile
    for i in (0..num_stacks) {
        pile.push(Stack::new())
    }
    pile
}

pub fn solve(input: &Input) -> Output {
    let box_input = input[0].clone();
    let boxes = box_input
        .iter()
        .map(|c| c.split('.').collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let mut pile = create_stacks(&boxes);

    for row in &boxes {
        for (i, &b) in row.iter().enumerate() {
            if b != "n" {
                pile[(i)].push(b.to_string())
            }
        }
    }

    let command_input = input[1].clone();
    let commands = command_input
        .iter()
        .rev()
        .map(|c| c.split('.').collect::<Vec<&str>>())
        .map(|c| {
            c.iter()
                .map(|&n| n.parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();
    process_commands(&mut pile, commands);

    let top = pile
        .iter()
        .map(|s| s.peek().unwrap().to_owned())
        .collect::<String>();

    Output::String(top)
}
