package stack

import (
	"fmt"
	"testing"
)

func Test_nextGreaterElement(t *testing.T) {
	// [-1,3,-1]
	fmt.Println("==> ", nextGreaterElement([]int{4, 1, 2}, []int{1, 3, 4, 2}))
	// [3,-1]
	fmt.Println("==> ", nextGreaterElement([]int{2, 4}, []int{1, 2, 3, 4}))
}
