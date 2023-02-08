// 0516 最长会问子序列

fn longest_palindrome_subsequence(s: String) -> i32 {
    // 定义dp
    let mut dp = vec![vec![0; s.len()]; s.len()];
    // 初始化
    for i in 0..s.len() {
        dp[i][i] = 1;
    }

    let s1 = s.as_bytes();

    // 遍历
    let mut z: i32 = (s.len() - 1) as i32;
    let mut i: usize;
    while z >= 0 {
        i = z as usize;
        for j in i + 1..s.len() {
            // 递推公式
            if s1[i] == s1[j] {
                dp[i][j] = dp[i + 1][j - 1] + 2;
            } else {
                dp[i][j] = dp[i][j - 1].max(dp[i + 1][j])
            }
        }
        z -= 1;
    }

    dp[0][s.len() - 1]
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_longest_palindrome_subsequence() {
        assert_eq!(longest_palindrome_subsequence("bbbab".to_string()), 4);
        assert_eq!(longest_palindrome_subsequence("cbbd".to_string()), 2);
    }
}
