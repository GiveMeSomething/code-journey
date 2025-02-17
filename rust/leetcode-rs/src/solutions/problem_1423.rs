use std::cmp::max;

pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
    // Calculate prefix sum for first k and last k
    let mut first_k: Vec<i32> = vec![0];
    let mut last_k: Vec<i32> = vec![0];

    let mut current_sum = 0;
    for i in 0..(k as usize) {
        current_sum += card_points[i];
        first_k.push(current_sum);
    }

    current_sum = 0;
    for i in (card_points.len() - (k as usize)..card_points.len()).rev() {
        current_sum += card_points[i];
        last_k.push(current_sum);
    }

    let mut max_point = 0;
    for i in 0..=k {
        let left = i as usize;
        let right = (k - i) as usize;

        max_point = max(max_point, first_k[left] + last_k[right]);
    }

    max_point
}

#[cfg(test)]
mod test {
    use super::max_score;

    #[test]
    fn test_max_score() {
        let test_cases: Vec<(Vec<i32>, i32, i32)> = vec![
            (vec![1, 2, 3, 4, 5, 6, 1], 3, 12),
            (vec![2, 2, 2], 2, 4),
            (vec![9, 7, 7, 9, 7, 7, 9], 7, 55),
        ];

        for (card_points, k, expected) in test_cases {
            println!("TEST CASE: {:?}", card_points);
            let result = max_score(card_points, k);
            assert_eq!(result, expected);
        }
    }
}
