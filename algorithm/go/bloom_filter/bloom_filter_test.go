package bloom_filter

import (
	"testing"

	"github.com/twmb/murmur3"
)

func TestMurmurNew64(t *testing.T) {
	murmur := murmur3.New64()
	_, _ = murmur.Write([]byte("hello, world!"))
	sum := murmur.Sum64()
	t.Log("sum", sum)
}

func TestNew(t *testing.T) {
	// https://hur.st/bloomfilter/
	// n = 4000
	// p = 0.0000001 (1 in 9994297)
	// m = 134191 (16.38KiB)
	// k = 23
}
