impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let graph_len = graph.len();
        let mut reverse_graph: Vec<Vec<i32>> = graph.clone();
        reverse_graph.reverse();
        let in_degree = vec![0; n];
        for i in 0..graph_len {
            for j in 0..graph[i].len() {}
        }
        return;
    }
}