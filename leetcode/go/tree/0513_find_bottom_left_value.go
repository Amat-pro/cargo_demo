package tree

func findBottomLeftValue(root *TreeNode) int {

	var gradation int
	stack := []*TreeNode{root}

	var length int
	//
	for len(stack) > 0 {
		length = len(stack)
		for i := 0; i < length; i++ {
			// 让gradation等于每一层最左侧的元素
			if i == 0 {
				gradation = stack[0].Val
			}

			if stack[i].Left != nil {
				stack = append(stack, stack[i].Left)
			}

			if stack[i].Right != nil {
				stack = append(stack, stack[i].Right)
			}
		}
		if len(stack) > length {
			stack = stack[length:]
		} else {
			stack = nil
		}
	}

	return gradation

}
