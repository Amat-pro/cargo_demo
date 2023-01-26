package hash

func intersection(nums1 []int, nums2 []int) []int {
	set := make(map[int]int, 0)
	for _, v := range nums1 {
		if _, ok := set[v]; !ok {
			set[v] = v
		}
	}

	result := []int{}
	for _, v := range nums2 {
		if _, ok := set[v]; ok {
			result = append(result, v)
			// 这里需要清除set中v值已达到不重复的目的
			delete(set, v)
		}
	}

	return result

}
