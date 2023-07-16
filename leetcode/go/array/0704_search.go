package array

func search(nums []int, target int) int {

	l := len(nums)

	return recursionsSearch(0, l-1, nums, target)

}

func recursionsSearch(left, right int, nums []int, target int) int {

	if right < left {
		return -1
	}

	if left == right {
		if nums[left] == target {
			return left
		} else {
			return -1
		}
	}

	m := (left + right + 1) / 2
	if target == nums[m] {
		return m
	}
	if target > nums[m] {
		// [m, right]
		return recursionsSearch(m, right, nums, target)
	} else {
		// [left, m-1]
		return recursionsSearch(left, m-1, nums, target)
	}

}

func whileSearch(nums []int, target int) int {
	left := 0
	right := len(nums) - 1

	var mid int

	for left <= right {
		mid = (left + right + 1) / 2
		if target == nums[mid] {
			return mid
		}
		if target > nums[mid] {
			left = mid
		} else {
			right = mid - 1
		}
	}

	return -1

}
