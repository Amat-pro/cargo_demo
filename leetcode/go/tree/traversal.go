package tree

func preOrderTraversal(root *TreeNode) []int {
	result := []int{}

	var traversalFunc func(node *TreeNode)
	traversalFunc = func(node *TreeNode) {
		if node == nil {
			return
		}

		result = append(result, node.Val)
		traversalFunc(node.Left)
		traversalFunc(node.Right)

	}

	traversalFunc(root)
	return result

}

func inOrderTraversal(root *TreeNode) []int {
	result := []int{}

	var traversalFunc func(node *TreeNode)
	traversalFunc = func(node *TreeNode) {
		if node == nil {
			return
		}

		traversalFunc(node.Left)
		result = append(result, node.Val)
		traversalFunc(node.Right)

	}

	traversalFunc(root)
	return result

}

func postOrderTraversal(root *TreeNode) []int {
	result := []int{}

	var traversalFunc func(node *TreeNode)
	traversalFunc = func(node *TreeNode) {
		if node == nil {
			return
		}

		traversalFunc(node.Left)
		traversalFunc(node.Right)
		result = append(result, node.Val)

	}

	traversalFunc(root)
	return result

}
