package tree

// 从中序与后序遍历序列构造二叉树
func buildTree(inOrder, postOrder []int) *TreeNode {
	if len(postOrder) == 0 {
		// 根结点(中结点从后序中找到的，如果没有则直接返回)
		return nil
	}

	root := build(inOrder, postOrder)
	return root
}

// rootIdx产生postOrder
func build(inOrder, postOrder []int) *TreeNode {

	if len(inOrder) == 1 {
		return &TreeNode{
			Val: inOrder[0],
		}
	}

	rootIdex := len(postOrder) - 1
	root := &TreeNode{Val: postOrder[rootIdex]}

	var rootIdx_InOrder int
	for i, v := range inOrder {
		if v == postOrder[rootIdex] {
			rootIdx_InOrder = i
			break
		}
	}

	var rightSubTree_fist_element_idex int
	for i, v := range postOrder {
		if v == inOrder[rootIdx_InOrder+1] {
			rightSubTree_fist_element_idex = i
		}
	}
	root.Left = build(inOrder[:rootIdx_InOrder], postOrder[:rightSubTree_fist_element_idex])
	root.Right = build(inOrder[rootIdx_InOrder+1:], postOrder[rightSubTree_fist_element_idex:len(postOrder)-1])
	return root
}
