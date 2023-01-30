package tree

import (
	"fmt"
	"testing"
)

func getSymmtricNode() *TreeNode {
	root := &TreeNode{
		Val: 1,
		Left: &TreeNode{
			Val: 2,
			Left: &TreeNode{
				Val: 3,
			},
			Right: &TreeNode{
				Val: 4,
			},
		},
		Right: &TreeNode{
			Val: 2,
			Left: &TreeNode{
				Val: 4,
			},
			Right: &TreeNode{
				Val: 3,
			},
		},
	}

	return root

}

func Test_isSymmtric(t *testing.T) {
	fmt.Println("==> ", isSymmtric(getSymmtricNode()))
}
