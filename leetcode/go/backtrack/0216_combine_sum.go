package backtrack

func combineSum(k int, n int) [][]int {
	result := [][]int{}

	nums := []int{1, 2, 3, 4, 5, 6, 7, 8, 9}

	current := []int{}
	sum := 0
	var backtracking func(startIdex int, sum int)
	backtracking = func(startIdex int, sum int) {
		// 确定终止条件
		if len(current) == k {
			// 收集结果
			if sum == n {
				temp := make([]int, k)
				copy(temp, current)
				result = append(result, temp)
				return
			}
			return
		}

		// for循环
		for i := startIdex; i < len(nums); i++ {
			// 单个循环处理逻辑
			current = append(current, nums[i])
			// 递归 starIdex为i+1
			backtracking(i+1, sum+nums[i])
			// 回溯
			current = current[:len(current)-1]
		}

	}

	backtracking(0, sum)
	return result

}
