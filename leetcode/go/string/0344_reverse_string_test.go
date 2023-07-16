package string

import (
	"fmt"
	"testing"
)

func Test_reverseString(t *testing.T) {
	fmt.Println("==>> ", string(reverseString([]byte("hello"))))
}
