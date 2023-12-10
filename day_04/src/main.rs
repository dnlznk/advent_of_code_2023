fn main() {
    let input = read_file("input.txt");
    let part_one_result = part_one(&input);
    let part_two_result = part_two(&input);

    println!("Result part one: {part_one_result}");
    println!("Result part two: {part_two_result}");
}

type Numbers = Vec<u32>;
type Scratchcard = (Numbers, Numbers);
type ScratchcardResult = (Scratchcard, u32);
type ScratchcardInstancesMap = std::collections::HashMap<usize, u32>;

fn read_file(path: &str) -> String {
    std::fs::read_to_string(path).expect("Error reading file")
}

fn part_one(input: &str) -> u32 {
    let card_results = get_card_results_from_input(input);
    card_results
        .iter()
        .map(|card| {
            if card.1 == 0 {
                return 0;
            }

            let mut result = 1;
            for _ in 1..card.1 {
                result *= 2
            }
            result
        })
        .sum()
}

fn part_two(input: &str) -> u32 {
    let card_results = get_card_results_from_input(input);
    let map = get_card_instance_map(card_results);

    map.iter()
        .map(|card| card.1.clone())
        .reduce(|acc, card| acc + card)
        .unwrap()
}

fn get_card_results_from_input(input: &str) -> Vec<ScratchcardResult> {
    input
        .lines()
        .map(|line| {
            let (winning_numbers, numbers) =
                line.split_once(": ").unwrap().1.split_once(" | ").unwrap();

            (
                split_into_numbers(winning_numbers),
                split_into_numbers(numbers),
            )
        })
        .map(|card| {
            let winning_numbers = card
                .1
                .iter()
                .filter(|number| card.0.contains(number))
                .count();

            (card.clone(), winning_numbers as u32)
        })
        .collect()
}

fn split_into_numbers(numbers: &str) -> Numbers {
    numbers
        .split(" ")
        .filter(|number| !number.is_empty())
        .map(|number| number.parse::<u32>().unwrap())
        .collect()
}

fn get_card_instance_map(card_results: Vec<ScratchcardResult>) -> ScratchcardInstancesMap {
    let mut map = ScratchcardInstancesMap::new();

    card_results.iter().enumerate().for_each(|(index, card)| {
        let mut card_amount = 1;

        if let Some(copy) = map.get_mut(&index) {
            *copy += 1;
            card_amount = *copy;
        } else {
            map.insert(index, card_amount);
        }

        for _ in 1..card_amount + 1 {
            for copy_index in 1..card.1 + 1 {
                let index = index + copy_index as usize;

                if let Some(copy) = map.get_mut(&index) {
                    *copy += 1;
                } else {
                    map.insert(index, 1);
                }
            }
        }
    });

    map
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_example() -> String {
        read_file("example.txt")
    }

    #[test]
    fn get_scratchcards_from_input_test() {
        let input = get_example();
        let numbers = get_card_results_from_input(&input);

        assert_eq!(
            vec![
                (
                    (vec![41, 48, 83, 86, 17], vec![83, 86, 6, 31, 17, 9, 48, 53]),
                    4
                ),
                (
                    (
                        vec![13, 32, 20, 16, 61],
                        vec![61, 30, 68, 82, 17, 32, 24, 19],
                    ),
                    2
                ),
                (
                    (vec![1, 21, 53, 59, 44], vec![69, 82, 63, 72, 16, 21, 14, 1]),
                    2
                ),
                (
                    (
                        vec![41, 92, 73, 84, 69],
                        vec![59, 84, 76, 51, 58, 5, 54, 83],
                    ),
                    1
                ),
                (
                    (
                        vec![87, 83, 26, 28, 32],
                        vec![88, 30, 70, 12, 93, 22, 82, 36],
                    ),
                    0
                ),
                (
                    (
                        vec![31, 18, 13, 56, 72],
                        vec![74, 77, 10, 23, 35, 67, 36, 11],
                    ),
                    0
                ),
            ],
            numbers
        )
    }

    #[test]
    fn split_into_numbers_test() {
        let numbers = split_into_numbers("41 48 83 86 17");

        assert_eq!(vec![41, 48, 83, 86, 17], numbers);
    }

    #[test]
    fn get_card_instance_map_test() {
        let input = get_example();
        let card_results = get_card_results_from_input(&input);
        let map = get_card_instance_map(card_results);

        let mut compare_map = ScratchcardInstancesMap::new();
        compare_map.insert(0, 1);
        compare_map.insert(1, 2);
        compare_map.insert(2, 4);
        compare_map.insert(3, 8);
        compare_map.insert(4, 14);
        compare_map.insert(5, 1);

        assert_eq!(compare_map, map);
    }

    #[test]
    fn part_one_test() {
        let input = get_example();
        let result = part_one(&input);

        assert_eq!(13, result)
    }

    #[test]
    fn part_two_test() {
        let input = get_example();
        let result = part_two(&input);

        assert_eq!(30, result);
    }
}
