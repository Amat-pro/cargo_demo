package limiter

import (
	"fmt"
	"testing"
	"time"
)

func TestLeakyBucketLimiter_Acquire(t *testing.T) {
	limiter := NewLeakyBucketLimiter(10, 2)

	timer := time.NewTimer(time.Second * 10) // duration: 10s  speed: 2/s  ==> success: 20(处理的)+10(待处理的)左右
	ticker := time.NewTicker(50 * time.Millisecond)

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
