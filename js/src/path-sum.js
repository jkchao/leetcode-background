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

var hasPathSum = function (node, targetSum) {
    if (!node) {
        return false;
    }

    if (!node.left && !node.right && targetSum === node.val) {
        return true;
    }

    return hasPathSum(node.left, targetSum - node.val) || hasPathSum(node.right, targetSum - node.val);
};
console.log(hasPathSum(tree, 5));
