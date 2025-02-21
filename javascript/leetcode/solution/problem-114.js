/**
 * Definition for a binary tree node.
 * function TreeNode(val, left, right) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.left = (left===undefined ? null : left)
 *     this.right = (right===undefined ? null : right)
 * }
 */
/**
 * @param {TreeNode} root
 * @return {void} Do not return anything, modify root in-place instead.
 */
var flatten = function (root) {
  if (root == null) {
    return;
  }

  const nodeStack = [root];
  let currentNode = null;

  while (nodeStack.length > 0) {
    const node = nodeStack.pop();
    if (node.right) {
      nodeStack.push(node.right);
    }
    if (node.left) {
      nodeStack.push(node.left);
    }

    if (currentNode == null) {
      currentNode = node;
    } else {
      currentNode.left = null;
      currentNode.right = node;
      currentNode = currentNode.right;
    }
  }
};
