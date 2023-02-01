package backtrack

import (
	"fmt"
	"sort"
)

func combinationSum_repeated(candidates []int, target int) [][]int {
	result := [][]int{}
	if len(candidates) == 0 {
		return result
	}

	// 去重问题： 排序！！！
	fmt.Println(candidates)
	sort.Ints(candidates)
	fmt.Println(candidates)

	path := []int{}
	used := make([]bool, len(candidates))
	sum := 0

	var backtracking func(idex int)
	backtracking = func(idex int) {
		// 确定终止条件
		if sum > target {
			return
		}
		if sum == target {
			temp := make([]int, len(path))
			copy(temp, path)
			result = append(result, temp)
			return
		}

		// 单层处理逻辑
		for i := idex; i < len(candidates); i++ {
			// 去重逻辑
			// 区分 树层去重 - 树枝去重  see: https://www.bilibili.com/video/BV12V4y1V73A/?vd_source=cc141d8be9451fa7758d9fc60e7bceb3
			if i > 0 && candidates[i] == candidates[i-1] && !used[i-1] {
				continue
			}
			path = append(path, candidates[i])
			sum += candidates[i]
			used[i] = true

			// 递归
			backtracking(i + 1)

			// 回溯
			path = path[:len(path)-1]
			sum -= candidates[i]
			used[i] = false
		}

	}

	backtracking(0)
	return result
}
