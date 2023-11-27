use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_signal_from_file() -> String {
    let file = File::open("src/inputs/six.txt")
        .unwrap_or_else(|err| panic!("Cannot open file with error {:?}", err));

    // There should be a single line
    let mut signal: String = String::from("");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        signal = line.unwrap_or_else(|err| panic!("Cannot read line with error {:?}", err));
        break;
    }
    return signal;
}

pub fn get_signal_start(signal: &String) -> usize {
    let mut char_set: HashSet<char> = HashSet::new();
    for i in 4..signal.len() - 4 {
        for char in signal[i - 4..i].chars() {
            char_set.insert(char);
        }
        if char_set.len() == 4 {
            return i;
        }
        char_set.clear();
    }

    return 0;
}
