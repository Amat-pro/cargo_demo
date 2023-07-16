package string

func reverseLeftWords(s string, n int) string {
	b := []byte(s)

	l := len(b)
	reverse_0058(b, 0, n-1)
	reverse_0058(b, n, l-1)
	reverse_0058(b, 0, l-1)
	return string(b)
}

func reverse_0058(b []byte, left, right int) {
	for left < right {
		b[left], b[right] = b[right], b[left]
		left++
		right--
	}
}
