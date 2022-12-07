/// leetcode 019
///
/// [19.删除链表的倒数第n个节点](https://leetcode.cn/problems/remove-nth-node-from-end-of-list/)
///
/// 双指针(快慢指针)
pub fn remove_nth_from_end(head: Option<Box<Node>>, n: i32) -> Option<Box<Node>> {
    let mut c0 = Some(Box::new(Node {
        val: 0,
        next: head,
    }));

    let mut second_pt: &mut Option<Box<Node>> = &mut c0;
    let mut first_pt: &Option<Box<Node>> = &second_pt.clone();

    // first 移动n+1次
    for _ in 0..=n {
        if first_pt.is_none() {
            return None;
        }
        first_pt = &first_pt.as_ref().unwrap().next;
    }

    while first_pt.is_some() {
        first_pt = &first_pt.as_ref().unwrap().next;
        second_pt = &mut second_pt.as_mut().unwrap().next;
    }

    // first_pt指向末尾的None节点
    // 并且second指向待移除节点的前一个节点
    let removed = &mut second_pt.as_mut().unwrap().next;
    second_pt.as_mut().unwrap().next = removed.as_mut().unwrap().next.take();

    return c0.unwrap().next;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

impl Node {
    pub fn new(val: i32) -> Self {
        return Node { val, next: None };
    }

    pub fn add(&mut self, val: i32) {
        if self.next.is_none() {
            self.next = Some(Box::new(Self::new(val)));
            return;
        }

        self.next.as_mut().unwrap().add(val);
        return;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn fmt_node(n: &Option<Box<Node>>, prefix: String) {
        if n.is_none() {
            println!();
            return;
        }

        let node = n.as_ref().unwrap();
        print!("{} {} ", prefix, node.val);


        fmt_node(&node.next, "->".to_string());
    }

    #[test]
    fn test_remove_nth_from_end() {
        let head1 = Node::new(1);
        assert_eq!(None, remove_nth_from_end(Some(Box::new(head1)), 1));

        let mut head2 = Node::new(1);
        head2.add(2);
        head2.add(3);
        head2.add(4);
        head2.add(5);
        let result = remove_nth_from_end(Some(Box::new(head2)), 2);
        // node:  1 -> 2 -> 3 -> 5
        fmt_node(&result, "node: ".to_string());
    }
}