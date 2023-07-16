package tree

// 前序遍历
func mergeTrees(t1, t2 *TreeNode) *TreeNode {

	if t1 == nil && t2 == nil {
		return nil
	}

	var node *TreeNode
	if t1 != nil && t2 != nil {
		node = &TreeNode{
			Val: t1.Val + t2.Val,
		}
		node.Left = mergeTrees(t1.Left, t2.Left)
		node.Right = mergeTrees(t1.Right, t2.Right)
	} else if t1 != nil && t2 == nil {
		node = &TreeNode{
			Val: t1.Val,
		}
		node.Left = t1.Left
		node.Right = t1.Right
	} else if t1 == nil && t2 != nil {
		node = &TreeNode{
			Val: t2.Val,
		}
		node.Left = t2.Left
		node.Right = t2.Right
	}

	return node

}

// 前序遍历
func mergeTrees_v2(root1 *TreeNode, root2 *TreeNode) *TreeNode {
	if root1 == nil {
		return root2
	}
	if root2 == nil {
		return root1
	}
	root1.Val += root2.Val
	root1.Left = mergeTrees(root1.Left, root2.Left)
	root1.Right = mergeTrees(root1.Right, root2.Right)
	return root1
}
