package hash

import (
	"fmt"
	"testing"
)

func Test_intersection(t *testing.T) {
	fmt.Println("==> ", intersection([]int{1, 2, 2, 1}, []int{2, 2}))
	fmt.Println("==> ", intersection([]int{4, 9, 5}, []int{9, 4, 9, 8, 4}))
}
