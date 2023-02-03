package dp

import (
	"fmt"
	"testing"
)

func Test_uniquePathsII(t *testing.T) {
	// 2
	fmt.Println("==> ", uniquePathsII([][]int{{0, 0, 0}, {0, 1, 0}, {0, 0, 0}}))
	// 1
	fmt.Println("==> ", uniquePathsII([][]int{{0, 1}, {0, 0}}))
}
