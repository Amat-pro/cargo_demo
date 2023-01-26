package hash

import (
	"fmt"
	"testing"
)

func Test_canConstruct(t *testing.T) {
	// false
	fmt.Println("==> ", canConstruct("a", "b"))
	// false
	fmt.Println("==> ", canConstruct("aa", "ab"))
	// true
	fmt.Println("==> ", canConstruct("aa", "aab"))
}
