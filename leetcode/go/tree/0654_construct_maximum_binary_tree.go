package tree

func constructMaximumBinaryTree(nums []int) *TreeNode {
	length := len(nums)
	if length == 0 {
		return nil
	}

	if length == 1 {
		return &TreeNode{
			Val: nums[0],
		}
	}

	var maxVal int
	var maxValIdex int

	maxValIdex = 0
	maxVal = nums[0]
	// 找最大值
	for i, v := range nums {
		if v > maxVal {
			maxVal = v
			maxValIdex = i
		}
	}

	root := &TreeNode{
		Val: maxVal,
	}
	root.Left = constructMaximumBinaryTree(nums[:maxValIdex])
	if maxValIdex+1 < length {
		root.Right = constructMaximumBinaryTree(nums[maxValIdex+1:])
	}

	return root

}
