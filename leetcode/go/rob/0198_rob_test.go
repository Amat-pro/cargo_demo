package rob

import (
	"fmt"
	"testing"
)

func Test_rob(t *testing.T) {
	// 4
	fmt.Println("==> ", rob([]int{1, 2, 3, 1}))
	// 12
	fmt.Println("==> ", rob([]int{2, 7, 9, 3, 1}))
}
