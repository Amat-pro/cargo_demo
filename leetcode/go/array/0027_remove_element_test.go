package array

import (
	"fmt"
	"testing"
)

func Test_removeElement(t *testing.T) {
	fmt.Println("==> ", removeElement([]int{}, 9))
	fmt.Println("==> ", removeElement([]int{9}, 9))
	fmt.Println("==> ", removeElement([]int{0, 1, 2, 3, 3, 0, 4, 2, 2}, 2))
}
