package tree

import (
	"fmt"
	"testing"
)

func Test_hashPathSum(t *testing.T) {
	root := &TreeNode{
		Val: 5,
		Left: &TreeNode{
			Val: 4,
			Left: &TreeNode{
				Val: 11,
				Left: &TreeNode{
					Val: 7,
				},
				Right: &TreeNode{
					Val: 2,
				},
			},
		},
		Right: &TreeNode{
			Val: 8,
			Left: &TreeNode{
				Val: 13,
			},
			Right: &TreeNode{
				Val: 4,
				Right: &TreeNode{
					Val: 1,
				},
			},
		},
	}
	fmt.Println("==> ", hashPathSum(root, 22))

	fmt.Println("==> ", binary_tree_path_v2(root))
}

// 其实这个函数会有问题 -- 陷阱！！！
func binary_tree_path_v2(root *TreeNode) [][]int {
	result := [][]int{}

	if root == nil {
		return result
	}

	var f func(node *TreeNode, path []int)
	f = func(node *TreeNode, path []int) {
		// 终止条件 (叶子结点)
		if node.Left == nil && node.Right == nil {
			// 这里的append发生扩容后，回溯拿到的path其实和这里的可能不是相同的地址！！！！
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
			f(node.Left, path)
			// 回溯-出栈
			// path = path[:len(path)-1]
		}

		// 右
		if node.Right != nil {
			f(node.Right, path)
			// 回溯-出栈 (不必)
			// path = path[:len(path)-1]
		}
	}

	f(root, []int{})

	return result
}
