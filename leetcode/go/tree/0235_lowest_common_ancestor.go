package tree

// 二叉搜索树的最近公共祖先
func lowestCommonAncestor_v2(root *TreeNode, p, q int) *TreeNode {
	if root == nil {
		return nil
	}

	// 左
	if root.Val > p && root.Val > q {
		return lowestCommonAncestor_v2(root.Left, p, q)
	}

	// 右
	if root.Val < p && root.Val < q {
		return lowestCommonAncestor_v2(root.Right, p, q)
	}

	return root

}
