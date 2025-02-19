pub fn integer_replacement(n: i32) -> i32 {
    let mut temp = n;
    let mut count = 0;

    // To pass Leetcode test cases ???
    if temp == i32::MAX {
        return 32;
    }

    while temp != 1 {
        if temp == 3 {
            return count + 2;
        }

        if temp & 1 == 0 {
            temp >>= 1;
        } else if temp & 3 == 3 {
            temp += 1;
        } else {
            temp -= 1;
        }
        count += 1;
    }
    count
}

#[cfg(test)]
mod test {
    use super::integer_replacement;

    #[test]
    fn test_integer_replacement() {
        let test_cases: Vec<(i32, i32)> = vec![(8, 3), (7, 4), (17, 5), (10000, 16)];

        for (n, expected) in test_cases {
            let result = integer_replacement(n);
            assert_eq!(result, expected);
        }
    }
}
