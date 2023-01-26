package string

func replaceSpace(s string) string {
	b := []byte(s)
	length := len(b)

	spaceCount := 0
	for _, v := range b {
		if v == ' ' {
			spaceCount++
		}
	}

	// 扩展数组 每个空格需要增加两个位置！！
	resizeCount := spaceCount * 2
	temp := make([]byte, resizeCount)
	b = append(b, temp...)
	left := length - 1
	right := len(b) - 1

	for left >= 0 {
		if b[left] != ' ' {
			b[right] = b[left]
			left--
			right--
		} else {
			b[right] = '0'
			b[right-1] = '2'
			b[right-2] = '%'

			left--
			right -= 3
		}
	}

	return string(b)
}
