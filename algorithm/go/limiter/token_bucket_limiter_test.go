package limiter

import (
	"fmt"
	"testing"
	"time"
)

func TestTokenBucketLimiter_Acquire(t *testing.T) {
	limiter := NewTokenBucketLimiter(10, 2) // 每秒钟产生2个  capacity: 10

	timer := time.NewTimer(20 * time.Second) // 能产生40个左右
	ticker := time.NewTicker(10 * time.Millisecond)

	requestCount := 0
	successCount := 0

	for {
		select {
		case <-ticker.C:
			requestCount++
			if limiter.Acquire() {
				successCount++
			}
		case <-timer.C:
			fmt.Println("Done")
			fmt.Println("RequestCount: ", requestCount)
			fmt.Println("SuccessCount: ", successCount)
			return
		}
	}

}
