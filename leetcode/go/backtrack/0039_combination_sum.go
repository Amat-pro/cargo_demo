package backtrack

func combinationSum(candidates []int, target int) [][]int {

	result := [][]int{}
	if len(candidates) == 0 {
		return result
	}

	sum := 0
	path := []int{}

	var backtracking func(idex int)
	backtracking = func(idex int) {
		// 确定终止条件
		if sum == target {
			temp := make([]int, len(path))
			copy(temp, path)
			result = append(result, temp)
			return
		}
		if sum > target {
			return
		}

		// 单层处理逻辑
		for i := idex; i < len(candidates); i++ {
			path = append(path, candidates[i])
			sum += candidates[i]

			// 递归
			backtracking(i)

			// 回溯
			path = path[:len(path)-1]
			sum -= candidates[i]
		}
	}

	backtracking(0)
	return result
}
