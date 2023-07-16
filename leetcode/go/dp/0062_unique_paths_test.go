package dp

import (
	"fmt"
	"testing"
)

func Test_uniquePaths(t *testing.T) {
	// 28
	fmt.Println("==> ", uniquePaths(3, 7))
	// 3
	fmt.Println("==> ", uniquePaths(2, 3))
	// 28
	fmt.Println("==> ", uniquePaths(7, 3))
	// 6
	fmt.Println("==> ", uniquePaths(3, 3))
}
