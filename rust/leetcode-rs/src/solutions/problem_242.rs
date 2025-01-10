pub fn is_anagram(s: String, t: String) -> bool {
    let mut char_map = [0; 26];
    for character in s.chars() {
        char_map[(character as usize) - 97] += 1;
    }

    for character in t.chars() {
        char_map[(character as usize) - 97] -= 1;
    }

    char_map.iter().all(|v| *v == 0)
}

#[cfg(test)]
mod test {
    use super::is_anagram;

    #[test]
    fn test_is_anagram() {
        let test_cases: Vec<(&str, &str, bool)> =
            vec![("anagram", "nagaram", true), ("rat", "car", false)];

        for (s, t, expected) in test_cases {
            let result = is_anagram(String::from(s), String::from(t));
            assert_eq!(result, expected);
        }
    }
}
