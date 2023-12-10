fn main() {
    let input = read_file("input.txt");
    let part_one_result = part_one(&input);
    let part_two_result = part_two(&input);

    println!("Part one: {part_one_result}");
    println!("Part two: {part_two_result}");
}

type Id = u32;
type Ids = Vec<Id>;

fn read_file(path: &str) -> String {
    std::fs::read_to_string(path).expect("Error reading file")
}

fn part_one(input: &str) -> u32 {
    let _seeds = get_seeds(input);
    0
}

fn part_two(_input: &str) -> u32 {
    0
}

fn get_seeds(input: &str) -> Ids {
    input
        .lines()
        .next()
        .map(|line| line.replace("seeds: ", ""))
        .map(|line| {
            line.split(" ")
                .map(|number| number.parse().unwrap())
                .collect::<Ids>()
        })
        .unwrap()
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

    #[test]
    fn get_seeds_test() {
        let input = get_example_file();
        let result = get_seeds(&input);

        assert_eq!(vec![79, 14, 55, 13], result)
    }
}
