use std::collections::HashMap;

/**
 * Is Unique: Implement an algorithm to determine if a string has all unique characters.
 * What if youcannot use additional data structures?
 */

// s contains ASCII characters (utf8 if normal ASCII, utf16 if extended ASCII, which is 128-bit and 256-bit long, respectively)
#[allow(dead_code)]
fn is_unique(s: String) -> bool {
    let mut appearance = [false; 128];

    for char in s.bytes() {
        let char_appeared = &mut appearance[char as usize];
        if *char_appeared {
            return false;
        }

        *char_appeared = true;
    }

    true
}

// Remember to ask for clarification for what s can contains
// If it contains only lowercase/uppercase English character, use an array of length 26
// Notes: "A" = 65, "a" = 97
// Else, use hash map
// If dealing with Unicode, remember to split them using grapheme
#[allow(dead_code)]
fn is_permutation(s1: String, s2: String) -> bool {
    let mut occurred = [0; 26];
    for char in s1.chars() {
        occurred[char as usize - 97] += 1;
    }
    for char in s2.chars() {
        occurred[char as usize - 97] -= 1;
    }
    for occurance_diff in occurred {
        if occurance_diff != 0 {
            return false;
        }
    }
    true
}

// Reasoning: If a string is a palindrome, it will have the same character count for any character
// OR will have a single character as core
// We use a hash map, increase count by 1, and if encounter 1, reduce to 0 to signify that current character has its counterpart
// Finally, we check for only 1 "core", or none
// Remember to skip the "space"
#[allow(dead_code)]
fn have_palindrome_permutation(s: String) -> bool {
    let mut char_map: HashMap<char, i32> = HashMap::new();
    for char in s.chars() {
        if char == ' ' {
            continue;
        }

        let count = char_map.entry(char).or_insert(0);
        if *count == 1 {
            *count = 0;
        } else {
            *count += 1;
        }
    }

    let mut core_allowed = 1;
    for (_, count) in char_map {
        if count == 1 {
            core_allowed -= 1;
            if core_allowed < 0 {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod test {
    use crate::cracking::strings::have_palindrome_permutation;

    use super::{is_permutation, is_unique};

    #[test]
    fn test_is_unique() {
        let test_cases: Vec<(&str, bool)> = vec![
            ("abc", true),
            ("aabc", false),
            ("!@#$%^&*()", true),
            ("1234567890!@#$%^&*()))))_+_)(*&^%", false),
        ];

        for (s, expected) in test_cases {
            println!("TEST CASE {}", s);

            let result = is_unique(String::from(s));
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_is_permutation() {
        let test_cases: Vec<(&str, &str, bool)> = vec![
            ("abc", "cba", true),
            ("asdfghjkl", "lkjhgfdsa", true),
            ("hello", "world", false),
        ];

        for (s1, s2, expected) in test_cases {
            let result = is_permutation(String::from(s1), String::from(s2));
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_have_palindrome_permutation() {
        let test_cases: Vec<(&str, bool)> = vec![
            ("tact coa", true),
            ("leet teel", true),
            ("hello world", false),
        ];

        for (s, expected) in test_cases {
            let result = have_palindrome_permutation(String::from(s));
            assert_eq!(result, expected);
        }
    }
}
