package backtrack

func subSetsII(nums []int) [][]int {
	result := [][]int{}

	if len(nums) == 0 {
		return result
	}

	path := []int{}
	used := make([]bool, len(nums))
	var backtracking func(idex int)
	backtracking = func(idex int) {
		// 确定终止条件
		if idex > len(nums) {
			return
		}
		// 收集结果集
		temp := make([]int, len(path))
		copy(temp, path)
		result = append(result, temp)

		// 单层处理逻辑
		// for
		for i := idex; i < len(nums); i++ {
			// 去重
			if i > 0 && nums[i] == nums[i-1] && !used[i-1] {
				continue
			}
			path = append(path, nums[i])
			used[i] = true

			// 递归
			backtracking(i + 1)

			// 回溯
			path = path[:len(path)-1]
			used[i] = false
		}

	}

	backtracking(0)
	return result
}
