package bags

import (
	"fmt"
	"testing"
)

func Test_coinsChange(t *testing.T) {
	// 4
	fmt.Println("==> ", coinsChange([]int{1, 2, 5}, 5))
	// 0
	fmt.Println("==> ", coinsChange([]int{2}, 3))
	// 1
	fmt.Println("==> ", coinsChange([]int{10}, 10))
}
