package bloom_filter

import (
	"fmt"
	"math"

	"github.com/bits-and-blooms/bitset"
	"github.com/spaolacci/murmur3"
)

// 使用murmur3 非加密哈希算法
type BloomFilter struct {
	m uint64 // 数组大小
	k uint32 // hash次数
	b *bitset.BitSet
}

func New(m uint64, k uint32) *BloomFilter {
	return &BloomFilter{
		m: m,
		k: k,
		b: bitset.New(uint(m)),
	}
}

// 最佳配置
// https://hur.st/bloomfilter/
// n代表最多个不同元素
// p代表假阳率
// m代表位图长度
// k代表hash函数的个数
func NewWithExpected(n uint, p float64) *BloomFilter {
	return New(uint64(math.Ceil(-1*float64(n)*math.Log(p)/math.Pow(math.Log(2), 2))), uint32(math.Ceil(math.Log(2)*float64(n)/float64(n))))
}

// panic if locate out of range
func (f *BloomFilter) Add(data []byte) {
	for i := uint32(0); i < f.k; i++ {
		l := f.locate(data, i)
		// if l >= f.m
		if f.m <= uint64(l) {
			panic(fmt.Sprintf("locate out of range, cap: %d  locate: %d", f.m, l))
		}
		f.b.Set(l)
	}
}

// panic if locate out of range
func (f *BloomFilter) Exist(data []byte) bool {
	for i := uint32(0); i < f.k; i++ {
		l := f.locate(data, i)
		if f.m <= uint64(l) {
			return false
		}
		if !f.b.Test(l) {
			return false
		}
	}
	return true
}

func (f *BloomFilter) locate(data []byte, seed uint32) uint {
	h := murmur3.New64WithSeed(seed)
	_, _ = h.Write(data)
	return uint(h.Sum64())
}
