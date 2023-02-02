package backtrack

func permute(nums []int) [][]int {
	result := [][]int{}
	if len(nums) == 0 {
		return result
	}

	path := []int{}
	var backtracking func(used []bool)
	backtracking = func(used []bool) {
		// 确定终止条件
		// 收集结果集
		if len(path) == len(nums) {
			temp := make([]int, len(nums))
			copy(temp, path)
			result = append(result, temp)
			return
		}

		// 单层处理逻辑
		for i := 0; i < len(nums); i++ {
			if used[i] {
				continue
			}
			path = append(path, nums[i])
			used[i] = true

			// 递归
			backtracking(used)

			path = path[:len(path)-1]
			used[i] = false
		}
	}

	used := make([]bool, len(nums))
	backtracking(used)
	return result
}
