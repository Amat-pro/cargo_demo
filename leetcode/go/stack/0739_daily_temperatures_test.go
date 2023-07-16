package stack

import (
	"fmt"
	"testing"
)

func Test_dailyTemperatures_v1(t *testing.T) {
	fmt.Println("==> ", dailyTemperatures_v1([]int{73, 74, 75, 71, 69, 72, 76, 73}))
	fmt.Println("==> ", dailyTemperatures_v2([]int{73, 74, 75, 71, 69, 72, 76, 73}))
}
