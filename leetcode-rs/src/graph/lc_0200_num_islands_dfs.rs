/// 0200 岛屿数量 dfs
pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut counter = 0;
    for (i, chars) in grid.iter().enumerate() {
        for (j, &c) in chars.iter().enumerate() {
            if !visited[i][j] && c == '1' {
                counter += 1;
                dfs(&grid, &mut visited, (i as i32, j as i32))
            }
        }
    }

    counter
}

/// 递归函数
/// grid: 地图
/// visited: 标记是否访问过
/// counter: 计数器
/// (x,y): 当前访问节点的坐标
fn dfs(grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, (x, y): (i32, i32)) {

    // 确定终止条件
}