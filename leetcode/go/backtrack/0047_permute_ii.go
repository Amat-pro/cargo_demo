package backtrack

import "sort"

func permute_ii(nums []int) [][]int {
	result := [][]int{}
	if len(nums) == 0 {
		return result
	}

	// 排序
	sort.Ints(nums)

	path := []int{}
	var backtracking func(used []bool)
	backtracking = func(used []bool) {
		// 确定终止条件
		// 收集结果
		if len(path) == len(nums) {
			temp := make([]int, len(path))
			copy(temp, path)
			result = append(result, temp)
			return
		}

		// 单层处理逻辑
		for i := 0; i < len(nums); i++ {
			if used[i] {
				continue
			}
			// 去重(树层)
			if i > 0 && nums[i] == nums[i-1] && !used[i-1] {
				continue
			}

			path = append(path, nums[i])
			used[i] = true

			// 递归
			backtracking(used)

			// 回溯
			path = path[:len(path)-1]
			used[i] = false
		}
	}

	used := make([]bool, len(nums))
	backtracking(used)
	return result
}
