use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Lines},
};

fn read_shape_from_file() -> Lines<BufReader<File>> {
    let file = match File::open("src/inputs/two.txt") {
        Ok(file) => file,
        Err(error) => panic!("Cannot open day 2 inputs file with error {:?}", error),
    };

    let reader = BufReader::new(file);
    return reader.lines();
}

pub fn calculate_score_part_1() -> i32 {
    let inputs = read_shape_from_file();

    let mut result: i32 = 0;
    for input in inputs {
        match input {
            Ok(input) => {
                let shapes: Vec<&str> = input.split(" ").collect();
                result += calculate_score_from_shape(shapes[0], shapes[1]);
            }
            Err(error) => panic!("Cannot read line with error {:?}", error),
        };
    }

    return result;
}

fn calculate_score_from_shape(shape1: &str, shape2: &str) -> i32 {
    let score_map: HashMap<&str, i32> =
        HashMap::from([("A", 1), ("B", 2), ("C", 3), ("X", 1), ("Y", 2), ("Z", 3)]);

    let opponent_score = match score_map.get(shape1) {
        Some(score) => *score,
        None => panic!("Cannot identify shape {} score", shape1),
    };
    let my_score = match score_map.get(shape2) {
        Some(score) => *score,
        None => panic!("Cannot identify shape {} score", shape2),
    };

    if (my_score == opponent_score + 1) || (my_score == 1 && opponent_score == 3) {
        // Win condition
        return my_score + 6;
    } else if my_score == opponent_score {
        // Draw condition
        return my_score + 3;
    } else {
        return my_score;
    }
}

pub fn calculate_score_part_2() -> i32 {
    let inputs = read_shape_from_file();

    let mut result: i32 = 0;
    for input in inputs {
        match input {
            Ok(input) => {
                let messages: Vec<&str> = input.split(" ").collect();
                result += calculate_score_from_outcome(messages[0], messages[1]);
            }
            Err(error) => panic!("Cannot read line with error {:?}", error),
        };
    }

    return result;
}

fn calculate_score_from_outcome(shape: &str, outcome: &str) -> i32 {
    let score_map: HashMap<&str, i32> =
        HashMap::from([("A", 1), ("B", 2), ("C", 3), ("X", 1), ("Y", 2), ("Z", 3)]);

    let opponent_score = match score_map.get(shape) {
        Some(score) => *score,
        None => panic!("Cannot identify shape {} score", shape),
    };

    if outcome.eq("X") {
        // Losing condition
        if opponent_score == 1 {
            return 3;
        } else {
            return opponent_score - 1;
        }
    } else if outcome.eq("Y") {
        // Draw condition
        return opponent_score + 3;
    } else {
        // Winning condition
        if opponent_score == 3 {
            return 1 + 6;
        } else {
            return opponent_score + 1 + 6;
        }
    }
}
