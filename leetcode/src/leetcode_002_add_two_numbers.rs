/// leetcode 002
///
/// [2. 两数相加](https://leetcode.cn/problems/add-two-numbers/)
///
pub struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

impl Node {
    pub fn new(val: i32) -> Self {
        return Node { val, next: None };
    }
}

// 递归
pub fn add_two_numbers(n1: Option<Box<Node>>, n2: Option<Box<Node>>) -> Option<Box<Node>> {
    compute_current_node(n1, n2, 0)
}

fn compute_current_node(n1: Option<Box<Node>>, n2: Option<Box<Node>>, carry: i32) -> Option<Box<Node>> {
    if n1.is_none() && n2.is_none() && carry == 0 {
        return None;
    }

    let mut carry_next = 0;
    let n1_next = n1.and_then(|x| {
        carry_next = carry + x.val;
        x.next
    });

    let n2_next = n2.and_then(|x| {
        carry_next = carry + x.val;
        x.next
    });


    let node = Node {
        val: carry / 10,
        next: compute_current_node(n1_next, n2_next, carry_next),
    };

    Some(Box::new(node))
}


// loop
pub fn add_two_numbers_v2(n1: Option<Box<Node>>, n2: Option<Box<Node>>) -> Option<Box<Node>> {
    let mut result: Option<Box<Node>> = None;
    // (pre_node_val, n1_next, n2_next, carry_next)
    let mut t: (i32, Option<Box<Node>>, Option<Box<Node>>, i32) = (0, n1, n2, 0);

    // tail永远指向最后一个节点，当计算完一个节点之后，tail会指向一个None
    let mut tail: &mut Option<Box<Node>> = &mut result;

    loop {
        t = match t {
            (_, Some(l1), Some(l2), carry) => {
                let sum = l1.val + l2.val + carry;
                (sum % 10, l1.next, l2.next, sum / 10)
            }
            (_, Some(l1), None, carry) => {
                let sum = l1.val + carry;
                (sum % 10, l1.next, None, sum / 10)
            }
            (_, None, Some(l2), carry) => {
                let sum = l2.val + carry;
                (sum % 10, None, l2.next, sum / 10)
            }
            (_, None, None, 0) => {
                break;
            }
            (_, None, None, carry) => {
                (carry, None, None, 0)
            }
        };


        // tail永远指向最后一个节点，当计算完一个节点之后，tail会指向一个None
        *tail = Some(Box::new(Node::new(t.0)));
        tail = &mut tail.as_mut().unwrap().next;
    }

    result
}
