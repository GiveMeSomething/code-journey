pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut start = 0;
    let mut end = nums.len();

    while start < end {
        let pivot = (start + end) / 2;
        let pivot_value = nums[pivot];

        if pivot_value == target {
            return pivot as i32;
        }

        if target > pivot_value {
            start = pivot + 1;
        }

        if target < pivot_value {
            end = pivot;
        }
    }

    return start as i32;
}
