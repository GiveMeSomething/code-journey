use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
pub struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug)]
pub struct VentLine {
    from: Point,
    to: Point,
    is_diagonal: bool,
}

impl VentLine {
    pub fn new(from_x: isize, from_y: isize, to_x: isize, to_y: isize) -> VentLine {
        VentLine {
            from: Point {
                x: from_x,
                y: from_y,
            },
            to: { Point { x: to_x, y: to_y } },
            is_diagonal: from_x == to_x || from_y == to_y,
        }
    }
}

pub fn read_vents_from_file() -> Vec<VentLine> {
    let file = File::open("src/inputs/five.txt").expect("Cannot find/open input file for day 5");
    let reader = BufReader::new(file);

    let mut vent_lines: Vec<VentLine> = vec![];
    for line in reader.lines() {
        let current_line = line.expect("Cannot read line from input file");
        vent_lines.push(extract_vent_line(current_line));
    }
    return vent_lines;
}

fn extract_vent_line(s: String) -> VentLine {
    let vent_line_regex =
        Regex::new(r"([0-9]*),([0-9]*) -> ([0-9]*),([0-9*])").expect("Should be a valid regex");

    let (_, [from_x, from_y, to_x, to_y]) = vent_line_regex
        .captures(s.as_str())
        .map(|c| c.extract())
        .unwrap();
    return VentLine::new(
        from_x.parse::<isize>().unwrap(),
        from_y.parse::<isize>().unwrap(),
        to_x.parse::<isize>().unwrap(),
        to_y.parse::<isize>().unwrap(),
    );
}
