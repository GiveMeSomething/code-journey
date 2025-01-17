use std::collections::HashMap;

pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
    let word_len = words[0].len();
    let substring_len = word_len * words.len();

    // Construct a map to check
    let mut word_map: HashMap<String, i32> = HashMap::new();
    for word in words {
        let count = word_map.entry(word).or_insert(0);
        *count += 1;
    }

    let mut result: Vec<i32> = vec![];

    for offset in 0..word_len {
        let mut current_word_map: HashMap<String, i32> = HashMap::new();
        let mut start = offset;

        while start + substring_len <= s.len() {
            if current_word_map.is_empty() {
                // If empty, init it with the current substring
                let current_substring = &s[start..start + substring_len];
                current_word_map = to_map(current_substring, word_len);
            } else {
                // Update the map with the latest word
                let last_word = &s[start + substring_len - word_len..start + substring_len];
                let count = current_word_map.entry(String::from(last_word)).or_insert(0);
                *count += 1;
            }

            println!("current word map {:?}", current_word_map);

            if is_map_equal(&current_word_map, &word_map) {
                result.push(start as i32);

                let removed_word = String::from(&s[start..start + word_len]);
                let count = current_word_map.entry(removed_word).or_insert(0);
                *count -= 1;
                start += word_len;
                continue;
            }

            current_word_map.clear();
            start += word_len;
        }
    }

    result
}

pub fn to_map(s: &str, word_len: usize) -> HashMap<String, i32> {
    let mut word_map = HashMap::new();
    let mut start = 0;
    while start + word_len <= s.len() {
        let current_word = String::from(&s[start..start + word_len]);
        let count = word_map.entry(current_word).or_insert(0);
        *count += 1;

        start += word_len;
    }
    word_map
}

pub fn is_map_equal(m1: &HashMap<String, i32>, m2: &HashMap<String, i32>) -> bool {
    for (substring, count) in m1 {
        if !m2.contains_key(substring) {
            return false;
        }

        if *count != *m2.get(substring).unwrap() {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod test {
    use super::find_substring;

    #[test]
    fn test_find_substring() {
        let test_cases: Vec<(&str, Vec<&str>, Vec<i32>)> = vec![
            ("barfoothefoobarman", vec!["foo", "bar"], vec![0, 9]),
            (
                "wordgoodgoodgoodbestword",
                vec!["word", "good", "best", "word"],
                vec![],
            ),
            (
                "barfoofoobarthefoobarman",
                vec!["bar", "foo", "the"],
                vec![6, 9, 12],
            ),
            (
                "wordgoodgoodgoodbestword",
                vec!["word", "good", "best", "good"],
                vec![8],
            ),
            (
                "aaaaaaaaaaaaaa",
                vec!["aa", "aa"],
                vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            ),
            ("abaababbaba", vec!["ba", "ab", "ab"], vec![1, 3]),
        ];

        for (s, words, expected) in test_cases {
            println!("TEST s: {}, words: {:?}", s, words);

            let mut result = find_substring(
                String::from(s),
                words.iter().map(|v| String::from(*v)).collect(),
            );

            println!("{:?}", result);

            // Result can be in any order, so we sort before compare with expected
            result.sort();

            assert_eq!(result.len(), expected.len());
            for i in 0..result.len() {
                assert_eq!(result[i], expected[i]);
            }
        }
    }
}
