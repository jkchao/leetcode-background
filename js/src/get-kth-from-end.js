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
 var getKthFromEnd = function(head, k) {
    const map = new Map()
    let i = 1;

    let node = head;
    map.set(i, node);

    while(node.next) {
        i++;
        map.set(i, node.next);
        node = node.next;
    }

    return map.get(map.size - k + 1);
};