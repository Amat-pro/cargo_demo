package array

func removeElement(nums []int, val int) int {
	i := 0
	p := len(nums) - 1

	for i <= p {
		if nums[i] == val {
			if nums[p] != val {
				nums[i] = nums[p]
				p--
			} else {
				p--
			}
		} else {
			i++
		}
	}

	return i
}
