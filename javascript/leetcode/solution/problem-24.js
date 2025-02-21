/**
 * Definition for singly-linked list.
 * function ListNode(val, next) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.next = (next===undefined ? null : next)
 * }
 */
/**
 * @param {ListNode} head
 * @return {ListNode}
 */
var swapPairs = function (head) {
  if (head == null) {
    return head;
  }

  let currentNode = head;
  let previous = null;
  while (currentNode.next != null) {
    let temp = currentNode.next;
    currentNode.next = currentNode.next.next;
    temp.next = currentNode;

    if (currentNode === head) {
      head = temp;
    }

    if (previous != null) {
      previous.next = temp;
    }

    previous = currentNode;
    currentNode = currentNode.next;
    if (currentNode == null) {
      return head;
    }
  }
  return head;
};
