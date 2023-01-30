package tree

// 非递归实现
// 可以使用栈模拟递归

func preOrderTraversal_v2(root *TreeNode) []int {
	result := []int{}

	if root == nil {
		return result
	}

	stack := []*TreeNode{root}

	var node *TreeNode
	for len(stack) > 0 {
		node = stack[len(stack)-1]
		stack = stack[:len(stack)-1]

		result = append(result, node.Val)
		if node.Right != nil {
			stack = append(stack, node.Right)
		}
		if node.Left != nil {
			stack = append(stack, node.Left)
		}
	}

	return result

}

func postOrderTraversal_v2(root *TreeNode) []int {
	result := []int{}

	if root == nil {
		return result
	}

	stack := []*TreeNode{root}

	var node *TreeNode
	for len(stack) > 0 {
		node = stack[len(stack)-1]
		stack = stack[:len(stack)-1]

		result = append(result, node.Val)
		if node.Left != nil {
			stack = append(stack, node.Left)
		}
		if node.Right != nil {
			stack = append(stack, node.Right)
		}
	}

	reverse(result)
	return result
}

func reverse(nums []int) {
	l, r := 0, len(nums)-1
	for l < r {
		nums[r], nums[l] = nums[l], nums[r]
		l++
		r--
	}
}

// 可以作为统一迭代的方法
func inOrderTraversal_v2(root *TreeNode) []int {
	result := []int{}

	if root == nil {
		return result
	}

	stack := []*TreeNode{}
	var p *TreeNode
	var node *TreeNode

	p = root

	for len(stack) > 0 || p != nil {
		if p != nil {
			stack = append(stack, p)
			p = p.Left
		} else {
			// 出栈
			node = stack[len(stack)-1]
			stack = stack[:len(stack)-1]

			result = append(result, node.Val)
			p = node.Right
		}
	}

	return result
}
