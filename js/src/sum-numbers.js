// @ts-check

/**
 * Definition for a binary tree node.
 * function TreeNode(val, left, right) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.left = (left===undefined ? null : left)
 *     this.right = (right===undefined ? null : right)
 * }
 */

function sumNumbers(root) {
    return dfs(root, 0);
}

function dfs(root, total) {
    if (!root) {
        return 0;
    }
    total = total * 10 + root.val;

    if (root.left === null && root.right === null) {
        return total;
    }
    return dfs(root.left, total) + dfs(root.right, total);
}
