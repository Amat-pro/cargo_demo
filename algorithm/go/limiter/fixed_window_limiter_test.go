package limiter

import (
	"testing"
	"time"
)

func TestFixedWindowLimiter_Acquire(t *testing.T) {

	limiter := NewFixedWindowLimiter(10, 2*time.Second)

	successCount := 0
	for i := 0; i < 20; i++ {
		if limiter.Acquire() {
			successCount++
		}
	}

	if successCount != 10 {
		t.Error("successCount is not correct")
	}

}
