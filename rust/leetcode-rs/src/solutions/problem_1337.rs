use std::collections::BinaryHeap;

pub struct SoliderRow {
    count: i32,
    index: usize,
}

impl PartialEq for SoliderRow {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count && self.index == other.index
    }
}

impl Eq for SoliderRow {}

// Custom order to make min_heap
impl PartialOrd for SoliderRow {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.count == other.count {
            return other.index.partial_cmp(&self.index);
        }

        other.count.partial_cmp(&self.count)
    }
}

impl Ord for SoliderRow {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.count == other.count {
            return other.index.cmp(&self.index);
        }
        other.count.cmp(&self.count)
    }
}

impl From<Vec<i32>> for SoliderRow {
    fn from(values: Vec<i32>) -> Self {
        let mut count = 0;
        for value in values {
            if value == 1 {
                count += 1;
            }
        }
        return SoliderRow { count, index: 0 };
    }
}

pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let mut rows: Vec<SoliderRow> = vec![];
    for i in 0..mat.len() {
        let mut row = SoliderRow::from(mat[i].to_owned());
        row.index = i;
        rows.push(row);
    }

    let mut min_heap: BinaryHeap<SoliderRow> = BinaryHeap::new();
    for row in rows {
        min_heap.push(row);
    }

    let mut result: Vec<i32> = vec![];
    for _ in 0..k {
        let min = min_heap.pop().unwrap();
        result.push(min.index as i32);
    }
    return result;
}

#[cfg(test)]
mod test {
    use super::k_weakest_rows;

    #[test]
    fn test_k_weakest_rows() {
        let test_cases: Vec<(Vec<Vec<i32>>, i32, Vec<i32>)> = vec![(
            vec![
                vec![1, 1, 0, 0, 0],
                vec![1, 1, 1, 1, 0],
                vec![1, 0, 0, 0, 0],
                vec![1, 1, 0, 0, 0],
                vec![1, 1, 1, 1, 1],
            ],
            3,
            vec![2, 0, 3],
        )];

        for (input, k, expected) in test_cases {
            let actual = k_weakest_rows(input, k);
            for i in 0..actual.len() {
                assert_eq!(actual[i], expected[i]);
            }
        }
    }
}
