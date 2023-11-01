use std::collections::HashMap;

/// 0001 two_sum 两数之和 - 梦开始的地方
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if nums.len() == 0 {
        return vec![];
    }
    // map-> key: item value: index
    let mut map = HashMap::<i32, usize>::new();
    let mut sub: i32;
    let mut val_result: Option<&usize>;
    for i in 0..nums.len() {
        sub = target - nums[i];
        val_result = map.get(&sub);
        match val_result {
            Some(idx) => {
                return vec![idx.clone() as i32, i as i32];
            }
            None => {
                map.insert(nums[i], i);
            }
        }
    }

    vec![]
}