fn main() {
    let input = read_file("input.txt");
    let part_one_result = part_one(&input);
    let part_two_result = part_two(&input);

    println!("Part one: {part_one_result}");
    println!("Part two: {part_two_result}");
}

fn read_file(path: &str) -> String {
    std::fs::read_to_string(path).expect("Error reading file")
}

fn part_one(_input: &str) -> u32 {
    0
}

fn part_two(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_example_file() -> String {
        read_file("example.txt")
    }

    #[test]
    fn part_one_test() {
        let input = get_example_file();
        let result = part_one(&input);

        assert_eq!(35, result)
    }

    #[test]
    fn part_two_test() {
        let input = get_example_file();
        let result = part_two(&input);

        assert!(result > 0)
    }
}
