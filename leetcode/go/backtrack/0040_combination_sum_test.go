package backtrack

import (
	"fmt"
	"testing"
)

func Test_combinationSum_repeated(t *testing.T) {
	// [[1, 7],[1, 2, 5],[2, 6],[1, 1, 6]]
	fmt.Println("==> ", combinationSum_repeated([]int{10, 1, 2, 7, 6, 1, 5}, 8))
	fmt.Println()
	// [[1,2,2], [5]]
	fmt.Println("==> ", combinationSum_repeated([]int{2, 5, 2, 1, 2}, 5))
}
