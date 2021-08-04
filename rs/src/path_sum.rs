
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

        if node.borrow().left.is_none() && node.borrow().right.is_none() && target_sum == node.borrow().val {
            return true;
        }

        return has_path_sum(node.borrow().left.clone(), target_sum - node.borrow().val) || has_path_sum(node.borrow().right.clone(), target_sum - node.borrow().val)
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
