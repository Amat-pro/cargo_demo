package tree

// 特殊情况能被下面的代码包含，详情：https://www.bilibili.com/video/BV1jd4y1B7E2/?vd_source=cc141d8be9451fa7758d9fc60e7bceb3
// 后序遍历
func lowestCommonAncestor(root *TreeNode, p, q int) *TreeNode {

	if root == nil {
		return nil
	}

	if root.Val == p || root.Val == q {
		return root
	}

	// 左
	left := lowestCommonAncestor(root.Left, p, q)

	// 右
	right := lowestCommonAncestor(root.Right, p, q)

	// 中
	if left != nil && right != nil {
		return root
	}

	if left == nil && right != nil {
		return right
	}

	if left != nil && right == nil {
		return left
	}

	return nil

}
