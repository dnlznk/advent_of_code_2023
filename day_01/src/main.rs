use std::fs;
use std::path::Path;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let input = read_file();
    let sum = input.lines().map(|line| {
        let first = line.chars().find(|c| c.is_numeric()).unwrap();
        let last = line.chars().rev().find(|c| c.is_numeric()).unwrap();
        let number = format!("{first}{last}");
        number.parse::<u32>().unwrap()
    }).sum::<u32>();
    println!("Result is {sum}");
}

fn part_two() {
    let digits = vec![("one", "o1e"), ("two", "t2o"), ("three", "t3e"), ("four", "f4r"), ("five", "f5e"), ("six", "s6x"), ("seven", "s7n"), ("eight", "e8t"), ("nine", "n9e")];
    let input = read_file();
    let sum: u32 = input.lines().map(|line| {
        let mut line = line.to_string();
        digits.iter().for_each(|digit| {
            line = line.replace(digit.0, &digit.1);
            ()
        });
        let first = line.chars().find(|c| c.is_numeric()).unwrap();
        let last = line.chars().rev().find(|c| c.is_numeric()).unwrap();
        let number = format!("{first}{last}");
        number.parse::<u32>().unwrap()
    }).sum();
    println!("Result is {sum}");
}

fn read_file() -> String {
    let path = Path::new("input.txt");
    fs::read_to_string(path).unwrap()
}

