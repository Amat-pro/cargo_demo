package backtrack

func combine(n int, k int) [][]int {
	// [1..n]  k个

	result := [][]int{}

	nums := []int{}
	for i := 0; i < n; i++ {
		nums = append(nums, i+1)
	}

	if len(nums) == 0 {
		return result
	}

	current := []int{}
	var backtracking func(startIdex int)
	backtracking = func(startIdex int) {
		// 确定终止条件
		if len(current) == k {
			// 收集结果
			temp := make([]int, k)
			copy(temp, current)
			result = append(result, temp)
			return
		}

		// for循环
		for i := startIdex; i < len(nums); i++ {
			// 单个循环处理逻辑
			current = append(current, nums[i])
			// 递归 starIdex为i+1
			backtracking(i + 1)
			// 回溯
			current = current[:len(current)-1]
		}

	}

	backtracking(0)
	return result

}
