// Brute-force
// pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
//     let mut result = 0;
//     for i in 0..nums.len() {
//         let mut current_product = 1;

//         for j in i..nums.len() {
//             current_product *= nums[j];
//             if current_product < k {
//                 result += 1;
//             } else {
//                 break;
//             }
//         }
//     }

//     result
// }

pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
    if k == 0 {
        return 0;
    }

    let mut start = 0;
    let mut end = 0;

    let mut result = 0;
    let mut current_product = 1;

    while end < nums.len() {
        current_product *= nums[end];

        while start <= end && current_product >= k {
            current_product /= nums[start];
            start += 1;
        }

        result += (end - start + 1) as i32;
        end += 1;
    }

    result
}

#[cfg(test)]
mod test {
    use super::num_subarray_product_less_than_k;

    #[test]
    fn test_num_subarray_product_less_than_k() {
        let test_cases: Vec<(Vec<i32>, i32, i32)> = vec![
            (vec![10, 5, 2, 6], 100, 8),
            (vec![1, 2, 3], 0, 0),
            (vec![3, 3, 6, 2, 10, 10, 9, 3], 19, 11),
            (vec![10, 9, 10, 4, 3, 8, 3, 3, 6, 2, 10, 10, 9, 3], 19, 18),
            (vec![10, 9, 10, 4, 3, 8], 19, 7),
        ];

        for (nums, k, expected) in test_cases {
            println!("TEST CASE {:?} k = {}", nums, k);

            let result = num_subarray_product_less_than_k(nums, k);
            assert_eq!(result, expected);
        }
    }
}
