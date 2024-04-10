use std::{
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
}
