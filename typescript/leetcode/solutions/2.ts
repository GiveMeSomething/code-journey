/**
 * Definition for singly-linked list.
 * class ListNode {
 *     val: number
 *     next: ListNode | null
 *     constructor(val?: number, next?: ListNode | null) {
 *         this.val = (val===undefined ? 0 : val)
 *         this.next = (next===undefined ? null : next)
 *     }
 * }
 */

function addTwoNumbers(
    l1: ListNode | null,
    l2: ListNode | null,
): ListNode | null {
    let result = new ListNode(0, null);
    const start = result;

    let extra = 0;
    while (l1 != null && l2 != null) {
        const sum = l1.val + l2.val + extra;
        extra = Math.floor(sum / 10);
        result.val = sum % 10;

        l1 = l1.next;
        l2 = l2.next;
        if (l1 != null || l2 != null) {
            result.next = new ListNode(0, null);
            result = result.next;
        }
    }

    if (result == null) {
        return start;
    }

    while (l1 != null) {
        const sum = l1.val + extra;
        extra = Math.floor(sum / 10);
        result.val = sum % 10;
        l1 = l1.next;
        if (l1 != null) {
            result.next = new ListNode(0, null);
            result = result.next;
        }
    }

    while (l2 != null) {
        const sum = l2.val + extra;
        extra = Math.floor(sum / 10);
        result.val = sum % 10;
        l2 = l2.next;
        if (l2 != null) {
            result.next = new ListNode(0, null);
            result = result.next;
        }
    }

    if (extra > 0) {
        result.next = new ListNode(extra, null);
    }

    return start;
}

export const main_2 = () => {
    // Cannot test on local because ther are no ListNode class
};
