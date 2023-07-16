package main

// A:起始柱
// B:辅助柱
// C:目标柱
func hanota(A []int, B []int, C []int) []int {

	var move func(n int, a *[]int, b *[]int, c *[]int)
	move = func(n int, a *[]int, b *[]int, c *[]int) {
		if n == 1 {
			*c = append(*c, (*a)[len(*a)-1])
			*a = (*a)[:len(*a)-1]
			return
		} else {
			// 将n-1个盘子从A移动到B
			move(n-1, a, c, b)
			// 将1个盘子从A移动到C
			move(1, a, b, c)
			// 将n-1个盘子从B移动到C
			move(n-1, b, a, c)
		}
	}

	n := len(A)
	if n == 0 {
		return A
	}

	move(n, &A, &B, &C)
	return C

}
