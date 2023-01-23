package remove_duplicates

import "fmt"

// / leetcode 026
// /
// / [26.删除有序数组中的重复项](https://leetcode.cn/problems/remove-duplicates-from-sorted-array/)
func removeDuplicates(nums []int) int {
	l := len(nums)
	if l < 2 {
		return l
	}

	if l == 2 {
		if nums[0] == nums[1] {
			return 1
		} else {
			return 2
		}
	}

	i := 1
	pointer := 1 // pointer代表移动指针

	for ; pointer < l; pointer++ {
		if nums[pointer] != nums[pointer-1] {
			i++
			nums[i] = nums[pointer]
		}
	}

	fmt.Println("===>> ", nums)
	return i

}
