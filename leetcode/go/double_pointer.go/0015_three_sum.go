package double_pointer

import (
	"sort"
)

func threeSum(nums []int) [][]int {
	result := [][]int{}
	length := len(nums)

	// 排序
	sort.Ints(nums)

	var left, right int

	for i := 0; i < length-2; i++ {
		if nums[i] > 0 {
			return result
		}

		// 对i去重
		if i > 0 && nums[i] == nums[i-1] {
			continue
		}

		left = i + 1
		right = length - 1

		for left < right {
			if nums[i]+nums[left]+nums[right] < 0 {
				left++
			} else if nums[i]+nums[left]+nums[right] > 0 {
				right--
			} else {
				result = append(result, []int{nums[i], nums[left], nums[right]})
				left++
				right--
				// 对left去重
				for left < right && nums[left] == nums[left-1] {
					left++
				}
				// 对right去重
				for left < right && nums[right] == nums[right+1] {
					right--
				}
			}
		}
	}

	return result
}
