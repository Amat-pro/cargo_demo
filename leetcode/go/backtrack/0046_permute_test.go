package backtrack

import (
	"fmt"
	"testing"
)

func Test_permute(t *testing.T) {
	// [[1,2,3], [1,3,2], [2,1,3], [2,3,1], [3,1,2], [3,2,1]]
	fmt.Println("==> ", permute([]int{1, 2, 3}))
}
