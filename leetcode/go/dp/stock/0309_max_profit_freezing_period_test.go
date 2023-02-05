package stock

import (
	"fmt"
	"testing"
)

func Test_maxProfitWithFreezingPeriod(t *testing.T) {
	// 3
	fmt.Println("==> ", maxProfitWithFreezingPeriod([]int{1, 2, 3, 0, 2}))
}
