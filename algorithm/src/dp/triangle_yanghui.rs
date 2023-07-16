/// # DP-杨辉三角
/// ## 给定一个非负整数 numRows，生成「杨辉三角」的前 numRows 行在「杨辉三角」中，每个数是它左上方和右上方的数的和
/// ## [杨辉三角示例图](https://camo.githubusercontent.com/e4068d60ab3dae1d6f4a875f1e04c58474ed06b764b7f9a966331a0a5c6cfe78/68747470733a2f2f666173746c792e6a7364656c6976722e6e65742f67682f646f6f63732f6c656574636f6465406d61696e2f736f6c7574696f6e2f303130302d303139392f303131382e50617363616c25323773253230547269616e676c652f696d616765732f313632363932373334352d445a6d6678422d50617363616c547269616e676c65416e696d61746564322e676966)
/// &nbsp;
/// ## 提示
/// * 1 <= numRows <= 30

pub fn generate_triangle_yanghui(num_rows: u64) -> Vec<Vec<u64>> {
    if num_rows == 0 {
        return vec![];
    }

    if num_rows == 1 {
        return vec![vec![1]];
    }

    let mut triangle: Vec<Vec<u64>> = vec![vec![]; num_rows as usize];
    triangle[0] = vec![1];

    for i in 1..num_rows {
        let mut element: Vec<u64> = vec![1; (i + 1) as usize];
        for j in 1..i {
            let left = triangle[(i - 1) as usize][(j - 1) as usize];
            let right = triangle[(i - 1) as usize][j as usize];
            let sum = left + right;
            element[j as usize] = sum;
        }
        triangle[i as usize] = element;
    }

    return triangle;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_generate_triangle_yanghui() {
        assert_eq!(vec![vec![1]], generate_triangle_yanghui(1));
        assert_eq!(vec![vec![1],
            vec![1, 1]],
            generate_triangle_yanghui(2));
        assert_eq!(vec![vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1]],
            generate_triangle_yanghui(5));
    }
}
