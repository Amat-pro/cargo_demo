package stack

func nextGreaterElement(nums1, nums2 []int) []int {
	result := make([]int, len(nums1))
	for i := 0; i < len(nums1); i++ {
		result[i] = -1
	}

	position_map := make(map[int]int)
	for i := 0; i < len(nums1); i++ {
		position_map[nums1[i]] = i
	}

	stack := []int{}
	for i := 0; i < len(nums2); i++ {
		if len(stack) == 0 {
			stack = append(stack, i)
			continue
		}

		for len(stack) > 0 && nums2[i] > nums2[stack[len(stack)-1]] {
			if position, ok := position_map[nums2[stack[len(stack)-1]]]; ok {
				result[position] = nums2[i]
			}
			stack = stack[:len(stack)-1]
		}

		stack = append(stack, i)

	}

	return result
}
