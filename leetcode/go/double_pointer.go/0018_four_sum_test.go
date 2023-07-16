package double_pointer

import (
	"fmt"
	"testing"
)

func Test_fourSum(t *testing.T) {
	// [[-2 -1 1 2] [-2 0 0 2] [-1 0 0 1]]
	fmt.Println("==> ", fourSum([]int{1, 0, -1, 0, -2, 2}, 0))
}
