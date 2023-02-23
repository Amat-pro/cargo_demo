package main

import (
	"fmt"
	"testing"
)

func Test_hanota(t *testing.T) {
	fmt.Println("==> ", hanota([]int{2, 1, 0}, []int{}, []int{}))
	fmt.Println("==> ", hanota([]int{1, 0}, []int{}, []int{}))
}
