package stack

import (
	"sort"
)

// 方法一：小顶堆  O(n*logk)
// func topKFrequent(nums []int, k int) []int {
// 	map_num := map[int]int{}
// 	//记录每个元素出现的次数
// 	for _, item := range nums {
// 		map_num[item]++
// 	}
// 	h := &IHeap{}
// 	heap.Init(h)
// 	//所有元素入堆，堆的长度为k
// 	for key, value := range map_num {
// 		heap.Push(h, [2]int{key, value})
// 		if h.Len() > k {
// 			heap.Pop(h)
// 		}
// 	}
// 	res := make([]int, k)
// 	//按顺序返回堆中的元素
// 	for i := 0; i < k; i++ {
// 		res[k-i-1] = heap.Pop(h).([2]int)[0]
// 	}
// 	return res
// }

// 方法二:利用O(nlogn)排序
func topKFrequent_v2(nums []int, k int) []int {
	ans := []int{}
	map_num := map[int]int{}
	for _, item := range nums {
		map_num[item]++
	}
	for key, _ := range map_num {
		ans = append(ans, key)
	}
	//核心思想：排序
	//可以不用包函数，自己实现快排
	sort.Slice(ans, func(a, b int) bool {
		return map_num[ans[a]] > map_num[ans[b]]
	})
	return ans[:k]
}
