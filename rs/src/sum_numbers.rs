
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>
}

fn dps(root: Option<Box<TreeNode>>, total: i32) -> i32 {
    if let Some(node) = root {
        total = 10 * node.val + total;

        if node.left.is_none() {
            if node.right.is_none() {
                return total
            }
        }


        return dps(node.left, total) + dps(node.right, total);
    }  else {
        return 0;
    }


}

pub fn sum_numbers(root: TreeNode) -> i32 {
    dps(Some(Box::new(root)), 0)
}
