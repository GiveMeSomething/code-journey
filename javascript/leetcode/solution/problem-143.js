/**
 * Definition for singly-linked list.
 * function ListNode(val, next) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.next = (next===undefined ? null : next)
 * }
 */
/**
 * @param {ListNode} head
 * @return {void} Do not return anything, modify head in-place instead.
 */
var reorderList = function (head) {
  let nodeStack = [];
  let listLength = 0;

  let currentNode = head;
  while (currentNode != null) {
    nodeStack.push(currentNode);
    listLength++;
    currentNode = currentNode.next;
  }

  currentNode = head;
  let temp = currentNode.next;

  let count = 1;
  while (true) {
    if (count % 2 == 1) {
      const node = nodeStack.pop();
      currentNode.next = node;

      listLength--;
      if (listLength === 0) {
        node.next = null;
        break;
      }
    } else {
      currentNode.next = temp;
      temp = temp.next;
      listLength--;
      if (listLength === 0) {
        temp.next = null;
        break;
      }
    }
    count += 1;
    currentNode = currentNode.next;
  }
};
