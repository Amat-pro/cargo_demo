package stack

import "fmt"

// 封装单调队列的方式解题
type Queue struct {
	queue []int
}

func NewQueue() *Queue {
	return &Queue{
		queue: make([]int, 0),
	}
}

func (m *Queue) Front() int {
	return m.queue[0]
}

func (m *Queue) Back() int {
	return m.queue[len(m.queue)-1]
}

func (m *Queue) Empty() bool {
	return len(m.queue) == 0
}

func (m *Queue) Push(val int) {
	for !m.Empty() && val > m.Back() {
		m.queue = m.queue[:len(m.queue)-1]
	}
	m.queue = append(m.queue, val)
}

func (m *Queue) Pop(val int) {
	if !m.Empty() && val == m.Front() {
		m.queue = m.queue[1:]
	}
}

func maxSlidingWindow(nums []int, k int) []int {
	queue := NewQueue()
	length := len(nums)
	res := make([]int, 0)
	// 先将前k个元素放入队列
	for i := 0; i < k; i++ {
		queue.Push(nums[i])
	}
	// 记录前k个元素的最大值
	res = append(res, queue.Front())

	for i := k; i < length; i++ {
		// 滑动窗口移除最前面的元素
		queue.Pop(nums[i-k])
		// 滑动窗口添加最后面的元素
		queue.Push(nums[i])
		// 记录最大值
		res = append(res, queue.Front())
		fmt.Println("--> len: ", len(queue.queue))
	}
	return res
}
