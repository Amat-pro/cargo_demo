package bags

import (
	"fmt"
	"testing"
)

func Test_findTargetSumWays(t *testing.T) {
	// 1
	fmt.Println("==> ", findTargetSumWays([]int{1}, 1))
	// 5
	fmt.Println("==> ", findTargetSumWays([]int{1, 1, 1, 1, 1}, 3))
}
