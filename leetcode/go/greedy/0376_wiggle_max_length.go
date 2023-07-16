package greedy

func wiggleMaxLength(nums []int) int {

	length := len(nums)
	if length < 2 {
		return length
	}

	preDiff := nums[1] - nums[0]
	count := 0
	if preDiff != 0 {
		// 前两个元素不是平坡，则count设置为2
		count = 2
	} else {
		count = 1
	}

	// 从第三个元素开始
	for i := 2; i < length; i++ {
		diff := nums[i] - nums[i-1]
		if (diff > 0 && preDiff <= 0) ||
			(diff < 0 && preDiff >= 0) {
			count++
			preDiff = diff
		}
	}

	return count

}
