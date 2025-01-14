pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut mutable_nums = nums.clone();
    mutable_nums.sort();

    let mut result: Vec<Vec<i32>> = vec![];

    let mut i = 0;
    while i < mutable_nums.len() {
        let mut start = i + 1;
        let mut end = mutable_nums.len() - 1;
        let target = -mutable_nums[i];

        while start < end {
            let start_num = mutable_nums[start];
            let end_num = mutable_nums[end];
            let current_sum = start_num + end_num;

            if current_sum > target {
                end -= 1;
            } else if current_sum < target {
                start += 1;
            } else {
                result.push(vec![
                    mutable_nums[i],
                    mutable_nums[start],
                    mutable_nums[end],
                ]);

                while start < end && mutable_nums[start] == start_num {
                    start += 1;
                }
                while start < end && mutable_nums[end] == end_num {
                    end -= 1;
                }
            }
        }

        while i + 1 < nums.len() && mutable_nums[i] == mutable_nums[i + 1] {
            i += 1;
        }

        i += 1;
    }

    result
}

#[cfg(test)]
mod test {
    use super::three_sum;

    #[test]
    fn test_three_sum() {
        let test_cases: Vec<(Vec<i32>, Vec<Vec<i32>>)> = vec![(
            vec![-1, 0, 1, 2, -1, -4],
            vec![vec![-1, -1, 2], vec![-1, 0, 1]],
        )];

        for (nums, expected) in test_cases {
            let result = three_sum(nums);
            println!("{:?}", result);

            assert_eq!(result.len(), expected.len());

            for i in 0..result.len() {
                for j in 0..result[i].len() {
                    assert_eq!(result[i][j], expected[i][j]);
                }
            }
        }
    }
}
