package bags

import (
	"fmt"
	"testing"
)

func Test_findMaxForm(t *testing.T) {
	// 4
	fmt.Println("==> ", findMaxForm([]string{"10", "0001", "111001", "1", "0"}, 5, 3))
	// 2
	fmt.Println("==> ", findMaxForm([]string{"10", "0", "1"}, 1, 1))
}
