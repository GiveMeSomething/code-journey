pub fn count_substrings(s: String) -> i32 {
    let mut result = 0;

    let input_chars: Vec<char> = s.chars().collect();

    // Find odd palindrome
    for i in 0..input_chars.len() {
        let mut start = i;
        let mut end = i;

        while end < s.len() && input_chars[start] == input_chars[end] {
            result += 1;
            if start == 0 {
                break;
            }

            start -= 1;
            end += 1;
        }
    }

    // Find even palindrome (by finding 2 same character substring)
    let mut start = 0;
    let mut end = 1;
    while end < s.len() {
        if input_chars[start] != input_chars[end] {
            start += 1;
            end += 1;
            continue;
        }

        // Double character = 1 palindrome
        let mut inner_start = start;
        let mut inner_end = end;

        while inner_end < input_chars.len() && input_chars[inner_start] == input_chars[inner_end] {
            result += 1;
            if inner_start == 0 {
                break;
            }

            inner_start -= 1;
            inner_end += 1;
        }

        start += 1;
        end += 1;
    }

    result
}

#[cfg(test)]
mod test {
    use super::count_substrings;

    #[test]
    fn test_count_substrings() {
        let test_cases: Vec<(&str, i32)> = vec![("abc", 3), ("aaa", 6), ("aaaaa",)];
        for (s, expected) in test_cases {
            let result = count_substrings(String::from(s));
            assert_eq!(result, expected);
        }
    }
}
