use std::cmp::{max, min};

// Brute-force way, will result in time limit exceed
// pub fn max_area(height: Vec<i32>) -> i32 {
//     let mut result = 0;
//     let n = height.len();

//     for i in 0..n {
//         for j in i + 1..n {
//             result = max(
//                 result,
//                 min(height[i], height[j]) * i32::try_from(j - i).unwrap(),
//             );
//         }
//     }
//     result
// }

// Better brute-force with lots of skip cases
// pub fn max_area(height: Vec<i32>) -> i32 {
//     let mut result = 0;
//     let n = height.len();

//     let mut current_height = -1;
//     for i in 0..n {
//         current_height = max(current_height, height[i]);
//         if current_height > height[i] {
//             continue;
//         }

//         for j in i + 1..n {
//             result = max(
//                 result,
//                 min(height[i], height[j]) * i32::try_from(j - i).unwrap(),
//             );
//         }
//     }
//     result
// }

pub fn max_area(height: Vec<i32>) -> i32 {
    let n = height.len();
    let mut start = 0;
    let mut end = n - 1;
    let mut result = 0;

    while start < end {
        let container_height = min(height[start], height[end]);
        result = max(result, container_height * (end - start) as i32);

        if container_height == height[start] {
            start += 1;
        } else {
            end -= 1;
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::max_area;

    #[test]
    fn test_max_area() {
        let test_cases: Vec<(Vec<i32>, i32)> =
            vec![(vec![1, 8, 6, 2, 5, 4, 8, 3, 7], 49), (vec![1, 1], 1)];

        for (height, expected) in test_cases {
            let result = max_area(height);
            assert_eq!(result, expected);
        }
    }
}
