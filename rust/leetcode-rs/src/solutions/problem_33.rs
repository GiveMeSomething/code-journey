#[allow(dead_code)]
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    recursive_search(&nums, target, 0, nums.len() - 1)
}

fn recursive_search(nums: &Vec<i32>, target: i32, start: usize, end: usize) -> i32 {
    println!("start = {}, end = {}", start, end);

    // Resolve two element problem
    if end - start == 1 {
        if nums[start] == target {
            return start as i32;
        }

        if nums[end] == target {
            return end as i32;
        }

        return -1;
    }

    if start == end {
        if nums[start] == target {
            return start as i32;
        }
        return -1;
    }

    let pivot = (start + end) / 2;
    let pivot_value = nums[pivot];

    if pivot_value == target {
        return pivot as i32;
    }

    if pivot == 0 {
        return recursive_search(nums, target, pivot + 1, end);
    }

    if pivot == nums.len() - 1 {
        return recursive_search(nums, target, start, pivot - 1);
    }

    if nums[start] <= pivot_value && target >= nums[start] && target <= pivot_value {
        return recursive_search(nums, target, start, pivot);
    }

    if pivot_value <= nums[end] && target >= pivot_value && target <= nums[end] {
        return recursive_search(nums, target, pivot, end);
    }

    if nums[start] > pivot_value {
        let left_result = recursive_search(nums, target, start, pivot);
        if left_result != -1 {
            return left_result;
        }
    }

    if pivot_value > nums[end] {
        let right_result = recursive_search(nums, target, pivot, end);
        if right_result != -1 {
            return right_result;
        }
    }

    -1
}
