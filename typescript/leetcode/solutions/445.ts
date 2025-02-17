// Definition for singly-linked list.
class ListNode {
    val: number;
    next: ListNode | null;
    constructor(val?: number, next?: ListNode | null) {
        this.val = val === undefined ? 0 : val;
        this.next = next === undefined ? null : next;
    }
}

function addTwoNumbers(
    l1: ListNode | null,
    l2: ListNode | null,
): ListNode | null {
    const l1Values = [];
    const l2Values = [];

    while (l1 != null) {
        l1Values.unshift(l1.val);
        l1 = l1.next;
    }

    while (l2 != null) {
        l2Values.unshift(l2.val);
        l2 = l2.next;
    }

    let extra = 0;
    for (let i = 0; i < Math.max(l1Values.length, l2Values.length); i++) {
        const sum: number = (l1Values[i] ?? 0) + (l2Values[i] ?? 0) + extra;
        extra = Math.floor(sum / 10);
        l1Values[i] = sum % 10;
    }

    if (extra > 0) {
        l1Values.push(extra);
    }

    const result = new ListNode(0, null);
    let resultCurrent = result;
    for (let i = l1Values.length - 1; i >= 0; i--) {
        resultCurrent.val = l1Values[i];
        if (i >= 1) {
            resultCurrent.next = new ListNode(0, null);
            resultCurrent = resultCurrent.next;
        }
    }

    return result;
}
