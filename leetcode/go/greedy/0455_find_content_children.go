package greedy

import "sort"

func findContentChildren(g []int, s []int) int {

	// 排序
	sort.Ints(g)
	sort.Ints(s)

	count := 0
	index := len(s) - 1 // 🍪
	for i := len(g) - 1; i >= 0; i-- {

		if index >= 0 && s[index] >= g[i] {
			// 满足条件则进行投喂
			count++
			index--
		}

	}

	return count

}
