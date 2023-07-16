package dp

import (
	"fmt"
	"testing"
)

func Test_fib(t *testing.T) {
	// 2 3 5 8
	fmt.Println("==> ", fib(3))
	fmt.Println("==> ", fib(4))
	fmt.Println("==> ", fib(5))
	fmt.Println("==> ", fib(6))
}
