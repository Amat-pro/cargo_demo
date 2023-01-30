package tree

import "strconv"

func binaryTreePaths(root *TreeNode) []string {
	res := make([]string, 0)

	if root == nil {
		return res
	}

	var travelFunc func(root *TreeNode, s string)
	travelFunc = func(node *TreeNode, s string) {
		if node.Left == nil && node.Right == nil {
			// 叶子结点
			v := s + strconv.Itoa(node.Val)
			res = append(res, v)
			return
		}

		s = s + strconv.Itoa(node.Val) + "->"

		if node.Left != nil {
			travelFunc(node.Left, s)
		}

		if node.Right != nil {
			travelFunc(node.Right, s)
		}

	}

	travelFunc(root, "")
	return res

}
