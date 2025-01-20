use std::{cmp::max, collections::HashMap};

pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut result = 0;

    let base_array: Vec<i32>;
    let target_array: Vec<i32>;
    if nums1.len() <= nums2.len() {
        base_array = nums1;
        target_array = nums2;
    } else {
        base_array = nums2;
        target_array = nums1;
    }

    let mut index_map: HashMap<i32, Vec<usize>> = HashMap::new();
    for i in 0..target_array.len() {
        let entry = index_map.entry(target_array[i]).or_insert(vec![]);
        entry.push(i);
    }

    for i in 0..base_array.len() {
        // Everything from this point will be less than result, therefore we can skip those
        if base_array.len() - i < result {
            break;
        }

        if !index_map.contains_key(&base_array[i]) {
            continue;
        }

        let index_list = index_map.get(&base_array[i]).unwrap();
        for j in index_list {
            if base_array[i] != target_array[*j] {
                continue;
            }

            let mut start = 0;
            while i + start < base_array.len()
                && j + start < target_array.len()
                && base_array[i + start] == target_array[j + start]
            {
                start += 1;
            }

            result = max(result, start);
        }
    }

    result as i32
}

#[cfg(test)]
mod test {
    use super::find_length;

    #[test]
    fn test_find_length() {
        let test_cases: Vec<(Vec<i32>, Vec<i32>, i32)> = vec![
            (vec![1, 2, 3, 2, 1], vec![3, 2, 1, 4, 7], 3),
            (vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0], 5),
        ];

        for (nums1, nums2, expected) in test_cases {
            println!("TEST CASE nums1 [{:?}], nums2 [{:?}]", nums1, nums2);
            let result = find_length(nums1, nums2);
            assert_eq!(result, expected);
        }
    }
}
