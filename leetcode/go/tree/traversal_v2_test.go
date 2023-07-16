package tree

import (
	"fmt"
	"testing"
)

func Test_OrderTraversal_v2(t *testing.T) {
	root := getRoot()
	// ==> :  [5 4 1 2 6]
	// ==> :  [1 4 2 5 6]
	// ==> :  [1 2 4 6 5]
	fmt.Println("==> : ", preOrderTraversal_v2(root))
	fmt.Println("==> : ", inOrderTraversal_v2(root))
	fmt.Println("==> : ", postOrderTraversal_v2(root))
}
