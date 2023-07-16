package backtrack

import (
	"fmt"
	"testing"
)

func Test_subSetsII(t *testing.T) {
	// [[], [1], [1,2], [1,2,2], [2], [2,2]]
	fmt.Println("==> ", subSetsII([]int{1, 2, 2}))
}
