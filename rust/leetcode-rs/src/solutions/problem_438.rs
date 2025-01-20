// Base solution
// pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
//     if p.len() > s.len() {
//         return vec![];
//     }

//     // Create a map of all p character
//     let mut p_char_map: HashMap<String, i32> = HashMap::new();
//     for i in 0..p.len() {
//         let count = p_char_map.entry(String::from(&p[i..i + 1])).or_insert(0);
//         *count += 1;
//     }

//     let mut start = 0;
//     let mut result: Vec<i32> = vec![];
//     let mut tracking_char_map: HashMap<String, i32> = HashMap::new();

//     while start + p.len() <= s.len() {
//         if tracking_char_map.is_empty() {
//             // Init current char_map if empty (first iteration)
//             for i in 0..p.len() {
//                 let count = tracking_char_map
//                     .entry(String::from(&s[i..i + 1]))
//                     .or_insert(0);
//                 *count += 1;
//             }
//         } else {
//             let count = tracking_char_map
//                 .entry(String::from(&s[start + p.len() - 1..start + p.len()]))
//                 .or_insert(0);
//             *count += 1;
//         }

//         if is_anagram(&tracking_char_map, &p_char_map) {
//             result.push(start as i32);
//         }

//         let first_char_count = tracking_char_map
//             .entry(String::from(&s[start..start + 1]))
//             .or_insert(0);
//         *first_char_count -= 1;
//         start += 1;
//     }

//     result
// }

// fn is_anagram(m1: &HashMap<String, i32>, m2: &HashMap<String, i32>) -> bool {
//     for (key, value) in m2 {
//         if !m1.contains_key(key) {
//             return false;
//         }
//         if *m1.get(key).unwrap() != *value {
//             return false;
//         }
//     }
//     true
// }

// Faster way, we use an array of length 26 to count character instead of HashMap to avoid computation overhead
pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
    if p.len() > s.len() {
        return vec![];
    }

    let s_chars: Vec<char> = s.chars().collect();
    let p_chars: Vec<char> = p.chars().collect();

    let mut anagram_map = [0; 26];
    for char in p_chars {
        anagram_map[char as usize - 97] += 1;
    }

    let mut tracking_map = [0; 26];
    let mut result: Vec<i32> = vec![];
    let mut start = 0;
    while start + p.len() <= s.len() {
        if start == 0 {
            for i in 0..p.len() - 1 {
                tracking_map[s_chars[i] as usize - 97] += 1;
            }
        }

        tracking_map[s_chars[start + p.len() - 1] as usize - 97] += 1;

        if is_anagram_chars(&tracking_map, &anagram_map) {
            result.push(start as i32);
        }

        tracking_map[s_chars[start] as usize - 97] -= 1;
        start += 1;
    }

    result
}

fn is_anagram_chars(m1: &[i32; 26], m2: &[i32; 26]) -> bool {
    for i in 0..m1.len() {
        if m1[i] != m2[i] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::find_anagrams;

    #[test]
    fn test_find_anagrams() {
        let test_cases: Vec<(&str, &str, Vec<i32>)> = vec![
            ("cbaebabacd", "abc", vec![0, 6]),
            ("abab", "ab", vec![0, 1, 2]),
            ("aa", "bb", vec![]),
        ];

        for (s, p, expected) in test_cases {
            let result = find_anagrams(String::from(s), String::from(p));

            println!("TEST CASE {} {}", s, p);
            assert_eq!(result.len(), expected.len());

            for i in 0..result.len() {
                assert_eq!(result[i], expected[i]);
            }
        }
    }
}
