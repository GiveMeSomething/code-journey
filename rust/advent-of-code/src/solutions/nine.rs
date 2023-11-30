use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub struct MoveDirection {
    direction: String,
    move_value: usize,
}

pub fn read_moves_from_file() -> Vec<MoveDirection> {
    let file = File::open("src/inputs/nine.txt")
        .unwrap_or_else(|err| panic!("Cannot open day9 input file with error {:?}", err));
    let reader = BufReader::new(file);

    let mut moves: Vec<MoveDirection> = vec![];
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let tokens: Vec<&str> = line.split(" ").collect();

                let direction =
                    String::from(*tokens.get(0).expect("Expect a direction at index 0"));
                let move_value: usize = tokens
                    .get(1)
                    .expect("Expect a value at index 1")
                    .parse()
                    .expect("Expect move_value to be a number");
                moves.push(MoveDirection {
                    direction,
                    move_value,
                });
            }
            Err(err) => {
                println!("Cannot parse current line with error {:?}", err);
                continue;
            }
        };
    }
    return moves;
}
