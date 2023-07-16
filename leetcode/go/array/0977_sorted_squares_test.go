package array

import (
	"fmt"
	"testing"
)

func Test_sortedSquares(t *testing.T) {
	fmt.Println("===>> ", sortedSquares([]int{}))
	fmt.Println("===>> ", sortedSquares([]int{4}))
	fmt.Println("===>> ", sortedSquares([]int{-4, -1, 0, 3, 10}))
	fmt.Println("===>> ", sortedSquares([]int{-7, -3, 2, 3, 11}))
}
