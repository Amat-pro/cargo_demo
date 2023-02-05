package rob

import (
	"fmt"
	"testing"
)

func Test_rob_iii(t *testing.T) {
	root := &TreeNode{
		Val: 3,
		Left: &TreeNode{
			Val: 4,
			Left: &TreeNode{
				Val: 1,
			},
			Right: &TreeNode{
				Val: 3,
			},
		},
		Right: &TreeNode{
			Val: 5,
			Right: &TreeNode{
				Val: 1,
			},
		},
	}
	// 9
	fmt.Println("==> ", rob_iii(root))
}
