package tree

// 求高度：后序
// 求深度：前序
// 最大深度 == 根节点高度 （所以使用后序求高度即可）
func maxDepth(root *TreeNode) int {
	if root == nil {
		return 0
	}

	return max(maxDepth(root.Left), maxDepth(root.Right)) + 1
}

func max(a, b int) int {
	if a >= b {
		return a
	}

	return b
}
