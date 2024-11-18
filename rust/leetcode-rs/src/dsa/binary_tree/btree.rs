pub struct BTree;

impl BTree {
    pub fn parent(i: usize) -> usize {
        if i == 0 {
            return 0;
        }
        return (i - 1) / 2;
    }

    pub fn left(i: usize) -> usize {
        return 2 * i + 1;
    }

    pub fn right(i: usize) -> usize {
        return 2 * i + 2;
    }
}
