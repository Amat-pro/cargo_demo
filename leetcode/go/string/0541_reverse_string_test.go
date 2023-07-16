package string

import (
	"fmt"
	"testing"
)

func Test_reverseStr(t *testing.T) {

	fmt.Println("==> ", "bacdfeg")
	fmt.Println("==> ", reverseStr("abcdefg", 2))
}
