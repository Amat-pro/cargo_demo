use std::collections::HashMap;

/// leetcode 003
///
/// [3.无重复字符的最长子串](https://leetcode.cn/problems/longest-substring-without-repeating-characters/)
///

pub fn length_of_longest_substring(s: String) -> i32 {
    let len = s.len() as usize;
    if len == 0 {
        return 0;
    }

    let index_end = len - 1;

    let chars = s.as_bytes();

    let mut max_length = 1;
    let mut current_length = 1;

    let mut vec_map: HashMap<u8, usize> = HashMap::new();
    let mut pt_left: usize = 0;
    let mut pt_right: usize = 0;

    loop {
        pt_right += 1;

        if pt_right > index_end {
            break;
        }

        if vec_map.get(&chars[pt_left]).is_none() {
            vec_map.insert(chars[pt_left], pt_left);
        }

        let vec_map_val_r = vec_map.get(&chars[pt_right]);
        match vec_map_val_r {
            None => {
                current_length += 1;
                if current_length > max_length {
                    max_length = current_length;
                }
                vec_map.insert(chars[pt_right], pt_right);
            }
            Some(vec_map_val) => {
                // pt_left左侧的重复 == None
                if vec_map_val < &pt_left {
                    current_length += 1;
                    if current_length > max_length {
                        max_length = current_length;
                    }
                    vec_map.insert(chars[pt_right], pt_right);
                } else {
                    pt_left = vec_map_val + 1;
                    current_length = pt_right - pt_left + 1;
                    vec_map.insert(chars[pt_right], pt_right);
                }
            }
        }
    }

    return max_length as i32;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let s1 = "abcabcbb".to_string();
        println!("s1: {}", length_of_longest_substring(s1));

        let s2 = "bbbbb".to_string();
        println!("s2: {}", length_of_longest_substring(s2));

        let s3 = "pwwkew".to_string();
        println!("s3: {}", length_of_longest_substring(s3));

        // 6
        let s4 = "sdfghjf".to_string();
        println!("s3: {}", length_of_longest_substring(s4));

        // 7
        let s5 = "sdfghjfzxc".to_string();
        println!("s3: {}", length_of_longest_substring(s5));
    }
}




















