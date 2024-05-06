/// 0797 所有可能的路径
pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let (mut result, mut path): (Vec<Vec<i32>>, Vec<i32>) = (vec![], vec![0]);
    dfs(&graph, &mut path, &mut result, 0);
    result
}

// 递归函数
fn dfs(graph: &Vec<Vec<i32>>, path: &mut Vec<i32>, result: &mut Vec<Vec<i32>>, node: usize) {
    let end: usize = graph.len() - 1;

    // 终止条件 - 收集结果并返回
    if node == end {
        result.push(path.clone());
        return;
    }

    // 处理当前节点 - 遍历当前节点能访问到的节点并进行dfs
    for &n in &(graph[node]) {
        path.push(n);
        // 递归
        dfs(graph, path, result, n as usize);
        // 回溯
        path.pop();
    }
}