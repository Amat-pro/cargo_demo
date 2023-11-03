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
    // 1. 终止条件
    // 2. 单层处理逻辑
    // 3. 收集结果集

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

pub fn preorder_traversal_v2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    // 1. 终止条件
    // 2. 单层处理逻辑
    // 3. 收集结果集
    let mut result: Vec<i32> = vec![];
    return match root {
        Some(node) => {
            // 中
            result.push(node.borrow().val.clone());
            // 左
            let mut left_result = preorder_traversal_v2(node.borrow().left.clone());
            if !left_result.is_empty() {
                result.append(&mut left_result);
            }
            // 右
            let mut right_result = preorder_traversal_v2(node.borrow().right.clone());
            if !right_result.is_empty() {
                result.append(&mut right_result);
            }
            result
        }
        None => {
            // 终止条件
            result
        }
    };
}