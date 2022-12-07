/// leetcode 021
///
/// [21.合并两个有序列表](https://leetcode.cn/problems/merge-two-sorted-lists/)
///
/// 迭代
/// 时间复杂度: O(m+n)
///  空间复杂度: O(1)
pub fn merge_two_lists(list1: Option<Box<Node>>, list2: Option<Box<Node>>) -> Option<Box<Node>> {
    if list1.is_none() {
        return list2;
    }

    if list2.is_none() {
        return list1;
    }

    let mut l1 = list1;
    let mut l2 = list2;

    // master为主链 （设置一个假节点）
    let mut master = Node::new(0);
    let mut ptr = &mut master;

    while let (Some(n1), Some(n2)) = (l1.as_ref(), l2.as_ref()) {
        if n1.val <= n2.val {
            ptr.next = l1;
            ptr = ptr.next.as_mut().unwrap();
            l1 = ptr.next.take();
        } else {
            ptr.next = l2;
            ptr = ptr.next.as_mut().unwrap();
            l2 = ptr.next.take();
        }
    }

    ptr.next = if l1.is_some() { l1 } else { l2 };
    master.next
}

/// 递归
/// 时间复杂度: O(m+n)
/// 空间复杂度: O(m+n)
pub fn merge_two_lists_v2(list1: Option<Box<Node>>, list2: Option<Box<Node>>) -> Option<Box<Node>> {
    match (list1, list2) {
        (Some(n1), None) => Some(n1),
        (None, Some(n2)) => Some(n2),
        (Some(n1), Some(n2)) =>
            if n1.val <= n2.val {
                Some(Box::new(Node { val: n1.val, next: merge_two_lists_v2(n1.next, Some(n2)) }))
            } else {
                Some(Box::new(Node { val: n2.val, next: merge_two_lists_v2(Some(n1), n2.next) }))
            }
        (None, None) => None,
    }
}


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

fn fmt_node(n: &Option<Box<Node>>, prefix: String) {
    if n.is_none() {
        println!();
        return;
    }

    let node = n.as_ref().unwrap();
    print!("{} {} ", prefix, node.val);


    fmt_node(&node.next, "->".to_string());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_merge_two_lists() {
        let mut l1 = Node::new(1);
        l1.add(2);
        l1.add(6);
        l1.add(9);
        l1.add(100);

        let mut l2 = Node::new(1);
        l2.add(4);
        l2.add(7);
        l2.add(8);
        l2.add(105);

        let result = merge_two_lists(Some(Box::new(l1)), Some(Box::new(l2)));
        if result.is_some() {
            fmt_node(&result, "test_merge_two_lists: ".to_string());
        }
    }

    #[test]
    fn test_merge_two_lists_v2() {
        let mut l1 = Node::new(1);
        l1.add(2);
        l1.add(6);
        l1.add(9);
        l1.add(100);

        let mut l2 = Node::new(1);
        l2.add(4);
        l2.add(7);
        l2.add(8);
        l2.add(105);

        let result = merge_two_lists_v2(Some(Box::new(l1)), Some(Box::new(l2)));
        if result.is_some() {
            fmt_node(&result, "test_merge_two_lists: ".to_string());
        }
    }
}


