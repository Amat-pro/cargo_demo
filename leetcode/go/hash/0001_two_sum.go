package hash

func twoSum(nums []int, target int) []int {
	set := make(map[int]int, 0)

	for i, v := range nums {
		if _, ok := set[target-v]; ok {
			return []int{i, set[target-v]}
		} else {
			set[v] = i
		}
	}

	return []int{}
}
