package stock

import (
	"fmt"
	"testing"
)

func Test_maxProfit_i(t *testing.T) {
	// 5
	fmt.Println("==> ", maxProfit_i([]int{7, 1, 5, 3, 6, 4}))
	// 0
	fmt.Println("==> ", maxProfit_i([]int{7, 6, 4, 3, 1}))
}
