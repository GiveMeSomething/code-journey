use std::{cmp::max, collections::HashMap};

pub fn character_replacement(s: String, k: i32) -> i32 {
    let mut char_map: HashMap<&str, i32> = HashMap::new();
    let mut start = 0;
    let mut end = 0;

    let mut max_occur = 0;
    let mut result = 0;

    while end < s.len() {
        let current_char_count = {
            let char_count = char_map.entry(s.get(end..end + 1).unwrap()).or_insert(0);
            *char_count += 1;
            *char_count
        };

        max_occur = max(max_occur, current_char_count);

        end += 1;

        if max_occur + k < i32::try_from(end - start).expect("Out of bound for i32") {
            start += 1;
            let first_char_count = char_map
                .entry(s.get(start - 1..start).unwrap())
                .or_insert(0);
            *first_char_count -= 1;

            // Search for max from the char map
            max_occur = match char_map.values().max() {
                Some(v) => *v,
                None => 0,
            };

            result = max(
                result,
                i32::try_from(end - start).expect("Out of bound for i32"),
            );
        }
    }

    i32::try_from(end - start).expect("Out of bound for i32")
}

#[cfg(test)]
mod test {
    use super::character_replacement;

    #[test]
    fn test_character_replacement() {
        let test_cases: Vec<(&str, i32, i32)> = vec![
            ("ABAB", 2, 4),
            ("AABABBA", 1, 4),
            ("AAAA", 2, 4),
            ("ABACADAA", 2, 6),
        ];

        for (input, k, expected) in test_cases {
            let result = character_replacement(input.to_string(), k);
            println!("Case {} with k = {}", input, k);
            assert_eq!(result, expected);
        }
    }
}
