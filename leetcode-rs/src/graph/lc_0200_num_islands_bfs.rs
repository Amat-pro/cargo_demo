/// 0200 岛屿数量 bfs
pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut counter = 0;
    for (i, chars) in grid.iter().enumerate() {
        for (j, &c) in chars.iter().enumerate() {
            if !visited[i][j] && c == '1' {
                counter += 1;
                bfs(&grid, &mut visited, (i as i32, j as i32))
            }
        }
    }

    counter
}

/// bfs 广度优先函数
/// grid: 地图
/// visited: 标记时候访问
/// (x,y): 当前坐标
fn bfs(grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, (x, y): (i32, i32)) {}