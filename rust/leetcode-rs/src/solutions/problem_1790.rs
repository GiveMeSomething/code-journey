pub fn are_almost_equal(s1: String, s2: String) -> bool {
    let mut diffs: Vec<usize> = vec![];
    for i in 0..s1.len() {
        if s1[i..i + 1] == s2[i..i + 1] {
            continue;
        }

        diffs.push(i);
    }

    if diffs.len() == 0 {
        return true;
    }

    if diffs.len() != 2 {
        return false;
    }

    return s1[diffs[0]..diffs[0] + 1] == s2[diffs[1]..diffs[1] + 1]
        && s1[diffs[1]..diffs[1] + 1] == s2[diffs[0]..diffs[0] + 1];
}

#[cfg(test)]
mod test {
    use super::are_almost_equal;

    #[test]
    fn test_are_almost_equal() {
        let test_cases: Vec<(&str, &str, bool)> = vec![
            ("bank", "kanb", true),
            ("attack", "defend", false),
            ("kelb", "kelb", true),
        ];

        for (s1, s2, expected) in test_cases {
            let result = are_almost_equal(String::from(s1), String::from(s2));
            assert_eq!(result, expected);
        }
    }
}
