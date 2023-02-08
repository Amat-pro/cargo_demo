// 0392 判断子序列

// 双指针解法
fn is_sub_sequence(s: String, t: String) -> bool {
    let mut left = 0;
    let mut right = 0;

    let s1 = s.as_bytes();
    let t1 = s.as_bytes();

    while left < s.len() && right < t.len() {
        if s1[left] == t1[right] {
            left += 1;
            right += 1;
            continue;
        }
        right += 1;
    }

    left == s.len()
}

// 动态规划解法
fn dp_is_sub_sequence(s: String, t: String) -> bool {
    let m = s.len();
    let n = t.len();

    let s1 = s.as_bytes();
    let t1 = t.as_bytes();

    // 定义dp并初始化为0
    let mut dp = vec![vec![0; n+1]; m+1];

    // 遍历
    for i in 1..=m {
        for j in 1..n {
            if s1[i - 1] == t1[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i][j - 1];
            }
        }
    }

    dp[m][n] == m
}
