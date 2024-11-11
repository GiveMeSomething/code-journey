pub fn is_ugly(n: i32) -> bool {
    if n == 0 {
        return false;
    }

    if n == 1 {
        return true;
    }

    let mut result = n;
    while result > 1 {
        if result % 2 == 0 {
            result /= 2;
            continue;
        }
        if result % 3 == 0 {
            result /= 3;
            continue;
        }
        if result % 5 == 0 {
            result /= 5;
            continue;
        }
        break;
    }
    return result == 1;
}

#[cfg(test)]
mod test {
    use super::is_ugly;

    #[test]
    fn test_is_ugly() {
        let test_cases: Vec<(i32, bool)> = vec![(6, true), (1, true), (14, false), (0, false)];

        for (n, expected) in test_cases {
            let actual = is_ugly(n);
            assert_eq!(actual, expected);
        }
    }
}
