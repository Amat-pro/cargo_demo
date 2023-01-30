package tree

func isSymmtric(root *TreeNode) bool {
	return compare(root.Left, root.Right)
}

func compare(left *TreeNode, right *TreeNode) bool {
	if left == nil && right == nil {
		return true
	}

	if (left != nil && right == nil) ||
		(left == nil && right != nil) {
		return false
	}

	if left.Val != right.Val {
		return false
	}

	// left与right值相等
	return compare(left.Left, right.Right) && compare(left.Right, right.Left)
}
