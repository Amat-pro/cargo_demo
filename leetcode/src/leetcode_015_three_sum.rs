/// leetcode 015
///
/// [15.三数之和](// https://leetcode.cn/problems/3sum/)
///
pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![];

    let mut nums = nums;
    nums.sort();

    let len = nums.len();
    if len < 3 {
        return result;
    }

    let mut i: usize;
    let mut j: usize;
    for k in 0..len {
        // -(num[k]) = num[i] + num[j]
        // 如果num[k] > 0, 则num[i] > 0, num[j] > 0， 达到不成立条件
        if nums[k] > 0 {
            return result;
        }

        if (k > 0) && (nums[k] == nums[k - 1]) {
            continue;
        }

        // 使用双指针
        i = k + 1;
        j = len - 1;
        while i < j {
            match nums[k] + nums[i] + nums[j] {
                0 => {
                    result.push(vec![nums[k], nums[i], nums[j]]);
                    i += 1;
                    j -= 1;

                    // 下面的情况应该对任意情况生效
                    while i < j && nums[i] == nums[i - 1] {
                        i += 1;
                    }
                    while i < j && nums[j] == nums[j + 1] {
                        j -= 1;
                    }
                }
                a if a > 0 => {
                    j -= 1;
                }
                _ => {
                    i += 1;
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_three_sum() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        println!("三数之和: {:?}", three_sum(nums));
    }
}
