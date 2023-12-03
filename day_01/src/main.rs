use std::fs;
use std::path::Path;

fn main() {
    let input = read_file();
    let part_one = sum_lines(input.clone());
    let part_two = sum_lines(parse_digits(input));

    println!("Result part one: {part_one}");
    println!("Result part two: {part_two}");
}

fn read_file() -> String {
    let path = Path::new("input.txt");
    fs::read_to_string(path).unwrap()
}

fn sum_lines(input: String) -> usize {
    input
        .lines()
        .map(|line| {
            let is_numeric = |c: &char| c.is_numeric();
            let first = line.chars().find(is_numeric).unwrap();
            let last = line.chars().rev().find(is_numeric).unwrap();
            format!("{first}{last}").parse::<usize>().unwrap()
        })
        .sum()
}

fn parse_digits(input: String) -> String {
    input
        .lines()
        .map(|line| {
            line.replace("one", "o1e")
                .replace("two", "t2o")
                .replace("three", "t3e")
                .replace("four", "f4r")
                .replace("five", "f5e")
                .replace("six", "s6x")
                .replace("seven", "s7n")
                .replace("eight", "e8t")
                .replace("nine", "n9e")
        })
        .collect::<Vec<String>>()
        .join("\n")
}
