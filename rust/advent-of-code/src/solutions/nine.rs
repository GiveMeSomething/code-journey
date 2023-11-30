use std::{
    collections::HashSet,
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
};

pub struct MoveDirection {
    direction: String,
    move_value: usize,
}

impl Debug for MoveDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MoveDirection [{} {}]\n",
            self.direction, self.move_value
        )
    }
}

struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Point {
        Point { x, y }
    }

    fn follow(&mut self, point: &Point) {
        // Head and tail still touching. Skip tail move
        if (self.x - point.x).abs() <= 1 && (self.y - point.y).abs() <= 1 {
            return;
        }

        // Same column, but not touching
        if self.x == point.x && (self.y - point.y).abs() > 1 {
            // Move tail closer to head
            if self.y < point.y {
                self.y += 1;
            } else {
                self.y -= 1;
            }
            return;
        }

        // Same row, but not touching
        if self.y == point.y && (self.x - point.x).abs() > 1 {
            if self.x < point.x {
                self.x += 1;
            } else {
                self.y -= 1;
            }
            return;
        }

        // Else, move diagonally toward head
        if self.x < point.x {
            self.x += 1;
        } else {
            self.x -= 1;
        }

        if self.y < point.y {
            self.y += 1;
        } else {
            self.y -= 1;
        }
    }
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

pub fn count_covered_tile(moves: &Vec<MoveDirection>) -> usize {
    let mut head_x: isize = 0;
    let mut head_y: isize = 0;

    let mut tail_x: isize = 0;
    let mut tail_y: isize = 0;

    let mut tile_set: HashSet<String> = HashSet::new();
    tile_set.insert(String::from("0-0"));

    for movement in moves {
        for _ in 0..movement.move_value {
            match movement.direction.as_str() {
                "L" => head_x -= 1,
                "R" => head_x += 1,
                "U" => head_y += 1,
                "D" => head_y -= 1,
                _ => {}
            };

            // Head and tail still touching. Skip tail move
            if (head_x - tail_x).abs() <= 1 && (head_y - tail_y).abs() <= 1 {
                continue;
            }

            // Same column, but not touching
            if head_x == tail_x && (head_y - tail_y).abs() > 1 {
                // Move tail closer to head
                match movement.direction.as_str() {
                    "U" => tail_y += 1,
                    "D" => tail_y -= 1,
                    _ => {}
                };
                tile_set.insert(format!("{}-{}", tail_x, tail_y));
                continue;
            }

            // Same row, but not touching
            if head_y == tail_y && (head_x - tail_x).abs() > 1 {
                // Move tail closer to head
                match movement.direction.as_str() {
                    "L" => tail_x -= 1,
                    "R" => tail_x += 1,
                    _ => {}
                };
                tile_set.insert(format!("{}-{}", tail_x, tail_y));
                continue;
            }

            // Else, move diagonally toward head
            if tail_x < head_x {
                tail_x += 1;
            } else {
                tail_x -= 1;
            }

            if tail_y < head_y {
                tail_y += 1;
            } else {
                tail_y -= 1;
            }
            tile_set.insert(format!("{}-{}", tail_x, tail_y));
        }
    }

    return tile_set.len();
}

pub fn count_covered_tile_10_knots(moves: &Vec<MoveDirection>) -> usize {
    let mut head_x: isize = 0;
    let mut head_y: isize = 0;

    let mut tail_x: isize = 0;
    let mut tail_y: isize = 0;

    let mut tile_set: HashSet<String> = HashSet::new();
    tile_set.insert(String::from("0-0"));

    for movement in moves {
        for _ in 0..movement.move_value {
            match movement.direction.as_str() {
                "L" => head_x -= 1,
                "R" => head_x += 1,
                "U" => head_y += 1,
                "D" => head_y -= 1,
                _ => {}
            };

            // Head and tail still touching. Skip tail move
            if (head_x - tail_x).abs() <= 1 && (head_y - tail_y).abs() <= 1 {
                continue;
            }

            // Same column, but not touching
            if head_x == tail_x && (head_y - tail_y).abs() > 1 {
                // Move tail closer to head
                match movement.direction.as_str() {
                    "U" => tail_y += 1,
                    "D" => tail_y -= 1,
                    _ => {}
                };
                tile_set.insert(format!("{}-{}", tail_x, tail_y));
                continue;
            }

            // Same row, but not touching
            if head_y == tail_y && (head_x - tail_x).abs() > 1 {
                // Move tail closer to head
                match movement.direction.as_str() {
                    "L" => tail_x -= 1,
                    "R" => tail_x += 1,
                    _ => {}
                };
                tile_set.insert(format!("{}-{}", tail_x, tail_y));
                continue;
            }

            // Else, move diagonally toward head
            if tail_x < head_x {
                tail_x += 1;
            } else {
                tail_x -= 1;
            }

            if tail_y < head_y {
                tail_y += 1;
            } else {
                tail_y -= 1;
            }
            tile_set.insert(format!("{}-{}", tail_x, tail_y));
        }
    }

    return tile_set.len();
}
