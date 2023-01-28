package stack

import (
	"fmt"
	"testing"
)

func Test_maxSlidingWindow(t *testing.T) {
	fmt.Println("==> ", maxSlidingWindow([]int{1, 3, -1, -3, 5, 3, 6, 7}, 3))
}
