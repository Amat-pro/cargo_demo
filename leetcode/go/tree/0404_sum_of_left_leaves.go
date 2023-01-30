package tree

// 后序遍历
func sumOfLeftLeaves(root *TreeNode) int {
	if root == nil {
		return 0
	}

	// 叶子结点 （遍历到叶子结点时返回0）
	if root.Left == nil && root.Right == nil {
		return 0
	}

	// 左 (求左子树左叶子节点和)
	leftVal := sumOfLeftLeaves(root.Left)
	// 叶子结点的父节点  （遍历到叶子结点的父节点时，能得出左叶子结点的值）
	if root.Left != nil && root.Left.Left == nil && root.Left.Right == nil {
		leftVal = root.Left.Val
	}

	// 右 (求右子树左叶子节点和)
	rightVal := sumOfLeftLeaves(root.Right)

	// 中
	return leftVal + rightVal

}
