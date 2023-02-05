package bags

import (
	"fmt"
	"math"
	"testing"
)

func Test_coinsChange_i(t *testing.T) {
	// 3
	fmt.Println("==> ", coinsChange_i([]int{1, 2, 5}, 11))
	// -1
	fmt.Println("==> ", coinsChange_i([]int{2}, 3))
	// 0
	fmt.Println("==> ", coinsChange_i([]int{1}, 0))
	// 1
	fmt.Println("==> ", coinsChange_i([]int{1}, 1))
	// 2
	fmt.Println("==> ", coinsChange_i([]int{1}, 2))

	fmt.Println(math.MaxInt)
}
