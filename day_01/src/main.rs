use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("input.txt");
    let input = fs::read_to_string(path).unwrap();
    let sum = input.lines().map(|line| {
        let first = line.chars().find(|c| c.is_numeric()).unwrap();
        let last = line.chars().rev().find(|c| c.is_numeric()).unwrap();
        let number = format!("{first}{last}");
        number.parse::<u32>().unwrap()
    }).sum::<u32>();
    println!("Result is {sum}");
}
