pub fn largest_number(nums: Vec<i32>) -> String {
    let mut nums = nums.clone();
    nums.sort_by(|a, b| {
        let order_1 = format!("{}{}", a, b);
        let order_2 = format!("{}{}", b, a);
        order_2.cmp(&order_1)
    });

    let mut result = String::from("");
    for num in nums {
        if result == "" && num == 0 {
            continue;
        }
        result = format!("{}{}", result, num);
    }

    if result == "" {
        return String::from("0");
    }
    result
}

#[cfg(test)]
mod test {
    use super::largest_number;

    #[test]
    fn test_largest_number() {
        let test_cases: Vec<(Vec<i32>, &str)> = vec![
            (vec![10, 2], "210"),
            (vec![3, 30, 34, 5, 9], "9534330"),
            (vec![0, 0], "0"),
        ];

        for (nums, expected) in test_cases {
            println!("Test case {:?}", nums);

            let result = largest_number(nums);
            assert_eq!(result, expected);
        }
    }
}
