use std::{cmp::max, collections::HashMap};

pub fn longest_substring(s: String, k: i32) -> i32 {
    if k > i32::try_from(s.len()).unwrap() {
        return 0;
    }

    if k == 1 {
        return i32::try_from(s.len()).unwrap();
    }

    let mut char_count = [0; 26];
    for character in s.chars() {
        char_count[(character as usize) - 97] += 1;
    }

    let mut max_len = 0;

    let unique_count = char_count.into_iter().filter(|v| *v > 0).count();
    for i in 2..=unique_count {
        let mut start = 0;
        let mut end = i;

        // Populate the map
        let mut char_map: HashMap<String, i32> = HashMap::new();
        for j in 0..i {
            let count = char_map.entry(String::from(&s[j..j + 1])).or_insert(0);
            *count += 1;
        }

        while end < s.len() {
            let count = char_map.entry(String::from(&s[end..end + 1])).or_insert(0);
            *count += 1;

            if char_map.values().filter(|v| **v > 0).all(|v| *v >= k) {
                max_len = max(max_len, i32::try_from(end - start + 1).unwrap());
            }

            while char_map.values().filter(|v| **v > 0).count() > i {
                let count = char_map
                    .entry(String::from(&s[start..start + 1]))
                    .or_insert(0);
                *count -= 1;
                start += 1;
            }

            end += 1;
        }
    }

    max_len
}

#[cfg(test)]
mod test {
    use super::longest_substring;

    #[test]
    fn test_longest_substring() {
        let test_cases: Vec<(&str, i32, i32)> = vec![
            ("aaabb", 3, 3),
            ("ababbc", 2, 5),
            ("a", 1, 1),
            ("a", 2, 0),
            ("abcdefgh", 2, 0),
        ];

        for (s, k, expected) in test_cases {
            println!("Test case: {}, k = {}", s, k);
            let result = longest_substring(String::from(s), k);
            assert_eq!(result, expected);
        }
    }
}
