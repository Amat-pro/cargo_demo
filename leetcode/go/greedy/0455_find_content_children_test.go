package greedy

import (
	"fmt"
	"testing"
)

func Test_findContentChildren(t *testing.T) {
	// 1
	fmt.Println("==> ", findContentChildren([]int{1, 2, 3}, []int{1, 1}))
	// 2
	fmt.Println("==> ", findContentChildren([]int{1, 2}, []int{1, 2, 3}))
}
