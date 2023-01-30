package tree

// 利用完全二叉树特性-递归解法
func countNodes(root *TreeNode) int {
	if root == nil {
		return 0
	}

	leftHeight, rightHeight := 0, 0

	leftNode := root.Left
	for leftNode != nil {
		leftNode = leftNode.Left
		leftHeight++
	}

	rightNode := root.Right
	for rightNode != nil {
		rightNode = rightNode.Right
		rightHeight++
	}

	if leftHeight == rightHeight {
		// 2的leftHeight次方 - 1
		return (2 << leftHeight) - 1
	}

	return countNodes(root.Left) + countNodes(root.Right) + 1
}
