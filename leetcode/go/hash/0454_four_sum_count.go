package hash

func fourSumCount(nums1, nums2, nums3, nums4 []int) int {

	// key: nums1 + nums2两数之和  value: count
	map1 := make(map[int]int, 0)

	for _, v1 := range nums1 {
		for _, v2 := range nums2 {
			map1[v1+v2]++
		}
	}

	result := 0
	for _, v3 := range nums3 {
		for _, v4 := range nums4 {
			result += map1[-v3-v4]
		}
	}

	return result

}
