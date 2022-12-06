use crate::day05::{Input, Stack};

const INPUT: &str = include_str!("../../input/05/input.txt");

pub fn read() -> Input {
    let ans = INPUT
        .split("\n\n")
        .map(|s| {
            s.split('\n')
                .map(|s| s.to_string())
                .filter(|s| !s.contains("   2"))
                .rev()
                .map(|l| {
                    let num_cols;
                    if l.contains('[') {
                        // [N] [C]     [T]
                        l.replace(' ', "x")
                            // [N]x[C]xxxxx[T]
                            .replace("xxxxxxxxx", ".n.n.")
                            .replace("xxxxxxxx", ".n.n")
                            .replace("xxxxxx", ".n.n.")
                            .replace("xxxxx", ".n.")
                            // [N]x[C]x[T]
                            .replace("xxx", "n")
                            // nx[D]nx
                            .replace('x', ".")
                            // n [D]n
                            .replace("]n", ".n")
                            // n [D n
                            .replace(['[', ']'], "")
                            // n D n
                            .trim()
                            .trim_end_matches('.')
                            .to_string()
                    } else if l.contains("   ") {
                        num_cols = l
                            .trim()
                            .split("   ")
                            .map(|s| s.parse::<u8>().unwrap())
                            .max()
                            .unwrap();
                        num_cols.to_string()
                    } else {
                        //do something for instructions
                        l.replace("move ", "")
                            .replace(" from ", ".")
                            .replace(" to ", ".")
                    }
                })
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();
    ans
}
