const MAX_RED: usize = 12;
const MAX_GREEN: usize = 13;
const MAX_BLUE: usize = 14;

fn main() {
    let input = read_file();
    let part_one = sum_valid_game_ids(input);

    println!("Result part one: {part_one}");
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
                    set
                        .iter()
                        .map(|value| {
                            let value: Vec<&str> = value.split(" ").collect();
                            let (quantity, color) = (value.first().unwrap().parse::<usize>().unwrap(), value.last().unwrap().to_string());

                            match color.as_str() {
                                "red" => quantity <= MAX_RED,
                                "green" => quantity <= MAX_GREEN,
                                "blue" => quantity <= MAX_BLUE,
                                _ => panic!(),
                            }
                        })
                        .reduce(|acc, value| {
                            !(!acc || !value)
                        })
                        .unwrap()
                })
                .reduce(|acc, value| {
                    !(!acc || !value)
                })
                .unwrap();

            if valid_sets {
                Some(game_id)
            } else {
                None
            }
        })
        .sum::<usize>()
}
