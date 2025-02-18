pub fn decode_string(s: String) -> String {
    let chars: Vec<char> = s.chars().collect();

    let mut multiplier_stack: Vec<usize> = vec![];
    let mut char_stack: Vec<String> = vec![];

    let mut current_multiplier = 0;
    for char in chars {
        if let Some(value) = char.to_digit(10) {
            current_multiplier = current_multiplier * 10 + value as usize;
            continue;
        }

        char_stack.push(char.to_string());

        if char == '[' {
            multiplier_stack.push(current_multiplier);
            current_multiplier = 0;
        }

        if char == ']' {
            let mut temp = String::from("");
            while let Some(last_char) = char_stack.pop() {
                if last_char == "[" {
                    break;
                }

                if last_char == "]" {
                    continue;
                }

                temp = format!("{}{}", last_char, temp);
            }

            char_stack.push(temp.repeat(multiplier_stack.pop().unwrap()));
        }
    }

    char_stack.join("")
}

#[cfg(test)]
mod test {
    use super::decode_string;

    #[test]
    fn test_decode_string() {
        let test_cases: Vec<(&str, &str)> = vec![
            ("3[a]2[bc]", "aaabcbc"),
            ("3[a2[c]]", "accaccacc"),
            ("2[abc]3[cd]ef", "abcabccdcdcdef"),
        ];

        for (input, expected) in test_cases {
            let result = decode_string(String::from(input));
            assert_eq!(result, String::from(expected));
        }
    }
}
