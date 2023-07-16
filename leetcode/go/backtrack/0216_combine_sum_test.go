package backtrack

import (
	"fmt"
	"testing"
)

func Test_combineSum(t *testing.T) {
	// [[1,2,4]]
	fmt.Println("==> ", combineSum(3, 7))
	// [[1,2,6], [1,3,5], [2,3,4]]
	fmt.Println("==> ", combineSum(3, 9))
}
