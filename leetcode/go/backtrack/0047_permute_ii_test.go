package backtrack

import (
	"fmt"
	"testing"
)

func Test_permute_ii(t *testing.T) {
	// [[1,1,2], [1,2,1], [2,1,1]]
	fmt.Println("==> ", permute_ii([]int{1, 1, 2}))
	// [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
	fmt.Println("==> ", permute_ii([]int{1, 2, 3}))
}
