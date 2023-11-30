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

    // True = Moved, False = Stay
    fn follow(&mut self, point: &Point) -> bool {
        // Head and tail still touching. Skip tail move
        if (self.x - point.x).abs() <= 1 && (self.y - point.y).abs() <= 1 {
            return false;
        }

        // Same column, but not touching
        if self.x == point.x && (self.y - point.y).abs() > 1 {
            if self.y < point.y {
                self.y += 1;
            } else {
                self.y -= 1;
            }
            return true;
        }

        // Same row, but not touching
        if self.y == point.y && (self.x - point.x).abs() > 1 {
            if self.x < point.x {
                self.x += 1;
            } else {
                self.x -= 1;
            }
            return true;
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
        return true;
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
    let mut head = Point::new(0, 0);
    let mut tail = Point::new(0, 0);

    let mut tile_set: HashSet<String> = HashSet::new();
    tile_set.insert(String::from("0-0"));

    for movement in moves {
        for _ in 0..movement.move_value {
            match movement.direction.as_str() {
                "L" => head.x -= 1,
                "R" => head.x += 1,
                "U" => head.y += 1,
                "D" => head.y -= 1,
                _ => {}
            };

            let moved = tail.follow(&head);
            if moved {
                tile_set.insert(format!("{}-{}", tail.x, tail.y));
            }
        }
    }

    return tile_set.len();
}

pub fn count_covered_tile_10(moves: &Vec<MoveDirection>) -> usize {
    // Init Vec<Point>
    let mut points: Vec<Point> = vec![];
    for _ in 0..10 {
        points.push(Point::new(0, 0));
    }

    let mut tile_set: HashSet<String> = HashSet::new();
    tile_set.insert(String::from("0-0"));

    for movement in moves {
        for _ in 0..movement.move_value {
            {
                let head = points
                    .get_mut(0)
                    .expect("There should be a point at index 0");
                match movement.direction.as_str() {
                    "L" => head.x -= 1,
                    "R" => head.x += 1,
                    "U" => head.y += 1,
                    "D" => head.y -= 1,
                    _ => {}
                };
            }

            for i in 1..10 {
                let last_point = {
                    let point = points.get(i - 1).unwrap();
                    Point::new(point.x, point.y)
                };
                let current_point = points.get_mut(i).unwrap();
                let moved = current_point.follow(&last_point);
                if !moved {
                    break;
                }

                if i == 9 {
                    tile_set.insert(format!("{}-{}", current_point.x, current_point.y));
                }
            }
        }
    }

    return tile_set.len();
}
