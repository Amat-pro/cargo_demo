package backtrack

func subSets(nums []int) [][]int {

	result := [][]int{}
	if len(nums) == 0 {
		return result
	}

	path := []int{}
	var backtracking func(idex int)
	backtracking = func(idex int) {
		// 收集结果集
		temp := make([]int, len(path))
		copy(temp, path)
		result = append(result, temp)

		// 单层处理逻辑
		for i := idex; i < len(nums); i++ {
			path = append(path, nums[i])

			// 递归
			backtracking(i + 1)

			// 回溯
			path = path[:len(path)-1]
		}
	}

	backtracking(0)

	return result

}
