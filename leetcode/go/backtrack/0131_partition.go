package backtrack

func partition(s string) [][]string {
	result := [][]string{}

	if s == "" {
		return result
	}

	path := []string{}

	var backtracking func(index int)
	backtracking = func(index int) {
		// 确定终止条件
		if index == len(s) {
			// 收集结果
			temp := make([]string, len(path))
			copy(temp, path)
			result = append(result, temp)
			return
		}

		// 单层处理逻辑

		// for
		for i := index; i < len(s); i++ {
			str := s[index : i+1]
			if isPalindrome(str) {
				path = append(path, str)
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

func isPalindrome(str string) bool {
	left, right := 0, len(str)-1

	for left < right {
		if str[left] != str[right] {
			return false
		}
		left++
		right--
	}

	return true
}
