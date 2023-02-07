package limiter

import (
	"errors"
	"sync"
	"time"
)

// 滑动窗口限流器
type SlideWindowLimiter struct {
	limit            int           // 请求上限
	windowSize       time.Duration // 窗口大小
	smallWindowCount int64         // 小窗口数量
	smallWindowSize  int64         // 小窗口大小(milliseconds)

	smallWindowCounters []int // 小窗口计数器集合

	windowStartTime time.Time // 窗口开始时间

	mutex sync.Mutex
}

func NewSlideWindowLimiter(limit int, windowSize time.Duration, smallWindowCount int64) (*SlideWindowLimiter, error) {

	if int64(windowSize.Milliseconds())%smallWindowCount != 0 {
		return nil, errors.New("windows cannot be divided into integer small windows")
	}

	smallWindowSize := windowSize.Milliseconds() / smallWindowCount

	return &SlideWindowLimiter{
		limit:               limit,
		windowSize:          windowSize,
		smallWindowCount:    smallWindowCount,
		smallWindowSize:     smallWindowSize,
		smallWindowCounters: make([]int, smallWindowCount),
		windowStartTime:     time.Now(),
		mutex:               sync.Mutex{},
	}, nil
}

func (limiter *SlideWindowLimiter) Acquire() bool {
	limiter.mutex.Lock()
	defer limiter.mutex.Unlock()

	now := time.Now()
	// 根据需要滑动窗口
	limiter.slideWindow(now)

	// 计算窗口请求数量
	count := 0
	for _, counter := range limiter.smallWindowCounters {
		count += counter
	}

	if count >= limiter.limit {
		return false
	}

	position := limiter.counterPosition(now)
	limiter.smallWindowCounters[position]++

	return true
}

func (limiter *SlideWindowLimiter) counterPosition(requestTime time.Time) int {
	subTimeDuration := requestTime.Sub(limiter.windowStartTime)

	if subTimeDuration >= limiter.windowSize+time.Duration(limiter.smallWindowSize)*time.Millisecond {
		// 重置
		limiter.windowStartTime = requestTime
		limiter.smallWindowCounters = make([]int, limiter.smallWindowCount)
		return 0
	}

	return int(subTimeDuration.Milliseconds() / limiter.smallWindowSize)

}

func (limiter *SlideWindowLimiter) slideWindow(requestTime time.Time) {
	counterPosition := limiter.counterPosition(requestTime)
	// 滑动窗口
	if counterPosition == int(limiter.smallWindowCount) {
		limiter.windowStartTime = limiter.windowStartTime.Add(time.Duration(limiter.smallWindowSize) * time.Millisecond)
		limiter.smallWindowCounters = limiter.smallWindowCounters[1:]
		limiter.smallWindowCounters = append(limiter.smallWindowCounters, 0)
	}

}
