package string

func strStr(haystack, needle string) int {

	l := len(needle)

	if l == 0 {
		return 0
	}

	next := getNext(needle)
	j := 0
	for i := 0; i < len(haystack); i++ {
		if haystack[i] == needle[j] {
			j++
			if j == l {
				return i - l + 1
			}
		}

		for j > 0 && haystack[i] != needle[j] {
			j = next[j-1] // 回退到next[j-1]位置重新匹配
		}

	}

	return -1
}

func getNext(s string) []int {
	b := []byte(s)

	l := len(b)
	if l <= 0 {
		return []int{}
	}

	next := make([]int, l)
	j := 0 // [0,i]前缀末尾位置， [0,i]最长相等前后缀长度
	next[0] = j
	i := 1 // [0,i]后缀末尾位置

	for i < l {
		if b[i] == b[j] {
			j++
		} else {
			for j > 0 {
				j--
				if b[i] == b[j] {
					break
				}
			}
		}
		next[i] = j
		i++
	}

	return next

}
