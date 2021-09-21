/**
 * Definition for singly-linked list.
 * function ListNode(val) {
 *     this.val = val;
 *     this.next = null;
 * }
 */

/**
 * @param {ListNode} head
 * @param {number} k
 * @return {ListNode}
 */

// 解法一：快慢指针
var getKthFromEnd1 = function(head, k) {
    let f = head;
    let s = head;

    while (k > 0) {
        f = head.next;
        k--;
    }

    while(f) {
        f = f.next;
        s = s.next;
    }

    return s;
};

// 解法二，转化为 map，取数
var getKthFromEnd2 = function(head, k) {
    const map = new Map();
    let i = 1;


    while (head) {
        map.set(i, head);

        head = head.next;
        i++;
    }

    return map.get(map.size - k + 1);
};