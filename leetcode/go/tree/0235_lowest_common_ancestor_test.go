package tree

import (
	"fmt"
	"testing"
)

func Test_lowestCommonAncestor_v2(t *testing.T) {
	root := &TreeNode{
		Val: 6,
		Left: &TreeNode{
			2,
			&TreeNode{
				Val: 0,
			},
			&TreeNode{
				4,
				&TreeNode{
					Val: 3,
				},
				&TreeNode{
					Val: 5,
				},
			},
		},
		Right: &TreeNode{
			8,
			&TreeNode{
				Val: 7,
			},
			&TreeNode{
				Val: 9,
			},
		},
	}

	// 6
	fmt.Println("==> ", lowestCommonAncestor_v2(root, 2, 8).Val)
	// 2
	fmt.Println("==> ", lowestCommonAncestor_v2(root, 2, 4).Val)
}
