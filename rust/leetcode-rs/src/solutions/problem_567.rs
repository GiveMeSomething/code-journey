pub fn check_inclusion(s1: String, s2: String) -> bool {
    let target_len = s1.len();
    if target_len > s2.len() {
        return false;
    }

    let char_map = to_char_map(s1);
    let mut current_char_map = to_char_map(String::from(&s2[0..target_len - 1]));

    let mut start = 0;
    while start + target_len <= s2.len() {
        current_char_map[char_at(s2.to_string(), start + target_len - 1) as usize - 97] += 1;
        if is_permutation_v2(&current_char_map, &char_map) {
            return true;
        }

        current_char_map[char_at(s2.to_string(), start) as usize - 97] -= 1;
        start += 1;
    }
    false
}

pub fn is_permutation_v2(char_map_1: &[i32; 26], char_map_2: &[i32; 26]) -> bool {
    for i in 0..26 {
        if char_map_1[i] != char_map_2[i] {
            return false;
        }
    }
    true
}

pub fn is_permutation(s1: &str, char_map: &[i32; 26]) -> bool {
    let mut s1_char_map = [-1; 26];
    for character in s1.chars() {
        let index = (character as usize) - 97;
        if s1_char_map[index] == -1 {
            s1_char_map[index] = char_map[index];
        }
        s1_char_map[index] -= 1;
        if s1_char_map[index] < 0 {
            return false;
        }
    }

    return true;
}

pub fn to_char_map(s1: String) -> [i32; 26] {
    let mut char_map = [0; 26];
    for character in s1.chars() {
        char_map[(character as usize) - 97] += 1;
    }
    char_map
}

pub fn char_at(input: String, index: usize) -> char {
    return input[index..index + 1].chars().nth(0).unwrap();
}

#[cfg(test)]
mod test {
    use super::{check_inclusion, is_permutation};

    #[test]
    fn test_check_inclusion() {
        let test_cases: Vec<(&str, &str, bool)> = vec![
            ("ab", "eidbaooo", true),
            ("ab", "eidboaoo", false),
            // s1 can be longer than s2
            ("abcd", "a", false),
        ];

        for (s1, s2, expected) in test_cases {
            println!("Test case s1 {} s2 {}", s1, s2);
            let result = check_inclusion(String::from(s1), String::from(s2));
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_is_permutatiion() {
        // Prepare the char_map based on base string
        let base = "abc";
        let mut char_map = [0; 26];
        for character in base.chars() {
            char_map[(character as usize) - 97] += 1;
        }

        let test_cases: Vec<(&str, bool)> = vec![("abc", true), ("bca", true), ("aaaabc", false)];

        for (s1, expected) in test_cases {
            let result = is_permutation(s1, &char_map);
            assert_eq!(result, expected);
        }
    }
}
