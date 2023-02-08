package array

func sortedSquares(nums []int) []int {
	l := 0
	r := len(nums) - 1

	var a, b int

	for l <= r {
		a = nums[l] * nums[l]
		b = nums[r] * nums[r]
		if a <= b {
			nums[r] = b
			r--
		} else {
			// 交换位置
			nums[l] = nums[r]
			nums[r] = a
			r--
		}
	}

	return nums
}
