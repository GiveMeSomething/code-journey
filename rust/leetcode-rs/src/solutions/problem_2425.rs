// Brute-force
// pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
//     let mut result = 0;

//     for i in 0..nums1.len() {
//         for j in 0..nums2.len() {
//             result ^= nums1[i] ^ nums2[j];
//         }
//     }
//     result
// }

// Optimal way
// Reasoning: Because we are XOR-ing pair between nums1 and nums2
// Therefore, each number in nums1 will appear nums2.len() times, and vice versa
// And in XOR, a^a = 0
// With that properties, we need to do XOR one time for each number if the opposing array doesn't have even length
// And if array length is even, we XOR with 0 (which mean we can skip the calculation)
pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut result = 0;

    let nums1_even = nums1.len() % 2 == 0;
    let nums2_even = nums2.len() % 2 == 0;
    for num in nums1 {
        if nums2_even {
            break;
        }
        result ^= num;
    }
    for num in nums2 {
        if nums1_even {
            break;
        }
        result ^= num;
    }
    result
}

#[cfg(test)]
mod test {
    use super::xor_all_nums;

    #[test]
    fn test_xor_all_nums() {
        let test_cases: Vec<(Vec<i32>, Vec<i32>, i32)> = vec![
            (vec![2, 1, 3], vec![10, 2, 5, 0], 13),
            (vec![1, 2], vec![3, 4], 0),
        ];

        for (nums1, nums2, expected) in test_cases {
            let result = xor_all_nums(nums1, nums2);
            assert_eq!(result, expected);
        }
    }
}
