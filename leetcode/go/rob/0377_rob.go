package rob

// 二叉树
func rob_iii(root *TreeNode) int {

	values := robTree(root)
	return max(values[0], values[1])

}

func robTree(root *TreeNode) [2]int {
	// index 0:偷当前节点  1:不偷当前节点
	dp := [2]int{0, 0}

	// 中序遍历

	if root == nil {
		return dp
	}

	left := robTree(root.Left)
	right := robTree(root.Right)

	// 偷当前节点
	root_0 := root.Val + left[1] + right[1]
	// 不偷当前节点
	root_1 := max(left[0], left[1]) + max(right[0], right[1])

	dp[0] = root_0
	dp[1] = root_1

	return dp

}

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}
