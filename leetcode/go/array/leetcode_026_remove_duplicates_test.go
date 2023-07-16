package array

import (
	"fmt"
	"testing"
)

func Test_removeDuplicatesV2(t *testing.T) {
	fmt.Println("===>>> ", removeDuplicates([]int{1, 2, 3, 3}))
	fmt.Println("===>>> ", removeDuplicates([]int{1, 1}))
}
