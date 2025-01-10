use std::collections::HashMap;

pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
    if s.len() < 10 {
        return vec![];
    }

    let mut seq_counter: HashMap<String, i32> = HashMap::new();

    let mut start = 0;
    let mut end = 9;

    while end < s.len() {
        let current_seq = String::from(&s[start..end + 1]);
        let count = seq_counter.entry(current_seq).or_insert(0);
        *count += 1;

        start += 1;
        end += 1;
    }

    seq_counter
        .iter()
        .filter(|(_, v)| **v > 1)
        .map(|(k, _)| k.to_string())
        .collect::<Vec<String>>()
}

pub fn find_repeated_dna_sequences_v2(s: String) -> Vec<String> {
    if s.len() < 10 {
        return vec![];
    }

    let mut seq_counter: HashMap<String, i32> = HashMap::new();

    let mut result: Vec<String> = vec![];
    let mut start = 0;
    let mut end = 9;

    while end < s.len() {
        let current_seq = String::from(&s[start..end + 1]);
        let count = seq_counter.entry(current_seq).or_insert(0);
        *count += 1;

        if *count == 2 {
            result.push(String::from(&s[start..end + 1]));
        }

        start += 1;
        end += 1;
    }

    result
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_find_repeated_dna_sequences() {
        let test_cases: Vec<(&str, Vec<&str>)> = vec![
            (
                "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT",
                vec!["AAAAACCCCC", "CCCCCAAAAA"],
            ),
            ("AAAAAAAAAAAAA", vec!["AAAAAAAAAA"]),
            // ("A", vec![]),
        ];

        for (input, expected) in test_cases {
            let result = find_repeated_dna_sequences(String::from(input));
            let expected_set: HashSet<&str> = HashSet::from_iter(expected);
            println!("{:?}", expected_set);

            assert_eq!(expected_set.len(), result.len());

            for sequence in result {
                assert!(expected_set.contains(sequence.as_str()));
            }
        }
    }
}
