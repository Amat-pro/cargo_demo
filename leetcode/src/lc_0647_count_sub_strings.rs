// 0647 回文子串

fn count_sub_strings(s: String) -> i32 {
    // 定义dp并初始化为false
    let mut dp = vec![vec![false; s.len()]; s.len()];

    let s1 = s.as_bytes();

    let mut result = 0;

    // 遍历
    let mut z: i32 = (s.len() - 1) as i32;
    while z >= 0 {
        let i = z as usize;
        // dp[i][j]: [i, j]
        // j: [i, s.len()-1]
        for j in i..s.len() {
            // 递推公式
            if s1[i] == s1[j] {
                if i == j {
                    dp[i][j] = true;
                    result += 1;
                } else if j - i == 1 {
                    dp[i][j] = true;
                    result += 1;
                } else if j - i > 1 {
                    if dp[i + 1][j - 1] {
                        dp[i][j] = true;
                        result += 1;
                    }
                }
            }
        }
        z -= 1;
    }

    result
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_count_sub_strings() {
        assert_eq!(count_sub_strings("abc".to_string()), 3);
        assert_eq!(count_sub_strings("aaa".to_string()), 6);
    }
}
