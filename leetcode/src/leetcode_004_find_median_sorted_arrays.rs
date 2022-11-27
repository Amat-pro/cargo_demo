use std::cmp::{max, min};

/// leetcode 003
///
/// [3.寻找两个正序数组的中位数](https://leetcode.cn/problems/median-of-two-sorted-arrays/)
///
///
// 二分查找
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let nums1_len = nums1.len();
    let nums2_len = nums2.len();

    if nums1_len == 0 && nums2_len == 0 {
        panic!("wrong input");
    }

    let sum = nums1_len + nums2_len;
    return if sum % 2 == 0 {
        // 偶数
        let k1 = sum / 2;
        let k2 = k1 + 1;

        let num_k1 = get_kth_element(&nums1, &nums2, k1);
        let num_k2 = get_kth_element(&nums1, &nums2, k2);

        (num_k1 + num_k2) as f64 / 2.0
    } else {
        // 奇数
        let k = sum / 2 + 1;

        let num_median = get_kth_element(&nums1, &nums2, k);

        num_median as f64
    };
}

// 使用循环实现，当然也可以选择使用递归
fn get_kth_element(nums1: &Vec<i32>, nums2: &Vec<i32>, mut k: usize) -> i32 {
    let mut num1_i = 0;
    let mut num2_i = 0;

    let num1_len = nums1.len();
    let num2_len = nums2.len();

    loop {
        if num1_i == num1_len {
            return nums2[num2_i - 1 + k];
        }

        if num2_i == num2_len {
            return nums1[num1_i - 1 + k];
        }

        if k == 1 {
            return min(nums1[num1_i], nums2[num2_i]);
        }

        let half = k / 2;

        let new_num1_i: usize;
        if half + num1_i > num1_len {
            new_num1_i = num1_len - 1;
        } else {
            // 注意这里的计算方式⚠️
            // 写作 num1_i - 1 + half 更容易理解，但是会导致num1_i attempt to subtract with overflow
            new_num1_i = num1_i + half - 1;
        }

        let new_num2_i: usize;
        if half + num2_i > num2_len {
            new_num2_i = num2_len - 1;
        } else {
            new_num2_i = num2_i + half - 1;
        }

        let val1 = nums1[new_num1_i];
        let val2 = nums2[new_num2_i];

        if val1 <= val2 {
            // 注意这里的计算方式⚠️
            k = k - (new_num1_i - num1_i + 1);
            num1_i = new_num1_i + 1;
        } else {
            k = k - (new_num2_i - num2_i + 1);
            num2_i = new_num2_i + 1;
        }
    }
}


// 划分数组
pub fn find_median_sorted_arrays_v2(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let num1_len = nums1.len();
    let num2_len = nums2.len();

    if num1_len > num2_len {
        return find_median_sorted_arrays_v2(nums2, nums1);
    }

    // 线左边的element数量: (num1_len + num2_len + 1) / 2
    let left_size = (num1_len + num2_len + 1) / 2;

    // 设 num1 i 的移动范围: [left, right] = (初始值)[0, num1_len]
    // 则 num2 j 的值为 left_size - i
    let mut left = 0;
    let mut right = num1_len;

    // 记录·上一次· 划线左侧最大值 右侧最小值
    let mut left_max: i32 = 0;
    let mut right_min: i32 = 0;

    // 找到最大的满足 A[i-1] <= B[j]的值
    while left <= right {
        let i = (left + right) / 2;
        let j = left_size - i;

        // 左侧：[i-1]  右侧：[i]
        let num1_a: i32;
        let num1_b: i32;

        let num2_a: i32;
        let num2_b: i32;

        // 四种极限情况
        // num1划线左侧取i32 min   划线右侧取i32 max
        if i == 0 {
            num1_a = i32::MIN;
            num1_b = nums1[i];
        } else if i == num1_len {
            num1_a = nums1[i - 1];
            num1_b = i32::MAX;
        } else {
            num1_a = nums1[i - 1];
            num1_b = nums1[i];
        }

        // num2划线左侧取i32 min   划线右侧取i32 max
        if j == 0 {
            num2_a = i32::MIN;
            num2_b = nums2[j];
        } else if j == num2_len {
            num2_a = nums2[j - 1];
            num2_b = i32::MAX;
        } else {
            num2_a = nums2[j - 1];
            num2_b = nums2[j];
        }

        if num1_a <= num2_b {
            // 左侧取max 右侧取min
            left_max = max(num1_a, num2_a);
            right_min = min(num1_b, num2_b);

            left = i + 1;
        } else {
            right = i - 1;
        }
    }

    return if (num1_len + num2_len) % 2 == 0 {
        // 偶数
        (left_max + right_min) as f64 / 2.0
    } else {
        // 奇数
        left_max as f64
    };
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_median_sorted_arrays() {
        let num1 = vec![1, 2, 3, 6];
        let num2 = vec![1, 3, 4, 5, 9, 10];

        let num_median = find_median_sorted_arrays(num1, num2);
        assert_eq!(num_median, 3.5);
        println!("{num_median}");

        let num3 = vec![1, 2, 3, 6];
        let num4 = vec![1, 3, 4, 5, 9, 10];

        let num_median2 = find_median_sorted_arrays(num4, num3);
        assert_eq!(num_median2, 3.5);
        println!("{num_median2}");
    }

    #[test]
    fn test_find_median_sorted_arrays_v2() {
        let num1 = vec![1, 2, 3, 6];
        let num2 = vec![1, 3, 4, 5, 9, 10];

        let num_median = find_median_sorted_arrays_v2(num1, num2);
        assert_eq!(num_median, 3.5);
        println!("{num_median}");

        let num3 = vec![1, 2, 3, 6];
        let num4 = vec![1, 3, 4, 5, 9, 10];

        let num_median2 = find_median_sorted_arrays_v2(num4, num3);
        assert_eq!(num_median2, 3.5);
        println!("{num_median2}");
    }
}












































