use crate::dsa::max_heap::MaxHeap;

pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
    if stones.len() == 1 {
        return stones[0];
    }

    if stones.len() == 2 {
        return (stones[0] - stones[1]).abs();
    }

    let mut max_heap = MaxHeap::from(stones);

    while max_heap.array.len() > 1 {
        // Get 2 largest values
        let a = max_heap.pop_max();
        let b = max_heap.pop_max();

        let new_stone = (a - b).abs();
        if new_stone == 0 {
            continue;
        }
        max_heap.insert(new_stone);
    }

    if max_heap.array.len() == 1 {
        return max_heap.array[0];
    }

    return 0;
}

#[cfg(test)]
mod test {
    use super::last_stone_weight;

    #[test]
    fn test_last_stone_weight() {
        let test_cases: Vec<(Vec<i32>, i32)> = vec![
            (vec![2, 7, 4, 1, 8, 1], 1),
            (vec![1], 1),
            (vec![1, 100], 99),
        ];

        for (input, expected) in test_cases {
            let actual = last_stone_weight(input);
            assert_eq!(actual, expected);
        }
    }
}
