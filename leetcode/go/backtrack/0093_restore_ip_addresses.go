package backtrack

import (
	"strconv"
	"strings"
)

func restoreIpAddresses(s string) []string {

	result := []string{}
	if len(s) == 0 {
		return result
	}

	path := []string{}
	var backtracking func(index int)
	backtracking = func(index int) {
		// 确定终止条件
		if len(path) == 4 {
			if index == len(s) { // 满足4个并且index为len(s)了，说明切割正确的结束了
				temp := make([]string, 4)
				copy(temp, path)
				result = append(result, strings.Join(temp, "."))
			}
			return
		}

		// 单层处理逻辑
		// for
		for i := index; i < len(s); i++ {
			str := s[index : i+1]
			if isValid(str) {
				path = append(path, str)
				// 递归
				backtracking(i + 1)

				// 回溯
				path = path[:len(path)-1]
			} else {
				break
			}
		}
	}

	backtracking(0)
	return result

}

func isValid(s string) bool {
	if len(s) > 1 && strings.HasPrefix(s, "0") {
		return false
	}

	val, err := strconv.Atoi(s)
	if err != nil {
		return false
	}

	if val >= 0 && val <= 255 {
		return true
	}

	return false
}
