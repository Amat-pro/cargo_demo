package backtrack

import (
	"fmt"
	"testing"
)

func Test_combine(t *testing.T) {
	// [[1234]]
	fmt.Println("==> ", combine(4, 4))
	// [[]]
	fmt.Println("==> ", combine(4, 5))
}
