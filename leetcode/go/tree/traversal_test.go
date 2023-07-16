package tree

import (
	"fmt"
	"testing"
)

func getRoot() *TreeNode {
	root := &TreeNode{
		Val:   5,
		Left:  nil,
		Right: nil,
	}

	one := &TreeNode{
		Val:   1,
		Left:  nil,
		Right: nil,
	}

	two := &TreeNode{
		Val:   2,
		Left:  nil,
		Right: nil,
	}

	four := &TreeNode{
		Val:   4,
		Left:  nil,
		Right: nil,
	}

	six := &TreeNode{
		Val:   6,
		Left:  nil,
		Right: nil,
	}

	root.Left = four
	four.Left = one
	four.Right = two
	root.Right = six

	return root
}

func Test_OrderTraversal(t *testing.T) {
	root := getRoot()

	// ==> :  [5 4 1 2 6]
	// ==> :  [1 4 2 5 6]
	// ==> :  [1 2 4 6 5]
	fmt.Println("==> : ", preOrderTraversal(root))
	fmt.Println("==> : ", inOrderTraversal(root))
	fmt.Println("==> : ", postOrderTraversal(root))
}
