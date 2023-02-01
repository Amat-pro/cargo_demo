package backtrack

import (
	"fmt"
	"testing"
)

func Test_combinationSum(t *testing.T) {
	// candidates = [2,3,6,7], target = 7,
	// [[7],[2,2,3]]
	fmt.Println("==> ", combinationSum([]int{2, 3, 6, 7}, 7))

	// candidates = [2,3,5], target = 8,
	// [[2,2,2,2],[2,3,3],[3,5]]
	fmt.Println("==> ", combinationSum([]int{2, 3, 5}, 8))
}
