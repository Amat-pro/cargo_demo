/// leetcode 010
///
/// [10.正则表达式匹配](https://leetcode.cn/problems/regular-expression-matching/)
///
pub fn is_match(s: String, p: String) -> bool {
    let m = s.len();
    let n = p.len();

    let s_chars = s.as_bytes();
    let p_chars = p.as_bytes();

    // 最终是求f[m][n]
    let mut f: Vec<Vec<bool>> = vec![vec![false; n + 1]; m + 1];
    f[0][0] = true;

    // i, j表示s前i个和p前j个  对应的元素则为s_chars[i-1], p_chars[j-1]
    // 注意这里的i是0开始的，比如 s="" p=a*的情况
    for i in 0..m + 1 {
        for j in 1..n + 1 {
            if p_chars[j - 1] == b'*' {
                // s的第i个 和 p的第j-1个 字符是否相等
                // // 下面的两个条件取||
                // if matches(&s_chars, &p_chars, i, j - 1) {
                //     // 相等则是匹配0个
                // } else {
                //     // 不相等是匹配1个或多个的情况
                // }
                // 当p的第j个字符是*时， j >= 2恒成立
                f[i][j] = f[i][j - 2];
                if matches(&s_chars, &p_chars, i, j - 1) {
                    f[i][j] = f[i][j] || f[i - 1][j];
                }
            } else {
                if matches(&s_chars, &p_chars, i, j) {
                    f[i][j] = f[i - 1][j - 1];
                }
            }
        }
    }

    f[m][n]
}

fn matches(s: &[u8], p: &[u8], i: usize, j: usize) -> bool {
    if i == 0 {
        return false;
    }

    if p[j - 1] == b'.' {
        return true;
    }
    return s[i - 1] == p[j - 1];
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_match() {
        let s0 = "aa".to_string();
        let p0 = "a".to_string();
        assert!(!is_match(s0, p0));

        let s1 = "aa".to_string();
        let p1 = "a*".to_string();
        assert!(is_match(s1, p1));


        let s2 = "ab".to_string();
        let p2 = ".*".to_string();
        assert!(is_match(s2, p2));
    }
}

