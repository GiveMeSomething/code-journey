pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
    // Find the start, which is where the x is greater than the current and smaller than the next
    let mut start = arr.len() - 1;
    while x < arr[start] {
        if start == 0 {
            break;
        }
        start -= 1;
    }

    let mut result: Vec<i32> = vec![];
    let mut end = start + 1;

    println!("start = {} end = {}", start, end);

    while result.len() < k as usize && end < arr.len() {
        let start_distance = (arr[start] - x).abs();
        let end_distance = (arr[end] - x).abs();

        if start_distance < end_distance {
            result.push(arr[start]);
            if start == 0 {
                break;
            } else {
                start -= 1;
            }
        } else if start_distance > end_distance {
            result.push(arr[end]);
            end += 1;
        } else {
            if arr[start] < arr[end] {
                result.push(arr[start]);
                if start == 0 {
                    break;
                } else {
                    start -= 1;
                }
            } else {
                result.push(arr[end]);
                end += 1;
            }
        }
    }

    // This only run when start or end reach its limit
    while result.len() < k as usize && end < arr.len() {
        result.push(arr[end]);
        end += 1;
    }

    while result.len() < k as usize {
        result.push(arr[start]);
        if start == 0 {
            break;
        } else {
            start -= 1;
        }
    }

    result.sort();

    result
}

#[cfg(test)]
mod test {
    use super::find_closest_elements;

    #[test]
    fn test_find_closest_elements() {
        let test_cases: Vec<(Vec<i32>, i32, i32, Vec<i32>)> = vec![
            (vec![1, 2, 3, 4, 5], 4, 3, vec![1, 2, 3, 4]),
            (vec![1, 1, 2, 3, 4, 5], 4, -1, vec![1, 1, 2, 3]),
            (vec![1, 1, 1, 10, 10, 10], 1, 9, vec![10]),
            (vec![0, 0, 1, 2, 3, 3, 4, 7, 7, 8], 3, 5, vec![3, 3, 4]),
        ];

        for (nums, k, x, expexted) in test_cases {
            println!("TEST CASE {:?} k = {} x = {}", nums, k, x);
            let result = find_closest_elements(nums, k, x);
            println!("{:?}", result);

            assert_eq!(result.len(), expexted.len());

            for i in 0..result.len() {
                assert_eq!(result[i], expexted[i]);
            }
        }
    }
}
