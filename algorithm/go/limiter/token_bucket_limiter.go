package limiter

import (
	"fmt"
	"sync"
	"time"
)

// TokenBucketLimiter 令牌桶限流器
type TokenBucketLimiter struct {
	tokenCapacity     int       // 令牌桶容量
	currentTokenCount int       // 当前令牌数量
	tokenRate         int       // 令牌发放速率 n/秒
	lastRequestTime   time.Time // 上次请求时间

	mutex sync.Mutex
}

func NewTokenBucketLimiter(limit int, tokenRate int) *TokenBucketLimiter {
	return &TokenBucketLimiter{
		tokenCapacity:     limit,
		currentTokenCount: 0,
		tokenRate:         tokenRate,
		lastRequestTime:   time.Now(),

		mutex: sync.Mutex{},
	}
}

func (limiter *TokenBucketLimiter) Acquire() bool {

	now := time.Now()

	// 计算距离上次请求的间隔时间
	interval := int(now.Sub(limiter.lastRequestTime).Seconds())
	if interval > 0 {
		fmt.Println("==> interval: ", interval)
		// 计算令牌数量 (这里以上次请求时间和当前请求时间作为依据)
		limiter.currentTokenCount = min(limiter.tokenCapacity, limiter.currentTokenCount+limiter.tokenRate*interval)
		limiter.lastRequestTime = now
	}

	// 没有得到令牌
	if limiter.currentTokenCount == 0 {
		return false
	}

	// 得到令牌
	limiter.currentTokenCount--
	return true

}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}
