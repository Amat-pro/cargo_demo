/// leetcode 005
///
/// [5.最长回文字串](https://leetcode.cn/problems/longest-palindromic-substring/)
///
///

// 使用动态规划解题
pub fn longest_palindromic_substring(s: String) -> String {
    let len = s.len();
    if len < 2 {
        return s;
    }

    let mut len_max = 1;
    let mut index_start = 0;

    // 默认设置为false
    let table: &mut Vec<Vec<bool>> = &mut (vec![vec![false; len]; len]);

    // 将对角线位置设置为true
    for i in 0..len {
        table[i][i] = true;
    }

    let chars = s.as_bytes();
    // 下面进行填表格
    for j in 1..len {
        for i in 0..j {  // i 到 j-1结束
            if chars[i] != chars[j] {
                table[i][j] = false;
            } else {
                // 判断一下dp[i, j]之间的元素数量，
                // 如果j - i 之间的数量是 2 或 3 则不需要执行dp[i+1, j-1]是否是回文字符串的判断
                if j - i + 1 < 4 { // j - i < 3
                    table[i][j] = true;
                } else {
                    table[i][j] = table[i + 1][j - 1];
                }
            }

            let element_count = j - i + 1;
            if table[i][j] && element_count > len_max {
                len_max = element_count;
                index_start = i;
            }
        }
    }

    // end - start + 1 = count
    // end = count + start - 1
    let index_end = len_max + index_start - 1;
    let mut new_chars: Vec<u8> = vec![];
    for i in index_start..index_end + 1 {
        new_chars.push(chars[i]);
    }

    String::from_utf8(new_chars).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_longest_palindromic_substring() {
        let str = "".to_string();
        assert_eq!(longest_palindromic_substring(str), "");

        let str1 = "db".to_string();
        assert_eq!(longest_palindromic_substring(str1), "d");

        let str2 = "babad".to_string();
        assert_eq!(longest_palindromic_substring(str2), "bab");
    }
}



























