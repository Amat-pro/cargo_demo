package stack

// nums是循环数组  需要循环搜索
func nextGreaterElement_ii(nums []int) []int {
	length := len(nums)
	if length == 0 {
		return []int{}
	}

	result := make([]int, length)
	for i := 0; i < length; i++ {
		result[i] = -1
	}

	stack := []int{}
	// length * 2 (模拟拼接)
	for i := 0; i < length*2; i++ {
		if len(stack) == 0 {
			// 这里的position也是i%length
			stack = append(stack, i%length)
			continue
		}

		// position: i % length
		for len(stack) > 0 && nums[i%length] > nums[stack[len(stack)-1]] {
			result[stack[len(stack)-1]] = nums[i%length]
			stack = stack[:len(stack)-1]
		}

		// 注意这里的position也是i%length
		stack = append(stack, i%length)

	}

	return result
}
