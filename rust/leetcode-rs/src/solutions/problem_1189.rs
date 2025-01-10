use std::{cmp::min, i32};

pub fn max_number_of_balloons(text: String) -> i32 {
    // "balloon" contains b,a,l,o,n

    let mut char_map = [0; 5];
    for character in text.chars() {
        if character == 'b' {
            char_map[0] += 1;
        }
        if character == 'a' {
            char_map[1] += 1;
        }
        if character == 'l' {
            char_map[2] += 1;
        }
        if character == 'o' {
            char_map[3] += 1;
        }
        if character == 'n' {
            char_map[4] += 1;
        }
    }

    char_map[2] /= 2;
    char_map[3] /= 2;

    let mut result = i32::MAX;
    for count in char_map {
        result = min(result, count);
    }
    result
}

#[cfg(test)]
mod test {
    use super::max_number_of_balloons;

    #[test]
    fn test_max_number_of_balloons() {
        let test_cases: Vec<(&str, i32)> =
            vec![("nlaebolko", 1), ("loonbalxballpoon", 2), ("leetcode", 0)];

        for (text, expected) in test_cases {
            let result = max_number_of_balloons(String::from(text));
            assert_eq!(result, expected);
        }
    }
}
