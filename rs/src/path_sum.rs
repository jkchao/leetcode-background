
use std::rc::Rc;
use std::cell::RefCell;

pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

#[allow(dead_code)]
pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {

    if let Some(node) = root {
        return dps(node.clone(), target_sum - node.clone().borrow().val);
    };

    return false;
}

fn dps(root: Rc<RefCell<TreeNode>>, rest: i32) -> bool {

    let left_node = root.borrow().left.clone();
    let right_node = root.borrow().right.clone();

    if left_node.is_none() && right_node.is_none() && rest == 0 {
        return true;
    }

    if let Some(node) = left_node {
        if dps(node.clone(), rest - node.clone().borrow().val) {
            return true;
        }
    }

    if let Some(node) = right_node {
       if dps(node.clone(), rest - node.clone().borrow().val) {
           return true;
       }
    }

    return false;
}


#[test]
fn internal() {
    let node = TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        }))),
    };
    let n = has_path_sum(Some(Rc::new(RefCell::new(node))), 4);
    assert_eq!(n, true);
}
