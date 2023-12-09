fn main() {
    let input = read_file("input.txt");
    let part_one_result = part_one(&input);

    println!("Result part one: {part_one_result}");
}

type Numbers = Vec<u32>;
type ScratchCard = (Numbers, Numbers);

fn read_file(path: &str) -> String {
    std::fs::read_to_string(path).expect("Error reading file")
}

fn part_one(input: &str) -> u32 {
    let numbers = get_numbers_from_input(input);
    get_result(numbers)
}

fn get_numbers_from_input(input: &str) -> Vec<ScratchCard> {
    input
        .lines()
        .map(|line| {
            let (winning_numbers, numbers) =
                line.split_once(": ").unwrap().1.split_once(" | ").unwrap();

            (split_numbers(winning_numbers), split_numbers(numbers))
        })
        .collect()
}

fn split_numbers(numbers: &str) -> Numbers {
    numbers
        .split(" ")
        .filter(|number| !number.is_empty())
        .map(|number| number.parse::<u32>().unwrap())
        .collect()
}

fn get_result(cards: Vec<ScratchCard>) -> u32 {
    cards
        .iter()
        .map(|card| {
            let winning_numbers = card
                .1
                .iter()
                .filter(|number| card.0.contains(number))
                .count();

            if winning_numbers == 0 {
                return 0;
            }

            let mut result = 1;
            for _ in 1..winning_numbers {
                result *= 2
            }
            result
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_example() -> String {
        read_file("example.txt")
    }

    #[test]
    fn get_numbers_from_input_test() {
        let input = get_example();
        let numbers = get_numbers_from_input(&input);

        assert_eq!(
            vec![
                (vec![41, 48, 83, 86, 17], vec![83, 86, 6, 31, 17, 9, 48, 53],),
                (
                    vec![13, 32, 20, 16, 61],
                    vec![61, 30, 68, 82, 17, 32, 24, 19],
                ),
                (vec![1, 21, 53, 59, 44], vec![69, 82, 63, 72, 16, 21, 14, 1],),
                (
                    vec![41, 92, 73, 84, 69],
                    vec![59, 84, 76, 51, 58, 5, 54, 83],
                ),
                (
                    vec![87, 83, 26, 28, 32],
                    vec![88, 30, 70, 12, 93, 22, 82, 36],
                ),
                (
                    vec![31, 18, 13, 56, 72],
                    vec![74, 77, 10, 23, 35, 67, 36, 11],
                ),
            ],
            numbers
        )
    }

    #[test]
    fn split_numbers_test() {
        let numbers = split_numbers("41 48 83 86 17");

        assert_eq!(vec![41, 48, 83, 86, 17], numbers);
    }

    #[test]
    fn part_one_test() {
        let input = get_example();
        let result = part_one(&input);

        assert_eq!(13, result)
    }
}
