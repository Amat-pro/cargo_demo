mod tree_node;

mod lc_0001_two_sum;
mod lc_0144_preorder_traversal;

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;
    use crate::lc_0001_two_sum;
    use crate::tree_node::TreeNode;
    use crate::lc_0144_preorder_traversal;

    #[test]
    fn two_sum() {
        let r = lc_0001_two_sum::two_sum(vec![2, 7, 11, 15], 9);
        println!("fn: two_sum, result: {:?}", r);
    }

    #[test]
    fn preorder_traversal() {
        let right_right = TreeNode::new(16);
        let right = TreeNode::new_with_sub(13, None,
                                           Some(Rc::new(RefCell::new(right_right))));
        let left = TreeNode::new(12);
        let root = TreeNode::new_with_sub(1,
                                          Some(Rc::new(RefCell::new(left))),
                                          Some(Rc::new(RefCell::new(right))));

        let result = lc_0144_preorder_traversal::preorder_traversal(Some(Rc::new(RefCell::new(root))));
        println!("fn: preorder_traversal, result: {:?}", result);
    }
}
