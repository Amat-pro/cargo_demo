package stock

import (
	"fmt"
	"testing"
)

func Test_maxProfit_ii(t *testing.T) {
	// 7
	fmt.Println("==> ", maxProfit_ii([]int{7, 1, 5, 3, 6, 4}))
	// 4
	fmt.Println("==> ", maxProfit_ii([]int{1, 2, 3, 4, 5}))
	// 0
	fmt.Println("==> ", maxProfit_ii([]int{7, 6, 4, 3, 1}))
}
