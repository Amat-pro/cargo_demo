use std::collections::HashMap;

/// leetcode 017
///
/// [17.电话号码的字母组合](https://leetcode.cn/problems/letter-combinations-of-a-phone-number/)
///
pub fn letter_combinations(digest: String) -> Vec<String> {
    if digest.len() == 0 {
        return vec![];
    }

    let mut phone_map = HashMap::new();
    phone_map.insert('2', "abc".to_string());
    phone_map.insert('3', "def".to_string());
    phone_map.insert('4', "ghi".to_string());
    phone_map.insert('5', "jkl".to_string());
    phone_map.insert('6', "mno".to_string());
    phone_map.insert('7', "pqrs".to_string());
    phone_map.insert('8', "tuv".to_string());
    phone_map.insert('9', "wxyz".to_string());

    let mut ans = Vec::new();
    back_trace(0, &digest, &phone_map, &mut String::new(), &mut ans);
    ans
}

fn back_trace(idx: usize,
              digest: &String,
              map: &HashMap<char, String>,
              path: &mut String,
              ans: &mut Vec<String>,
) {
    if digest.len() == idx {
        ans.push(path.clone());
        return;
    }

    let cc = digest.chars().nth(idx).unwrap();
    for c in map.get(&cc).unwrap().chars() {
        path.push(c);
        back_trace(idx + 1, &digest, &map, path, ans);
        // 调用path.pop() 在每一次调用结束后，能够利用递归清空path
        path.pop();
    }
}



