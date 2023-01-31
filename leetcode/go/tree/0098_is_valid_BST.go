package tree

// 方法一
// 判断中序产生的数组是否时单调递增的即可

// 方法二
func isValidBST(root *TreeNode) bool {
	var preNode *TreeNode
	preNode = nil

	var checkFn func(node *TreeNode) bool
	// 	中序遍历
	checkFn = func(node *TreeNode) bool {
		if node == nil {
			return true
		}

		// 左
		isLeft := checkFn(node.Left)
		if !isLeft {
			return false
		}

		// 中
		if preNode != nil && node.Val <= preNode.Val {
			return false
		}
		preNode = node

		// 右
		isRight := checkFn(node.Right)

		return isRight
	}

	return checkFn(root)
}
