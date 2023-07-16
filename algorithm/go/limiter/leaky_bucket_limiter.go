package limiter

import (
	"sync"
	"time"
)

// LeakyBucketLimiter 漏桶限流器
type LeakyBucketLimiter struct {
	capacity int // 容量

	currentLevel int // 当前水位

	leakSpeed int // 水流速度 leakSpeed滴/秒

	lastRequestTime time.Time // 上次请求时间

	mutex sync.Mutex
}

func NewLeakyBucketLimiter(limit int, leakSpeed int) *LeakyBucketLimiter {
	return &LeakyBucketLimiter{
		capacity:        limit,
		currentLevel:    0,
		leakSpeed:       leakSpeed,
		lastRequestTime: time.Now(),
		mutex:           sync.Mutex{},
	}
}

func (limiter *LeakyBucketLimiter) Acquire() bool {
	limiter.mutex.Lock()
	defer limiter.mutex.Unlock()

	now := time.Now()
	// 计算距离上次放水间隔时间 (根据本次请求和上次请求时间计算)
	interval := int(now.Sub(limiter.lastRequestTime).Seconds())
	if interval > 0 {
		// 计算当前水位
		limiter.currentLevel = max(0, limiter.currentLevel-(interval*limiter.leakSpeed))
		limiter.lastRequestTime = now
	}

	if limiter.currentLevel+1 > limiter.capacity {
		return false
	}

	limiter.currentLevel++
	return true

}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}
