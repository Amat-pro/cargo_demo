package dp

import (
	"fmt"
	"testing"
)

func Test_integerBreak(t *testing.T) {
	// 1
	fmt.Println("==> ", integerBreak(2))
	// 36
	fmt.Println("==> ", integerBreak(10))
}

func Test_max(t *testing.T) {
	type args struct {
		a int
		b int
	}
	tests := []struct {
		name string
		args args
		want int
	}{
		// TODO: Add test cases.
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := max(tt.args.a, tt.args.b); got != tt.want {
				t.Errorf("max() = %v, want %v", got, tt.want)
			}
		})
	}
}
