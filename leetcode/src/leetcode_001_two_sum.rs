use std::collections::HashMap;

/// leetcode 001
///
/// [1. 两数之和](https://leetcode.cn/problems/two-sum/)
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if nums.is_empty() {
        return vec![];
    }

    let mut value_indexs: HashMap<i32, usize> = HashMap::new();

    for (index, value) in nums.iter().enumerate() {
        let sub = target - value;
        if let Some(index2) = value_indexs.get(&sub) {
            return vec![index as i32, *index2 as i32];
        }
        value_indexs.insert(*value, index);
    }

    vec![]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![2, 7, 11, 15];
        assert_eq!(two_sum(nums, 9), vec![1, 0]);
    }
}