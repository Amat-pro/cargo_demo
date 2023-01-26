package string

func reverseWords(s string) string {
	b := []byte(s)

	if len(b) <= 0 {
		return s
	}

	// 移除多余的空格
	b = removeRedundantSpaces(b)

	// 反转整个字符串
	reverse_0151(b, 0, len(b)-1)
	// 反转每个单词
	i := 0
	for i < len(b) {
		// j记录end
		j := i
		for ; j < len(b) && b[j] != ' '; j++ {
		}
		reverse_0151(b, i, j-1)
		i = j + 1
	}

	return string(b)
}

func reverse_0151(b []byte, left, right int) {
	for left < right {
		b[left], b[right] = b[right], b[left]
		left++
		right--
	}
}

func removeRedundantSpaces(b []byte) []byte {
	// 快慢指针
	// 与027相似，027是使用了末尾元素替换，这里有顺序要求，所以会将相应的元素向前移
	slow, fast := 0, 0
	length := len(b)
	// 1.移除前面冗余的空格
	// 2.移除中间冗余的空格
	// 3.移除末尾冗余的空格
	for fast < length && b[fast] == ' ' {
		fast++
	}

	for fast < length {
		if fast-1 > 0 && b[fast] == b[fast-1] && b[fast] == ' ' {
			fast++
			continue
		}

		b[slow] = b[fast]
		fast++
		slow++
	}

	if slow-1 > 0 && b[slow-1] == ' ' {
		return b[:slow-1]
	} else {
		return b[:slow]
	}

}
