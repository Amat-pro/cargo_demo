package remove_element

// / leetcode 027
// /
// / [27.移除重复元素](https://leetcode.cn/problems/remove-element/)
func removeElement(nums []int, val int) int {
	// 双指针
	left, right := 0, len(nums)-1
	for left <= right {
		if nums[left] == val {
			nums[left] = nums[right]
			right--
		} else {
			left++
		}
	}

	return left
}
