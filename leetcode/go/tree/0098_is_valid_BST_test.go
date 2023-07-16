package tree

import (
	"fmt"
	"testing"
)

func Test_isValidBST(t *testing.T) {
	root := &TreeNode{
		Val:  10,
		Left: &TreeNode{Val: 5},
		Right: &TreeNode{
			Val: 15,
			Left: &TreeNode{
				Val: 6,
			},
			Right: &TreeNode{
				Val: 20,
			},
		},
	}

	// false
	fmt.Println("==> ", isValidBST(root))
}
