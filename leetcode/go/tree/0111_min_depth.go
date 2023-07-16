package tree

// 求最小深度 == 求最小高度（后序）
func minDepth(root *TreeNode) int {
	if root == nil {
		return 0
	}

	if root.Left != nil && root.Right == nil {
		return 1 + minDepth(root.Left)
	}

	if root.Left == nil && root.Right != nil {
		return 1 + minDepth(root.Right)
	}

	if root.Left == nil && root.Right == nil {
		return 1
	}

	return 1 + min(minDepth(root.Left), minDepth(root.Right))

}

func min(a, b int) int {
	if a <= b {
		return a
	}

	return b
}
