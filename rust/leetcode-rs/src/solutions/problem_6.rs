pub fn convert_zigzag(s: String, num_rows: i32) -> String {
    if s.len() == 1 || num_rows == 1 {
        return s;
    }

    let max_distance = usize::try_from((num_rows - 1) * 2).unwrap();
    let mut distance_list: Vec<usize> = vec![];

    let mut current_distance = max_distance;
    while current_distance > 0 {
        distance_list.push(current_distance);
        current_distance -= 2;
    }
    distance_list.push(max_distance);

    let mut result = String::from("");
    for i in 0..usize::try_from(num_rows).unwrap() {
        let mut start = i as usize;
        while start < s.len() {
            result = format!("{}{}", result, &s[start..start + 1]);

            if distance_list[i] != max_distance && start + distance_list[i] < s.len() {
                result = format!(
                    "{}{}",
                    result,
                    &s[start + distance_list[i]..start + distance_list[i] + 1]
                );
            }

            start += max_distance;
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::convert_zigzag;

    #[test]
    fn test_convert_zigzag() {
        let test_cases: Vec<(&str, i32, &str)> = vec![
            ("ABCDEFGH", 3, "AEBDFHCG"),
            ("ABCDEFGH", 4, "AGBFHCED"),
            // ("PAYPALISHIRING", 3, "PAHNAPLSIIGYIR"),
            // ("PAYPALISHIRING", 4, "PINALSIGYAHRPI"),
            // ("A", 1, "A"),
        ];

        for (s, rows, expected) in test_cases {
            let result = convert_zigzag(String::from(s), rows);
            assert_eq!(result, String::from(expected));
        }
    }
}
