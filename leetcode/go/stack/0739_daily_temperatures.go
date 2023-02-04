package stack

// 暴力求解
func dailyTemperatures_v1(temperatures []int) []int {
	length := len(temperatures)
	result := make([]int, length)

	if length <= 1 {
		return result
	}

	for i := 0; i < length-1; i++ {
		for j := i; j < length; j++ {
			if temperatures[j] > temperatures[i] {
				result[i] = j - i
				break
			}
		}
	}

	return result
}

// 单调栈
func dailyTemperatures_v2(temperatures []int) []int {
	length := len(temperatures)
	result := make([]int, length)
	if length <= 1 {
		return result
	}

	stack := make([]int, 0)
	for i := 0; i < length; i++ {
		if len(stack) == 0 {
			stack = append(stack, i)
			continue
		}

		for len(stack) > 0 && temperatures[i] > temperatures[stack[len(stack)-1]] {
			result[stack[len(stack)-1]] = i - stack[len(stack)-1]
			stack = stack[:len(stack)-1]
		}
		stack = append(stack, i)

	}

	return result
}
