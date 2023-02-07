package limiter

import (
	"fmt"
	"testing"
	"time"
)

func TestSlideWindowLimiter_Acquire(t *testing.T) {
	limiter, _ := NewSlideWindowLimiter(2, time.Second, 5)

	timer := time.NewTimer(time.Second * 6)
	ticker := time.NewTicker(300 * time.Millisecond)

	counter := 0
	successCount := 0

	for {
		select {
		case <-ticker.C:
			counter++
			success := limiter.Acquire()
			if success {
				successCount++
			}
		case <-timer.C:
			ticker.Stop()
			fmt.Println("Done")
			fmt.Println("count: ", counter)
			fmt.Println("successCount: ", successCount) // 12左右
			return
		}
	}

}
