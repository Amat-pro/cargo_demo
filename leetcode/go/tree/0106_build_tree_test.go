package tree

import (
	"fmt"
	"testing"
)

func Test_buildTree(t *testing.T) {
	fmt.Println("==> ", inOrderTraversal(buildTree([]int{9, 3, 15, 20, 7}, []int{9, 15, 7, 20, 3})))
	fmt.Println("==> ", inOrderTraversal(buildTree([]int{1, 4, 2, 5, 6}, []int{1, 2, 4, 6, 5})))

}
