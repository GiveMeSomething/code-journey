pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
    let chars_char_map = to_char_map(chars);
    let mut result = 0;
    for word in words {
        println!("Current word {}", word);
        let word_len = word.len();
        let word_char_map = to_char_map(word);
        if is_good_string(&word_char_map, &chars_char_map) {
            println!("OK");
            result += word_len;
        }
    }
    result as i32
}

pub fn to_char_map(s: String) -> [i32; 26] {
    let mut char_map = [0; 26];
    for character in s.chars() {
        char_map[character as usize - 97] += 1;
    }
    char_map
}

pub fn is_good_string(input_char_map: &[i32; 26], target_char_map: &[i32; 26]) -> bool {
    for i in 0..input_char_map.len() {
        if input_char_map[i] != 0 && input_char_map[i] > target_char_map[i] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::count_characters;

    #[test]
    fn test_count_characters() {
        let test_cases: Vec<(Vec<&str>, &str, i32)> = vec![
            (vec!["cat", "bt", "hat", "tree"], "atach", 6),
            (vec!["hello", "world", "leetcode"], "welldonehoneyr", 10),
        ];

        for (words, chars, expected) in test_cases {
            println!("Test case {:?} {}", words, chars);

            let result = count_characters(
                words.iter().map(|v| String::from(*v)).collect(),
                String::from(chars),
            );
            assert_eq!(result, expected);
        }
    }
}
