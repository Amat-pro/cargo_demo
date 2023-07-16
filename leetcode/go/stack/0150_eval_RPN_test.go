package stack

import (
	"fmt"
	"testing"
)

func Test_evalRPN(t *testing.T) {
	// 22
	fmt.Println("==> ", evalRPN([]string{"10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"}))
}
