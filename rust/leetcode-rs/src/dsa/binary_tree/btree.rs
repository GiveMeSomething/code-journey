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

/* Implementation of binary tree */

pub struct BinaryTree {
    pub root: TreeNode,
}

pub struct TreeNode {
    value: isize,
    parent: TreeNode,
}

impl BinaryTree {
    pub fn traverse(root: &TreeNode) {
        if let Some(node) = &root.left {
            BinaryTree::traverse(node);
        }
        print!("{} ", root.value);
        if let Some(node) = &root.right {
            BinaryTree::traverse(node);
        }
    }

    pub fn new(value: isize) -> Self {
        let root = TreeNode {
            value,
            left: None,
            right: None,
        };

        BinaryTree { root }
    }
}
