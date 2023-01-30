package tree

func isBalanced(root *TreeNode) bool {
	h := getHeight(root)
	// if h == -1 {
	// 	return false
	// }

	// return true

	return h != -1

}

// 当左子树与右子树高度差<=1时，返回高度
// 否则，返回-1
func getHeight(node *TreeNode) int {
	if node == nil {
		return 0
	}

	leftHeight, rightHeight := getHeight(node.Left), getHeight(node.Right)
	if leftHeight == -1 || rightHeight == -1 {
		return -1
	}

	if leftHeight-rightHeight > 1 || rightHeight-leftHeight > 1 {
		return -1
	}

	return max(leftHeight, rightHeight) + 1

}
