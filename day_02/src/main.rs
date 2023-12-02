fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let compare_set = GameSet::default();
    let sum: usize = input
        .lines()
        .map(|line| Game::new(line, &compare_set))
        .filter(|game| game.is_valid())
        .map(|game| game.id)
        .sum();

    println!("Result is {sum}");
}

#[derive(Debug)]
struct GameSet {
    red: usize,
    green: usize,
    blue: usize,
}

impl GameSet {
    fn create_from_input(value: &str) -> Vec<Self> {
        let mut sets: Vec<GameSet> = vec![];
        let set_str = value.split(":").last().unwrap().trim();
        set_str
            .split(";")
            .map(|game_set_string| {
                let mut game_set = GameSet::default();
                game_set_string
                    .split(",")
                    .map(|set| {
                        let splitted = set
                            .trim()
                            .split(" ")
                            .map(|s| s.to_string())
                            .collect::<Vec<String>>();
                        let mut iter = splitted.iter().rev();
                        let color = iter.next().unwrap().to_string();
                        let quantity = iter.next().unwrap().parse::<usize>().unwrap();
                        match color.as_str() {
                            "red" => game_set.set_red(quantity),
                            "green" => game_set.set_green(quantity),
                            "blue" => game_set.set_blue(quantity),
                            _ => println!("No match!"),
                        };
                    })
                    .for_each(drop);
                sets.push(game_set);
            })
            .for_each(drop);
        sets
    }

    fn set_red(&mut self, quantity: usize) {
        self.red = quantity;
    }

    fn set_green(&mut self, quantity: usize) {
        self.green = quantity;
    }

    fn set_blue(&mut self, quantity: usize) {
        self.blue = quantity;
    }

    fn is_valid(&self, compare_set: &GameSet) -> bool {
        &self.red <= &compare_set.red
            && &self.green <= &compare_set.green
            && &self.blue <= &compare_set.blue
    }
}

impl Default for GameSet {
    fn default() -> Self {
        Self {
            red: 12,
            green: 13,
            blue: 14,
        }
    }
}

struct Game<'a> {
    id: usize,
    sets: Vec<GameSet>,
    compare_set: &'a GameSet,
}

impl<'a> Game<'a> {
    fn new(line: &str, compare_set: &'a GameSet) -> Self {
        let id = line
            .split(": ")
            .find(|s| s.contains("Game"))
            .unwrap()
            .trim()
            .split(" ")
            .find_map(|s| s.parse::<usize>().ok())
            .unwrap();
        let sets = GameSet::create_from_input(line);

        Game {
            id,
            sets,
            compare_set,
        }
    }

    fn is_valid(&self) -> bool {
        for set in &self.sets {
            if !set.is_valid(self.compare_set) {
                return false;
            }
        }
        true
    }
}
