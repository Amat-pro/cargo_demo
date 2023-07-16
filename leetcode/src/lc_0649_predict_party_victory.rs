// 0649 Dota2参议院
fn predict_party_victory(senate: String) -> String {
    let mut binding = senate.into_bytes();
    let bytes: &mut Vec<u8> = binding.as_mut();

    let mut has_b = true;
    let mut has_d = true;
    let mut flag = 0; // 遍历到当前元素时，当前元素前对手的数量  负数表示当前R前有-flag个D 正数表示当前D前有flag个R
    while has_b && has_d {
        for i in 0..bytes.len() {
            if bytes[i] == "R".as_bytes()[0]{
                if flag < 0 {
                    // 当前R前面有-flag个D
                    bytes[i] = b'0'; // 当前R被消灭
                    has_b = false;

                    // flag = -(-flag - 1);
                    flag += 1; // flag+1 当前R被消灭，下一个D前面就会少一个R
                } else {
                    // 当前R前面没有D，当前R不被消灭
                    has_b = true;

                    flag += 1; // flag+1 当前R没被消灭，下一个D前面就会多一个R
                }
            }

            if bytes[i] == "D".as_bytes()[0] {
                // 当前D前面有flag个R
                if flag > 0 {
                    bytes[i] = b'0'; // 当前D被消灭
                    has_d = false;

                    flag -= 1; // 当前D被消灭，下一个R前面就会少一个D
                } else {
                    has_d = true;

                    // flag = -(-flag + 1);
                    flag -= 1; // 当前D没被消灭，下一个R前面就会多一个D
                }
            }
        }
    }

    if has_b {
        return "Radiant".to_string();
    }
    "Dire".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_predict_party_victory() {
        assert_eq!(predict_party_victory("RD".to_string()), "Radiant");
        assert_eq!(predict_party_victory("RDD".to_string()), "Dire");
        assert_eq!(predict_party_victory("RDDRD".to_string()), "Dire");
    }
}
