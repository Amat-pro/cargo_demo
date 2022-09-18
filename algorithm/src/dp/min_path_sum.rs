/**
# 最小路径和

## 描述

给定一个包含非负整数的 m x n 网格 grid，请找出一条从左上角到右下角的路径，使得路径上的数字总和为最小
注意：每次只能向下或者向右移动一步

[最小路径和示例图](https://camo.githubusercontent.com/b4d64ee47839d755d9670e6ff497fa807d22750804ccf90b5602c175b643b134/68747470733a2f2f666173746c792e6a7364656c6976722e6e65742f67682f646f6f63732f6c656574636f6465406d61696e2f736f6c7574696f6e2f303030302d303039392f303036342e4d696e696d756d2532305061746825323053756d2f696d616765732f6d696e706174682e6a7067)

&nbsp;

## 提示
* m >= 1 && n <= 200
* 0 <= grid\[i]\[j] <= 100
* m == grid.length
* n == grid\[i].length

## Solution
行 n
列 m
第一行第一列： dp\[1,1] =  grid\[0,0]
第n行第m列： dp\[n,m] = min(dp\[n,m-1], dp\[n-1,m]) + grid\[n,m]

走到dp\[0,m-1]的最小和是第1行相加
走到dp\[n-1,0]的最小和是第1列相加

结果：dp\[n-1,m-1]

 **/

pub fn min_path_sum(mut grid: Vec<Vec<i64>>) -> i64 {
    // sum of path dp[0, m-1]
    let col_len = grid[0].len();
    for col in 1..col_len {
        grid[0][col] += grid[0][col - 1];
    }

    // sum of path dp[n-1, 0]
    let row_len = grid.len();
    for row in 1..row_len {
        grid[row][0] += grid[row - 1][0];
    }

    // calculate min_sum of
    for n in 1..row_len {
        for m in 1..col_len {
            grid[n][m] += grid[n][m - 1].min(grid[n - 1][m])
        }
    }

    grid[row_len - 1][col_len - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_path_sum() {
        let grid = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
        assert_eq!(7, min_path_sum(grid))
    }
}


