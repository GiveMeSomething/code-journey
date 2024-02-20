#[allow(dead_code)]
pub fn find_min(nums: Vec<i32>) -> i32 {
    let start = 0;
    let end = nums.len() - 1;

    recursive_find_min(&nums, start, end)
}

fn recursive_find_min(nums: &Vec<i32>, start: usize, end: usize) -> i32 {
    if start >= end {
        return *nums.get(start).unwrap();
    }

    let pivot: usize = (start + end) / 2;
    let pivot_value = *nums.get(pivot).unwrap();

    if pivot == 0 {
        if pivot_value < nums[pivot + 1] {
            return pivot_value;
        }

        return recursive_find_min(nums, pivot + 1, end);
    }

    if pivot == nums.len() - 1 {
        if pivot_value < nums[pivot - 1] {
            return pivot_value;
        }

        return recursive_find_min(nums, start, pivot - 1);
    }

    if pivot_value < nums[pivot + 1] && pivot_value < nums[pivot - 1] {
        return pivot_value;
    }

    let left_min = recursive_find_min(nums, start, pivot - 1);
    let right_min = recursive_find_min(nums, pivot + 1, end);
    if left_min < right_min {
        left_min
    } else {
        right_min
    }
}
