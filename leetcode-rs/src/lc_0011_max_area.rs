/// 0011 max_area 盛水最多的容器
/// 给定一个长度为 n 的整数数组 height 。有 n 条垂线，第 i 条线的两个端点是 (i, 0) 和 (i, height[i])
/// 找出其中的两条线，使得它们与 x 轴共同构成的容器可以容纳最多的水
/// 返回容器可以储存的最大水量
/// [1,8,6,2,5,4,8,3,7] -> 49
/// area = (j - i) * min(height[i], height[j]) [h=min(height[i], height[j]]
/// 双指针 - left,right left向右移动,right向左移动，
/// 当left<right&&height[left]<=h时left++ (left向右移动时,j-i变小,height[left]<=h可以直接跳过)
/// 当left<right&&height[right]<=h时right-- (right向左移动时,j-i变小,height[right]<=h可以直接跳过)
pub fn max_area(height: Vec<i32>) -> i32 {
    let len = height.len();
    if len < 2 {
        return 0;
    }
    let mut left: usize = 0;
    let mut right: usize = len - 1;

    let mut result = 0;
    let mut h: i32;

    while left < right {
        h = height[left].min(height[right]);
        result = result.max(h * (right - left) as i32);

        while left < right && height[left] <= h {
            left += 1;
        }
        while left < right && height[right] <= h {
            right -= 1;
        }
    }
    result
}