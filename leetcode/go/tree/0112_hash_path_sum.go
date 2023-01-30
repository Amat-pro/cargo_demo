package tree

func hashPathSum(root *TreeNode, target int) bool {
	all_path_vals := binary_tree_path(root)
	sum := 0
	for _, path := range all_path_vals {
		sum = 0
		for _, val := range path {
			sum += val
		}

		if sum == target {
			return true
		}
	}
	return false
}

// 使用前序遍历-递归+回溯
func binary_tree_path(root *TreeNode) [][]int {
	result := [][]int{}
	path := []int{}

	if root == nil {
		return result
	}

	var f func(node *TreeNode)
	f = func(node *TreeNode) {
		// 终止条件 (叶子结点)
		if node.Left == nil && node.Right == nil {
			path = append(path, node.Val)
			temp := make([]int, len(path))
			copy(temp, path)
			result = append(result, temp)
			return
		}

		// 中
		path = append(path, node.Val)

		// 左
		if node.Left != nil {
			f(node.Left)
			// 回溯-出栈
			path = path[:len(path)-1]
		}

		// 右
		if node.Right != nil {
			f(node.Right)
			// 回溯-出栈 （un_need）
			// path = path[:len(path)-1]
		}
	}

	f(root)

	return result
}
