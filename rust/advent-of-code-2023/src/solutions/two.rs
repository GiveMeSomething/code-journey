use std::{
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
    vec,
};

pub struct GameMetadata {
    red: usize,
    green: usize,
    blue: usize,
}

impl Debug for GameMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Red {}, Green {}, Blue {}\n",
            self.red, self.green, self.blue,
        )
    }
}

pub fn read_games_from_file() -> Vec<Vec<GameMetadata>> {
    let file = File::open("src/inputs/two.txt").expect("Should be an input file for day 2");

    let reader = BufReader::new(file);
    let mut result: Vec<Vec<GameMetadata>> = vec![];

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let header_split: Vec<&str> = line.split(":").collect();

                // // Extract game id
                // let game_id = &header_split[0]["Game".len()..];
                // let game_id_int: usize = game_id
                //     .parse()
                //     .expect("Expect game id to be an unsigned int");

                let mut current_game: Vec<GameMetadata> = vec![];
                let game_sets: Vec<&str> = header_split[1].split(";").collect();

                for game_set in game_sets {
                    let mut red = 0;
                    let mut green = 0;
                    let mut blue = 0;

                    let game_set_items = game_set.split(",");
                    for item in game_set_items {
                        let game_set_items_split: Vec<&str> = item.trim().split(" ").collect();

                        let item_value: usize = game_set_items_split[0]
                            .parse()
                            .expect("Expect cube value to be an unsigned integer");
                        let item_color = game_set_items_split[1];

                        match item_color {
                            "red" => red += item_value,
                            "green" => green += item_value,
                            "blue" => blue += item_value,
                            _ => continue,
                        };
                    }

                    current_game.push(GameMetadata { red, green, blue });
                }
                result.push(current_game);
            }
            Err(_) => panic!("Cannot read line"),
        };
    }
    return result;
}
