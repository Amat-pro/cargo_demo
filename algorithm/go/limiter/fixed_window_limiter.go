package limiter

import (
	"sync"
	"time"
)

// FixedWindowLimiter 固定窗口限流器
type FixedWindowLimiter struct {
	limit int           // 请求上限
	size  time.Duration // 窗口大小

	counter int // 计数器

	windowStartTime time.Time // 窗口开始时间

	mutex sync.Mutex
}

func NewFixedWindowLimiter(limit int, size time.Duration) *FixedWindowLimiter {
	return &FixedWindowLimiter{
		limit: limit,
		size:  size,
		mutex: sync.Mutex{},
	}
}

func (limiter *FixedWindowLimiter) Acquire() bool {

	limiter.mutex.Lock()
	defer limiter.mutex.Unlock()

	now := time.Now()
	// 当前时间窗口过期, 需要新开一个窗口
	if now.Sub(limiter.windowStartTime) > limiter.size {
		limiter.counter = 0
		limiter.windowStartTime = now
	}

	// 判断请求数是否达到当前窗口上限
	if limiter.counter >= limiter.limit {
		return false
	} else {
		limiter.counter++
		return true
	}
}
