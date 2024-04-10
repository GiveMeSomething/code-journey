use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_signal_from_file() -> Vec<(String, String)> {
    let file = File::open("src/inputs/eight.txt").expect("Cannot find/open input file for day 8");
    let reader = BufReader::new(file);

    let mut result: Vec<(String, String)> = vec![];
    for line in reader.lines() {
        let current_line = line.unwrap();

        let parts = current_line
            .split("|")
            .map(|part| part.trim())
            .collect::<Vec<&str>>();
        result.push((parts[0].to_string(), parts[1].to_string()));
    }
    return result;
}

pub fn count_unique_signals(signals: &Vec<(String, String)>) -> usize {
    let mut sum = 0;

    for signal in signals {
        let (_, second) = signal;

        sum += second
            .split(" ")
            .filter(|part| part.len() == 2 || part.len() == 4 || part.len() == 3 || part.len() == 7)
            .count();
    }

    return sum;
}

pub fn sum_decode_signal(signals: &Vec<(String, String)>) -> usize {
    let magic_map: HashMap<usize, usize> = vec![
        (42, 0),
        (17, 1),
        (34, 2),
        (39, 3),
        (30, 4),
        (37, 5),
        (41, 6),
        (25, 7),
        (49, 8),
        (45, 9),
    ]
    .into_iter()
    .collect();

    let mut sum = 0;

    for signal in signals {
        let mut char_map = HashMap::new();
        let (first, second) = signal;
        for char in first.split(" ").flat_map(|part| part.chars()) {
            let value = char_map.entry(char).or_insert(0);
            *value += 1;
        }

        let mut decode = 0;
        for part in second.split(" ") {
            let mut part_sum = 0;
            for char in part.chars() {
                part_sum += char_map.get(&char).unwrap_or(&0);
            }
            decode = decode * 10 + magic_map.get(&part_sum).unwrap_or(&0);
        }

        sum += decode;
    }

    return sum;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_unique_signals() {
        let inputs = vec![
            vec![(
                String::from("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb"),
                String::from("fdgacbe cefdb cefbgd gcbe"),
            )],
            vec![(
                String::from("edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec"),
                String::from("fcgedb cgb dgebacf gc"),
            )],
        ];
        let expecteds = [2, 3];

        for i in 0..inputs.len() {
            let receive = count_unique_signals(&inputs[i]);
            assert_eq!(receive, expecteds[i]);
        }
    }

    #[test]
    fn test_sum_decode_signal() {
        let inputs = vec![
            vec![(
                String::from("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb"),
                String::from("fdgacbe cefdb cefbgd gcbe"),
            )],
            vec![(
                String::from("edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec"),
                String::from("fcgedb cgb dgebacf gc"),
            )],
        ];
        let expecteds = [8394, 9781];

        for i in 0..inputs.len() {
            let receive = sum_decode_signal(&inputs[i]);
            assert_eq!(receive, expecteds[i]);
        }
    }
}
