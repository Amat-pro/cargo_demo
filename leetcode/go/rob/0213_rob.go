package rob

// 环
func rob_ii(nums []int) int {

	if len(nums) == 0 {
		return 0
	}

	if len(nums) == 1 {
		return nums[0]
	}

	return max(rob(nums[:len(nums)-1]), rob(nums[1:]))

}
