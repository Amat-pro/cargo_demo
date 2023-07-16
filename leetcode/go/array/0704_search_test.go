package array

import (
	"fmt"
	"testing"
)

func Test_search(t *testing.T) {
	fmt.Println("==>", search([]int{}, 9))
	fmt.Println("==>", search([]int{9}, 3))
	fmt.Println("==>", search([]int{9}, 9))
	fmt.Println("==>", search([]int{1, 2, 3, 4, 7, 9, 10}, 3))
}

func Test_whileSearch(t *testing.T) {
	fmt.Println("==>", whileSearch([]int{}, 9))
	fmt.Println("==>", whileSearch([]int{9}, 3))
	fmt.Println("==>", whileSearch([]int{9}, 9))
	fmt.Println("==>", whileSearch([]int{1, 2, 3, 4, 7, 9, 10}, 3))
}
