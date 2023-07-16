use std::cmp::{max, min};

/// leetcode 011
///
/// [11.盛最多水的容器](https://leetcode.cn/problems/container-with-most-water/)
///
pub fn max_area(height: Vec<i32>) -> i32 {
    let n = height.len();
    if n < 2 {
        return 0;
    }

    let mut max_area = 0;
    let mut pt_left = 0;
    let mut pt_right = n - 1;

    let mut current_area: i32;

    while pt_left < pt_right {
        current_area = (pt_right - pt_left) as i32 * min(height[pt_left], height[pt_right]);
        max_area = max(max_area, current_area);
        if height[pt_left] < height[pt_right] {
            pt_left += 1;
        } else {
            pt_right -= 1;
        }
    }

    max_area
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max_area() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        assert_eq!(max_area(height), 49);

        let height1 = vec![1, 8];
        assert_eq!(max_area(height1), 1);
    }
}