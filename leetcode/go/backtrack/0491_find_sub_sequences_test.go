package backtrack

import (
	"fmt"
	"testing"
)

func Test_findSubSequences(t *testing.T) {
	// [[4,6], [4,6,7], [4,6,7,7], [4,7], [4,7,7], [6,7], [6,7,7], [7,7]]
	fmt.Println("==> ", findSubSequences([]int{4, 6, 7, 7}))
	// [[4,7], [4,7,7], [4,6], [4,6,7], [7,7], [6,7]]
	fmt.Println("==> ", findSubSequences([]int{4, 7, 6, 7}))
}
