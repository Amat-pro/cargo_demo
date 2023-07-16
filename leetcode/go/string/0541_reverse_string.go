package string

func reverseStr(s string, k int) string {
	ss := []byte(s)
	length := len(ss)

	for i := 0; i < length; i += 2 * k {

		if i+k <= length {
			// == length时，恰好有k个
			reverse(ss[i : i+k])
		} else {
			reverse(ss[i:length])
		}

	}

	return string(ss)

}

func reverse(b []byte) {
	left := 0
	right := len(b) - 1
	for left < right {
		b[left], b[right] = b[right], b[left]
		left++
		right--
	}
}
