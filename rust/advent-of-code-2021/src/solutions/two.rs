use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub enum Direction {
    Forward,
    Down,
    Up,
}

pub struct Command {
    direction: Direction,
    steps: usize,
}

pub fn read_commands_from_file() -> Vec<Command> {
    let file = File::open("src/inputs/two.txt").expect("Cannot find/open input file for day2");
    let reader = BufReader::new(file);

    let mut commands: Vec<Command> = vec![];
    for line in reader.lines() {
        let current_line = line.expect("Cannot read current line. Aborting...");

        let parts: Vec<&str> = current_line.split(" ").collect();
        let direction = match parts[0] {
            "forward" => Direction::Forward,
            "down" => Direction::Down,
            "up" => Direction::Up,
            _ => Direction::Forward,
        };

        commands.push(Command {
            direction,
            steps: parts[1]
                .parse()
                .expect("Cannot parse step from string to usize"),
        });
    }

    return commands;
}

pub fn simulate_commands(commands: &Vec<Command>) -> (isize, isize) {
    let mut x: isize = 0;
    let mut y: isize = 0;
    for command in commands {
        let value: isize = isize::try_from(command.steps).unwrap();
        match command.direction {
            Direction::Forward => x += value,
            Direction::Down => y += value,
            Direction::Up => y -= value,
        };
    }

    (x, y)
}

pub fn simulate_commands_with_aim(commands: &Vec<Command>) -> (isize, isize) {
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut aim: isize = 0;
    for command in commands {
        let value: isize = isize::try_from(command.steps).unwrap();
        match command.direction {
            Direction::Forward => {
                x += value;
                y += value * aim;
            }
            Direction::Down => aim += value,
            Direction::Up => aim -= value,
        };
    }

    (x, y)
}
