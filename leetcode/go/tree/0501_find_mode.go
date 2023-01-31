package tree

// 二叉搜索树中的众树
func findMod(root *TreeNode) []int {

	result := []int{}
	count := 0
	maxCount := 0

	var travel func(node *TreeNode)
	// 二叉搜索树-中序遍历
	var pre *TreeNode = nil
	travel = func(node *TreeNode) {
		if node == nil {
			return
		}

		// 左
		travel(node.Left)

		// 中
		if pre == nil {
			count = 1
		} else if node.Val == pre.Val {
			count++
		} else {
			// 二叉搜索树，不相等了说明不会再有重复元素出现了
			count = 1
		}

		//
		pre = node

		// 收获结果集
		if count == maxCount {
			result = append(result, node.Val)
		} else if count > maxCount {
			result = []int{}
			result = append(result, node.Val)

			maxCount = count
		}

		// 右
		travel(node.Right)

	}

	travel(root)

	return result
}
