use std::rc::Rc;
use std::cell::RefCell;

pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

fn dps(root: Option<Rc<RefCell<TreeNode>>>, mut total: i32) -> i32 {
    if let Some(node) = root {
        total = node.borrow().val + total * 10;

        if node.borrow().left.is_none() && node.borrow().right.is_none() {
            return total
        }

        return dps(node.borrow().left.clone(), total) + dps(node.borrow().right.clone(), total);
    }  else {
        return 0;
    }
}

#[allow(dead_code)]
pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    dps(root, 0)
}

#[test]
fn internal() {
    let node = TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: None
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None
        })))
    };
    let n = sum_numbers(Some(Rc::new(RefCell::new(node))));
    assert_eq!(n, 25);
}
