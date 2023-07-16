package dp

import (
	"fmt"
	"testing"
)

func Test_minCostClimbStairs(t *testing.T) {
	// 15
	fmt.Println("==> ", minCostClimbStairs([]int{10, 15, 20}))
	// 6
	fmt.Println("==> ", minCostClimbStairs([]int{1, 100, 1, 1, 1, 100, 1, 1, 100, 1}))
}
