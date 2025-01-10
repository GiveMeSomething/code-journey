use std::{cmp::max, collections::HashMap};

// Thought process:
//
// Problem:
// Given a string s and an integer k,
// return the length of the longest substring of s,
// such that the frequency of each character in this substring
// is GTE to k.

// Hint: Sliding window problem
// For sliding window problem that trying to find the maximum,
// we always try to increase the substring length until we cannot increase it without violating requirement
// Then we decrease it to satisfy the requirement

// First attempt: We try to increase the window until it satisfy the requirement,
// then decrease it if current does not satisfy the requirement
// Problem: The current substring does not always satisfy the requirement, so the start is always increase with the end

// When this happened, we should look for another requirement that in turn can satisfy the main problem
// For this problem, the requirement is that for a substring with n unique character, how many of them can satisfy the requirement
// For instance, for a substring with only 1 unique character, how many of them have more than k occurence?
// e.g. "aaa" contains only "a" and "a" have 3 occurences. But "aaab" does not fall in this category as it has 2 unique character
// By finding in each unique character count, we then can deduce the result for the main problem

// IMO, this should be a HARD problem on Leetcode
// The step to the hidden requirement is very hard to see without any hint

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
    for i in 1..=unique_count {
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
