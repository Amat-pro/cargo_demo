package array

import (
	"fmt"
	"testing"
)

func Test_min_sub_array_len(t *testing.T) {
	fmt.Println("==>> ", min_sub_array_len([]int{}, 3))                 // 0
	fmt.Println("==>> ", min_sub_array_len([]int{3}, 3))                // 1
	fmt.Println("==>> ", min_sub_array_len([]int{2, 3, 1, 2, 4, 3}, 7)) // 2
	fmt.Println("==>> ", min_sub_array_len([]int{1, 1, 1, 20}, 5))      // 0
}

func Test_min_sub_array_len_v2(t *testing.T) {
	fmt.Println("==>> ", min_sub_array_len_v2([]int{}, 3))                 // 0
	fmt.Println("==>> ", min_sub_array_len_v2([]int{3}, 3))                // 1
	fmt.Println("==>> ", min_sub_array_len_v2([]int{2, 3, 1, 2, 4, 3}, 7)) // 2
	fmt.Println("==>> ", min_sub_array_len_v2([]int{1, 1, 1, 20}, 5))      // 0
	fmt.Println("==>> ", min_sub_array_len_v2([]int{90}, 9))      // 0
}
