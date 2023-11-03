mod tree_node;

mod lc_0001_two_sum;
mod lc_0144_preorder_traversal;
mod lc_0206_reverse_list;

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;
    use crate::lc_0001_two_sum;
    use crate::tree_node::TreeNode;
    use crate::lc_0144_preorder_traversal;
    use crate::lc_0206_reverse_list;

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

        let result = lc_0144_preorder_traversal::preorder_traversal(Some(Rc::new(RefCell::new(root.clone()))));
        println!("fn: preorder_traversal, result: {:?}", result);

        let result_v2 = lc_0144_preorder_traversal::preorder_traversal_v2(Some(Rc::new(RefCell::new(root))));
        println!("fn: preorder_traversal_v2, result: {:?}", result_v2);
    }

    #[test]
    fn reverse_list() {
        // 创建链表 1 -> 2 -> 3 -> 4 -> 5
        let mut node_5 = lc_0206_reverse_list::ListNode { val: 5, next: None };
        let mut node_4 = lc_0206_reverse_list::ListNode { val: 4, next: Some(Box::new(node_5)) };
        let mut node_3 = lc_0206_reverse_list::ListNode { val: 3, next: Some(Box::new(node_4)) };
        let mut node_2 = lc_0206_reverse_list::ListNode { val: 2, next: Some(Box::new(node_3)) };
        let node_1 = lc_0206_reverse_list::ListNode { val: 1, next: Some(Box::new(node_2)) };
        // 反转链表
        let revered_head = lc_0206_reverse_list::reverse_list(Some(Box::new(node_1)));
        // 输出链表
        let mut current_node: &Option<Box<lc_0206_reverse_list::ListNode>> = &revered_head; // 借用
        while let Some(node) = current_node {
            println!("{}", node.val);
            current_node = &node.next;
        }
    }
}
