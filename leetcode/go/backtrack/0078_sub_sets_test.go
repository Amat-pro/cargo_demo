package backtrack

import (
	"fmt"
	"testing"
)

func Test_subSets(t *testing.T) {
	// [[3],[1],[2],[1,2,3],[1,3],[2,3],[1,2],[]]
	fmt.Println("==> ", subSets([]int{1, 2, 3}))
}
