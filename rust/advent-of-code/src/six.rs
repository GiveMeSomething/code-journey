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

pub fn get_start_of_packet(signal: &String) -> usize {
    // let mut char_set: HashSet<char> = HashSet::new();
    // for i in 4..signal.len() - 4 {
    //     for char in signal[i - 4..i].chars() {
    //         char_set.insert(char);
    //     }
    //     if char_set.len() == 4 {
    //         return i;
    //     }
    //     char_set.clear();
    // }

    // return 0;

    return find_unique_seq(signal, 4);
}

pub fn get_start_of_message(signal: &String) -> usize {
    // let mut char_set: HashSet<char> = HashSet::new();
    // for i in 14..signal.len() - 4 {
    //     for char in signal[i - 14..i].chars() {
    //         char_set.insert(char);
    //     }
    //     if char_set.len() == 14 {
    //         return i;
    //     }
    //     char_set.clear();
    // }

    // return 0;

    return find_unique_seq(signal, 14);
}

fn find_unique_seq(input: &String, seq_len: usize) -> usize {
    let mut char_map: HashMap<char, usize> = HashMap::new();
    let mut input_chars: Vec<char> = vec![];

    for (i, char) in input.char_indices() {
        input_chars.push(char);

        if i < seq_len {
            let value = match char_map.get(&char) {
                Some(value) => *value,
                None => 0,
            };
            char_map.insert(char, value + 1);
            continue;
        }

        let bottom_char = input_chars.get(i - seq_len).unwrap();
        let bottom_char_count = match char_map.get(bottom_char) {
            Some(value) => *value,
            None => 0,
        };

        if bottom_char_count - 1 <= 0 {
            char_map.remove(bottom_char);
        } else {
            char_map.insert(*bottom_char, bottom_char_count - 1);
        }

        let value = match char_map.get(&char) {
            Some(value) => *value,
            None => 0,
        };
        char_map.insert(char, value + 1);

        if char_map.len() == seq_len {
            // Index + 1 = The number of character
            return i + 1;
        }
    }

    return 0;
}
