package greedy

import (
	"fmt"
	"testing"
)

func Test_wiggleMaxLength(t *testing.T) {
	// 6
	fmt.Println("==> ", wiggleMaxLength([]int{1, 7, 4, 9, 2, 5}))
	// 7
	fmt.Println("==> ", wiggleMaxLength([]int{1, 17, 5, 10, 13, 15, 10, 5, 16, 8}))
	// 2
	fmt.Println("==> ", wiggleMaxLength([]int{1, 2, 3, 4, 5, 6, 7, 8, 9}))
}
