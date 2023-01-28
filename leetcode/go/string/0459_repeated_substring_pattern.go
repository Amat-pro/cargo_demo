package string

func repeatedSubstringPattern(s string) bool {

	l := len(s)
	j := 0 // j: [0, i]
	z := 0
	for i := 0; i < l-1; i++ {
		// i 表示子串结束位置 字串：[0, i]
		j = 0
		z = 0
		for z < l {
			if s[z] != s[j] {
				break
			}

			j++
			if j > i {
				j = 0
			}

			z++
			if z == l && j == 0 {
				return true
			}
		}
	}

	return false
}

func repeatedSubstringPatternKMP(s string) bool {
	l := len(s)
	if l == 0 {
		return false
	}

	next := getNext_0459(s)
	if next[l-1] == 0 {
		return false
	}

	// 最长相等前后缀不包含的那部分子字符串（重复子字符串）的长度l - next[l-1]
	if l%(l-next[l-1]) == 0 {
		return true
	} else {
		return false
	}
}

func getNext_0459(s string) []int {

	i := 1 // 后缀末尾位置
	j := 0 // 前缀末尾位置(也代表i及i之前字符串最长相等前缀数)

	b := []byte(s)
	l := len(b)
	next := make([]int, l)
	next[0] = j

	for i < l {
		if b[i] == b[j] {
			j++
		} else {
			for j > 0 && b[i] != b[j] {
				j--
			}
		}

		next[i] = j
		i++
	}

	return next
}
