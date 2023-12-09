use std::collections::HashMap;

fn main() {
    let input = read_file("input.txt");
    let part_one_result = part_one(&input);
    let part_two_result = part_two(&input);

    println!("Result of part one is {part_one_result}");
    println!("Result of part two is {part_two_result}");
}

type InputMap = HashMap<String, char>;
type Number = u32;
type Position = (i32, i32);
type PositionNumber = (Vec<Position>, Number);
type GearPositionNumber = (Vec<Position>, Number, Position);

fn read_file(path: &str) -> String {
    std::fs::read_to_string(path).unwrap()
}

fn part_one(input: &str) -> u32 {
    let map = build_map(input);

    input
        .lines()
        .enumerate()
        .filter_map(filter_map_position_numbers)
        .flatten()
        .filter(|position_number| filter_part_numbers(position_number, &map))
        .map(|position_number| position_number.1)
        .sum()
}

fn part_two(input: &str) -> u32 {
    let map = build_map(input);

    let gear_numbers: Vec<GearPositionNumber> = input
        .lines()
        .enumerate()
        .filter_map(filter_map_position_numbers)
        .flatten()
        .filter_map(|position_number| filter_map_gear_numbers(&position_number, &map))
        .collect();

    let result: u32 = gear_numbers
        .iter()
        .enumerate()
        .map(|(index, gear_position)| {
            sum_corresponding_gears(index, gear_position, &mut gear_numbers.clone())
        })
        .sum();

    result / 2
}

fn build_map(input: &str) -> InputMap {
    let mut map = HashMap::new();

    input.lines().enumerate().for_each(|(y_index, line)| {
        line.chars().enumerate().for_each(|(x_index, char)| {
            map.insert(create_map_key((x_index as i32, y_index as i32)), char);
        })
    });

    map
}

fn create_map_key(position: Position) -> String {
    format!("{}/{}", position.0, position.1)
}

fn filter_map_position_numbers((y_index, line): (usize, &str)) -> Option<Vec<PositionNumber>> {
    let mut position_numbers: Vec<PositionNumber> = vec![];
    let mut positions: Vec<Position> = vec![];
    let mut str_number = String::new();

    line.chars().enumerate().for_each(|(x_index, char)| {
        if char.is_ascii_digit() {
            str_number.push(char);
            positions.push((x_index as i32, y_index as i32));
        } else {
            if !str_number.is_empty() && !positions.is_empty() {
                position_numbers.push((positions.clone(), str_number.parse().unwrap()));
                positions = vec![];
                str_number = String::new();
            }
        }
    });

    if !str_number.is_empty() && !positions.is_empty() {
        position_numbers.push((positions.clone(), str_number.parse().unwrap()));
    }

    if position_numbers.is_empty() {
        None
    } else {
        Some(position_numbers)
    }
}

fn get_search_positions() -> Vec<Position> {
    vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
}

fn filter_part_numbers(position_number: &PositionNumber, map: &InputMap) -> bool {
    let search_positions = get_search_positions();
    for position in &position_number.0 {
        for search_position in &search_positions {
            let key = create_map_key((
                position.0 + search_position.0,
                position.1 + search_position.1,
            ));

            if let Some(key) = map.get(&key) {
                if !key.is_ascii_digit() && key.to_owned() != '.' {
                    return true;
                }
            }
        }
    }

    false
}

fn filter_map_gear_numbers(
    position_number: &PositionNumber,
    map: &InputMap,
) -> Option<GearPositionNumber> {
    let search_positions = get_search_positions();
    for position in &position_number.0 {
        for search_position in &search_positions {
            let symbol_position = (
                position.0 + search_position.0,
                position.1 + search_position.1,
            );
            let key = create_map_key(symbol_position);

            if let Some(key) = map.get(&key) {
                if key.to_owned() == '*' {
                    return Some((
                        position_number.0.clone(),
                        position_number.1.clone(),
                        symbol_position,
                    ));
                }
            }
        }
    }

    None
}

fn compare_gear_symbol_position(gear_position: &Position, compare_position: &Position) -> bool {
    if gear_position.0 == compare_position.0 && gear_position.1 == compare_position.1 {
        return true;
    }
    false
}

fn sum_corresponding_gears(
    index: usize,
    gear_position: &GearPositionNumber,
    gear_positions: &mut Vec<GearPositionNumber>,
) -> u32 {
    gear_positions.remove(index);

    if let Some(found_gear) = gear_positions
        .iter()
        .find(|search_gear| compare_gear_symbol_position(&gear_position.2, &search_gear.2))
    {
        gear_position.1 * found_gear.1
    } else {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_example() -> String {
        read_file("example.txt")
    }

    #[test]
    fn create_map_key_test() {
        let key = create_map_key((10, 20));

        assert_eq!("10/20", key.as_str());
    }

    #[test]
    fn build_map_test() {
        let input = get_example();

        let map = build_map(&input);

        assert_eq!('4', map.get("0/0").unwrap().to_owned());
        assert_eq!('*', map.get("3/4").unwrap().to_owned());
        assert_eq!('.', map.get("9/9").unwrap().to_owned());
    }

    #[test]
    fn filter_map_position_numbers_test() {
        let input = get_example();

        let position_numbers: Vec<PositionNumber> = input
            .lines()
            .enumerate()
            .filter_map(filter_map_position_numbers)
            .flatten()
            .collect();

        assert_eq!(
            vec![
                (vec![(0, 0), (1, 0), (2, 0)], 467),
                (vec![(5, 0), (6, 0), (7, 0)], 114),
                (vec![(2, 2), (3, 2)], 35),
                (vec![(6, 2), (7, 2), (8, 2)], 633),
                (vec![(0, 4), (1, 4), (2, 4)], 617),
                (vec![(7, 5), (8, 5)], 58),
                (vec![(2, 6), (3, 6), (4, 6)], 592),
                (vec![(6, 7), (7, 7), (8, 7)], 755),
                (vec![(1, 9), (2, 9), (3, 9)], 664),
                (vec![(5, 9), (6, 9), (7, 9)], 598),
            ],
            position_numbers
        )
    }

    #[test]
    fn filter_position_numbers_test() {
        let input = get_example();
        let map = build_map(&input);

        let position_numbers: Vec<PositionNumber> = input
            .lines()
            .enumerate()
            .filter_map(filter_map_position_numbers)
            .flatten()
            .filter(|position_number| filter_part_numbers(position_number, &map))
            .collect();

        assert_eq!(
            vec![
                (vec![(0, 0), (1, 0), (2, 0)], 467),
                (vec![(2, 2), (3, 2)], 35),
                (vec![(6, 2), (7, 2), (8, 2)], 633),
                (vec![(0, 4), (1, 4), (2, 4)], 617),
                (vec![(2, 6), (3, 6), (4, 6)], 592),
                (vec![(6, 7), (7, 7), (8, 7)], 755),
                (vec![(1, 9), (2, 9), (3, 9)], 664),
                (vec![(5, 9), (6, 9), (7, 9)], 598),
            ],
            position_numbers
        )
    }

    #[test]
    fn part_one_test() {
        let input = get_example();

        let result = part_one(&input);

        assert_eq!(4361, result)
    }

    #[test]
    fn filter_gear_numbers_test() {
        let input = get_example();
        let map = build_map(&input);

        let position_numbers: Vec<GearPositionNumber> = input
            .lines()
            .enumerate()
            .filter_map(filter_map_position_numbers)
            .flatten()
            .filter_map(|position_number| filter_map_gear_numbers(&position_number, &map))
            .collect();

        assert_eq!(
            vec![
                (vec![(0, 0), (1, 0), (2, 0)], 467, (3, 1)),
                (vec![(2, 2), (3, 2)], 35, (3, 1)),
                (vec![(0, 4), (1, 4), (2, 4)], 617, (3, 4)),
                (vec![(6, 7), (7, 7), (8, 7)], 755, (5, 8)),
                (vec![(5, 9), (6, 9), (7, 9)], 598, (5, 8)),
            ],
            position_numbers
        )
    }

    #[test]
    fn compare_gear_symbol_position_test() {
        let compare_result = compare_gear_symbol_position(&(0, 0), &(0, 0));

        assert_eq!(true, compare_result)
    }

    #[test]
    fn part_two_test() {
        let input = get_example();

        let result = part_two(&input);

        assert_eq!(result, 467835)
    }
}
