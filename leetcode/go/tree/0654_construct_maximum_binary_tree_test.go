package tree

import (
	"fmt"
	"testing"
)

func Test_constructMaximumBinaryTree(t *testing.T) {

	fmt.Println(inOrderTraversal(constructMaximumBinaryTree([]int{3, 2, 1, 6, 0, 5})))
	fmt.Println(inOrderTraversal(constructMaximumBinaryTree([]int{3, 2, 1})))

}
