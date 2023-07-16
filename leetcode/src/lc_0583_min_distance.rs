// 0583 两个字符串的删除操作

fn min_distance(word1: String, word2: String) -> i32 {
    let m = word1.len();
    let n = word2.len();

    let w1 = word1.as_bytes();
    let w2 = word2.as_bytes();

    // 定义dp
    let mut dp = vec![vec![0; n + 1]; m + 1];

    // 初始化dp
    for i in 0..=m {
        dp[i][0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }

    // 遍历
    for i in 1..=m {
        for j in 1..=n {
            // 递推公式
            if w1[i - 1] == w2[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = (dp[i][j - 1] + 1)
                    .min(dp[i - 1][j] + 1)
                    .min(dp[i - 1][j - 1] + 2);
            }
        }
    }

    dp[m][n] as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_distance() {
        assert_eq!(min_distance("sea".to_string(), "eat".to_string()), 2);
        assert_eq!(min_distance("leetcode".to_string(), "etco".to_string()), 4);
    }
}
