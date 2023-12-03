const MAX_RED: usize = 12;
const MAX_GREEN: usize = 13;
const MAX_BLUE: usize = 14;

fn main() {
    let input = read_file();
    let part_one = sum_valid_game_ids(input.clone());
    let part_two = minimum_number_of_cubes(input);

    println!("Result part one: {part_one}");
    println!("Result part two: {part_two}");
}

fn read_file() -> String {
    std::fs::read_to_string("input.txt").unwrap()
}

fn sum_valid_game_ids(input: String) -> usize {
    input
        .lines()
        .filter_map(|line| {
            let (game_id, sets) = line.split_once(": ").unwrap();
            let game_id = game_id
                .chars()
                .filter(|c| c.is_numeric())
                .collect::<String>()
                .parse::<usize>()
                .unwrap();

            let valid_sets = sets
                .split("; ")
                .map(|set| {
                    set.split(", ")
                        .map(|value| value.to_string())
                        .collect::<Vec<String>>()
                })
                .map(|set| {
                    set.iter()
                        .map(|value| {
                            let value: Vec<&str> = value.split(" ").collect();
                            let (quantity, color) = (
                                value.first().unwrap().parse::<usize>().unwrap(),
                                value.last().unwrap().to_string(),
                            );

                            match color.as_str() {
                                "red" => quantity <= MAX_RED,
                                "green" => quantity <= MAX_GREEN,
                                "blue" => quantity <= MAX_BLUE,
                                _ => panic!(),
                            }
                        })
                        .reduce(|acc, value| !(!acc || !value))
                        .unwrap()
                })
                .reduce(|acc, value| !(!acc || !value))
                .unwrap();

            if valid_sets {
                Some(game_id)
            } else {
                None
            }
        })
        .sum()
}

fn minimum_number_of_cubes(input: String) -> usize {
    input
        .lines()
        .map(|line| line.split(": ").last().unwrap())
        .map(|line| {
            let mut red: usize = 0;
            let mut green: usize = 0;
            let mut blue: usize = 0;

            line
                .split("; ")
                .map(|set| {
                    set.split(", ").for_each(|values| {
                        let value: Vec<&str> = values.split(" ").collect();
                        let (quantity, color) = (
                            value.first().unwrap().parse::<usize>().unwrap(),
                            value.last().unwrap().to_string(),
                        );

                        match color.as_str() {
                            "red" => red = red.max(quantity),
                            "green" => green = green.max(quantity),
                            "blue" => blue = blue.max(quantity),
                            _ => panic!(),
                        }
                    });


                    (red * green * blue) as usize
                })
                .for_each(drop);

            (red * green * blue) as usize
        })
        .sum()
}
