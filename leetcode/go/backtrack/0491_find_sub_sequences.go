package backtrack

func findSubSequences(nums []int) [][]int {
	result := [][]int{}

	if len(nums) == 0 {
		return result
	}

	path := []int{}
	var backtracking func(idex int)
	backtracking = func(idex int) {
		// 确定终止条件
		if idex > len(nums) {
			return
		}

		// 单层处理逻辑
		// 收集结果
		if len(path) >= 2 {
			temp := make([]int, len(path))
			copy(temp, path)
			result = append(result, temp)
		}
		// for循环
		used := make(map[int]bool)
		for i := idex; i < len(nums); i++ {
			// 去重
			if used[nums[i]] {
				continue
			}
			used[nums[i]] = true
			// 判断递增
			if len(path) == 0 || (len(path) != 0 && nums[i] >= path[len(path)-1]) {
				path = append(path, nums[i])
				// 递归
				backtracking(i + 1)
				// 回溯
				path = path[:len(path)-1]
			}
		}

	}

	backtracking(0)
	return result

}
