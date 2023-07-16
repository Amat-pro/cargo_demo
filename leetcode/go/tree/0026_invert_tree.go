package tree

// 最好使用前序或后序实现

// 前序递归实现
func invertTree_pre(root *TreeNode) *TreeNode {
	if root == nil {
		return nil
	}

	root.Left, root.Right = root.Right, root.Left
	invertTree_pre(root.Left)
	invertTree_pre(root.Right)

	return root

}

// 后序递归实现
func invertTree_post(root *TreeNode) *TreeNode {
	if root == nil {
		return nil
	}

	invertTree_post(root.Left)
	invertTree_post(root.Right)
	root.Left, root.Right = root.Right, root.Left

	return root

}
