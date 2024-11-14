use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

pub struct MatrixNode {
    value: i32,
    i: usize,
    j: usize,
}

impl PartialEq for MatrixNode {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl Eq for MatrixNode {}

impl PartialOrd for MatrixNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for MatrixNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let mut min_heap: BinaryHeap<Reverse<MatrixNode>> = BinaryHeap::new();
    min_heap.push(Reverse(MatrixNode {
        i: 0,
        j: 0,
        value: matrix[0][0],
    }));

    let i_limit = matrix.len();
    let j_limit = matrix[0].len();

    let limit = usize::try_from(k).expect("k is not compatible with usize");
    let mut k_count = 0;
    let mut min = 0;
    while k_count < limit {
        let min_node = min_heap.pop().unwrap().0;
        min = min_node.value;
        k_count += 1;

        if min_node.i + 1 < i_limit && !visited.contains(&(min_node.i + 1, min_node.j)) {
            min_heap.push(Reverse(MatrixNode {
                i: min_node.i + 1,
                j: min_node.j,
                value: matrix[min_node.i + 1][min_node.j],
            }));
            visited.insert((min_node.i + 1, min_node.j));
        }

        if min_node.j + 1 < j_limit && !visited.contains(&(min_node.i, min_node.j + 1)) {
            min_heap.push(Reverse(MatrixNode {
                i: min_node.i,
                j: min_node.j + 1,
                value: matrix[min_node.i][min_node.j + 1],
            }));
            visited.insert((min_node.i, min_node.j + 1));
        }
    }

    min
}

#[cfg(test)]
mod test {
    use super::kth_smallest;

    #[test]
    fn test_kth_smallest() {
        let test_cases: Vec<(Vec<Vec<i32>>, i32, i32)> = vec![
            (
                vec![vec![1, 5, 9], vec![10, 11, 13], vec![12, 13, 15]],
                8,
                13,
            ),
            // (vec![vec![-5]], 1, -5),
            (
                vec![vec![1, 100, 1000], vec![2, 200, 2000], vec![3, 300, 3000]],
                8,
                2000,
            ),
        ];

        for (matrix, k, expected) in test_cases {
            let actual = kth_smallest(matrix, k);
            assert_eq!(actual, expected);
        }
    }
}
