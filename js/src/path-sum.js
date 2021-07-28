/**
 * @param {TreeNode} root
 * @param {number} targetSum
 * @return {boolean}
 */

const tree = {
    val: 0,
    left: {
        val: 1,
        left: {
            val: 2,
            left: {
                val: 3,
            },
        },
        right: {
            val: 4,
        },
    },
    right: {
        val: 5,
    },
};

var hasPathSum = function (root, targetSum) {
    if (root) {
        return dfs(root, targetSum - root.val);
    }
    return false;
};

function dfs(node, rest) {
    if (!node.left && !node.right && rest === 0) {
        return true;
    }
    // if (node.left && dfs(node.left, rest - node.left.val)) {
    //     return true;
    // }
    // if (node.right && dfs(node.right, rest - node.right.val)) {
    //     return true;
    // }

    return dfs(node.left, rest - node.left.val) || dfs(node.right, rest - node.right.val)

    return false;
}

console.log(hasPathSum(tree, 5));
