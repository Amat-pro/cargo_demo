package tree

import (
	"fmt"
	"testing"
)

func Test_findMod(t *testing.T) {
	root1 := &TreeNode{
		Val: 1,
		Right: &TreeNode{
			Val: 2,
			Left: &TreeNode{
				Val: 2,
			},
		},
	}

	root2 := &TreeNode{
		Val: 5,
		Left: &TreeNode{
			Val: 2,
			Left: &TreeNode{
				Val: 2,
			},
		},
		Right: &TreeNode{
			Val: 6,
			Left: &TreeNode{
				Val: 6,
			},
		},
	}

	fmt.Println("==> ", findMod(root1))
	fmt.Println("==> ", findMod(root2))

}
