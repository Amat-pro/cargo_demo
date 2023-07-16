package tree

import (
	"fmt"
	"testing"
)

func Test_lowestCommonAncestor(t *testing.T) {
	root := &TreeNode{
		Val: 3,
		Left: &TreeNode{
			Val: 5,
			Left: &TreeNode{
				Val: 6,
			},
			Right: &TreeNode{
				Val: 2,
				Left: &TreeNode{
					Val: 7,
				},
				Right: &TreeNode{
					Val: 4,
				},
			},
		},
		Right: &TreeNode{
			Val: 1,
			Left: &TreeNode{
				Val: 0,
			},
			Right: &TreeNode{
				Val: 8,
			},
		},
	}

	// 3
	fmt.Println("==> ", lowestCommonAncestor(root, 5, 1).Val)
	// 5
	fmt.Println("==> ", lowestCommonAncestor(root, 5, 4).Val)
}
