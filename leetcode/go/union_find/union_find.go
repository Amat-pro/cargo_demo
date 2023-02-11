package union_find

// 并查集简单实现
type UnionSet struct {
	Father []int
}

func (u *UnionSet) Init(n int) []int {

	// [0, 1, 2, 3] [第0个元素的祖先，第1个元素的祖先，第3个元素的祖先，...]
	father := make([]int, n)
	for i := 0; i < n; i++ {
		father[i] = i
	}

	u.Father = father

	return u.Father

}

func (u *UnionSet) Union(i, j int) {

	iFather := u.Find(i)        // 找到i的祖先节点
	jFather := u.Find(j)        // 找到j的祖先节点
	u.Father[iFather] = jFather // i的祖先指向j的祖先

	//                               iFather -> jFather
	// 比如 Father  [..  m,..   n,..  m1,..      n1]
	//     Index        i,..   j,..  m,..       n

}

func (u *UnionSet) Find(i int) int {
	if i == u.Father[i] {
		return i
	}

	u.Father[i] = u.Find(u.Father[i]) // 进行路径压缩

	// 比如 Father [1,  2,  3,  4,  4]
	//     Index   0   1   2   3   4
	// 执行Find(0)查找第0个元素的祖先节点
	// 最终整体会变为:
	//     Father [4,  4,  4,  4,  4]
	//     Index   0   1   2   3   4
	return u.Father[i]
}
