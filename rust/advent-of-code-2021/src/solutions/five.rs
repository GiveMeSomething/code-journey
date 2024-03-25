use regex::Regex;
use std::{
    cmp::{max, min},
    collections::HashMap,
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
            is_diagonal: from_x != to_x && from_y != to_y,
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

pub fn count_intersection(vent_lines: &Vec<VentLine>) -> usize {
    let mut count = 0;

    let mut point_map: HashMap<String, usize> = HashMap::new();
    for vent_line in vent_lines {
        if vent_line.is_diagonal {
            continue;
        }

        // Vertical line
        if vent_line.from.x == vent_line.to.x {
            for i in min(vent_line.from.y, vent_line.to.y)..=max(vent_line.from.y, vent_line.to.y) {
                let key = vent_line.from.x.to_string() + "," + i.to_string().as_str();
                let value = point_map.entry(key).or_insert(0);
                *value += 1;
            }
        } else {
            for i in min(vent_line.from.x, vent_line.to.x)..=max(vent_line.from.x, vent_line.to.x) {
                let key = i.to_string() + "," + vent_line.from.y.to_string().as_str();
                let value = point_map.entry(key).or_insert(0);
                *value += 1;
            }
        }
    }

    for (_, intersect) in point_map {
        if intersect >= 2 {
            count += 1;
        }
    }

    return count;
}

fn extract_vent_line(s: String) -> VentLine {
    let vent_line_regex =
        Regex::new(r"([0-9]*),([0-9]*) -> ([0-9]*),([0-9]*)").expect("Should be a valid regex");

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
