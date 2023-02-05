package stock

import (
	"fmt"
	"testing"
)

func Test_maxProfitWithCommission(t *testing.T) {
	// 8
	fmt.Println("==> ", maxProfitWithCommission([]int{1,3,2,8,4,9}, 2))
	// 6
	fmt.Println("==> ", maxProfitWithCommission([]int{1,3,7,5,10,3}, 3))
}
