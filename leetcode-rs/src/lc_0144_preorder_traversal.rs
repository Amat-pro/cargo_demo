use std::cell::RefCell;
use std::rc::Rc;
use crate::tree_node::TreeNode;

/// 0144 preorder_traversal 二叉树的前序遍历
pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    if root.is_none() {
        return vec![];
    }
    let mut result = vec![];

    recursion(&mut result, root);
    result
}

fn recursion(result: &mut Vec<i32>, node: Option<Rc<RefCell<TreeNode>>>) {
    // 1. 递归函数
    // 2. 递归条件
    // 3. 终止条件

    match node {
        Some(n) => {
            // current
            result.push(n.borrow().val);
            // left
            recursion(result, n.borrow().left.clone());
            // right
            recursion(result, n.borrow().right.clone())
        }
        None => {
            return;
        }
    }
}