// 1143 最长公共子序列

fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    // 定义dp 初始化为0
    let mut dp = vec![vec![0; text2.len() + 1]; text1.len() + 1];

    let s1 = text1.as_bytes();
    let s2 = text2.as_bytes();
    // 遍历
    for i in 1..=text1.len() {
        for j in 1..=text2.len() {
            if s1[i - 1] == s2[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }

    dp[text1.len()][text2.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_subsequence() {
        assert_eq!(
            longest_common_subsequence("abcde".to_string(), "ace".to_string()),
            3
        );
        assert_eq!(
            longest_common_subsequence("abc".to_string(), "abc".to_string()),
            3
        );
        assert_eq!(
            longest_common_subsequence("abc".to_string(), "def".to_string()),
            0
        );
    }
}
