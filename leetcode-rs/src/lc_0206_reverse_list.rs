#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

    // 设置一个虚拟头节点 ()
    let mut virtual_head = None;
    // 当前节点
    let mut current_node = head.clone();
    // 下一个节点
    let mut next_node: Option<Box<ListNode>>;

    while let Some(mut current_n) = current_node {
        next_node = current_n.next.take();
        current_n.next = virtual_head;
        virtual_head = Some(current_n);
        current_node = next_node;
    }

    virtual_head
}