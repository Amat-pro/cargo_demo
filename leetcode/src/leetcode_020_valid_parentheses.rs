/// leetcode 020
///
/// [20.有效的括号](https://leetcode.cn/problems/valid-parentheses/)
///
/// 栈
pub fn is_valid(s: String) -> bool {
    if s.len() % 2 != 0 {
        return false;
    }

    let mut stack = vec![];
    for c in s.chars() {
        match c {
            '(' | '{' | '[' => stack.push(c),
            ')' => { if stack.pop().unwrap() != '(' { return false; } }
            '}' => { if stack.pop().unwrap() != '{' { return false; } }
            ']' => { if stack.pop().unwrap() != ']' { return false; } }
            _ => {}
        }
    }

    true
}