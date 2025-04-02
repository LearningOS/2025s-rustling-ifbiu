/*
	bfs
	This problem requires you to implement a basic BFS algorithm
*/

use std::collections::VecDeque;

// Define a graph
struct Graph {
    adj: Vec<Vec<usize>>, 
}

impl Graph {
    // Create a new graph with n vertices
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    // Add an edge to the graph
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest); 
        self.adj[dest].push(src); 
    }

    // Perform a breadth-first search on the graph, return the order of visited nodes
    fn bfs_with_return(&self, start: usize) -> Vec<usize> {
        // 创建结果向量存储访问顺序
        let mut visit_order = Vec::new();
        
        // 如果图为空，则直接返回空结果
        if self.adj.is_empty() {
            return visit_order;
        }
        
        // 创建队列和已访问节点集
        let mut queue = VecDeque::new();
        let mut visited = std::collections::HashSet::new();
        
        // 将起始节点加入队列和已访问集
        queue.push_back(start);
        visited.insert(start);
        
        // BFS 主循环
        while let Some(node) = queue.pop_front() {
            // 将当前节点添加到结果中
            visit_order.push(node);
            
            // 遍历当前节点的所有邻居
            for &neighbor in &self.adj[node] {
                // 如果邻居尚未访问，加入队列和已访问集
                if !visited.contains(&neighbor) {
                    queue.push_back(neighbor);
                    visited.insert(neighbor);
                }
            }
        }
        
        visit_order
    }
}

fn bfs(graph: &Graph, start: usize) -> Vec<usize> {
    // 创建结果向量存储访问顺序
    let mut result = Vec::new();
    
    // 如果图为空，则返回空结果
    if graph.adj.is_empty() {
        return result;
    }
    
    // 创建队列和已访问节点集
    let mut queue = std::collections::VecDeque::new();
    let mut visited = std::collections::HashSet::new();
    
    // 将起始节点加入队列和已访问集
    queue.push_back(start);
    visited.insert(start);
    
    // BFS 主循环
    while let Some(node) = queue.pop_front() {
        // 将当前节点添加到结果中
        result.push(node);
        
        // 遍历当前节点的所有邻居
        if let Some(neighbors) = graph.adj.get(node) {
            for &neighbor in neighbors {
                // 如果邻居尚未访问，加入队列和已访问集
                if !visited.contains(&neighbor) {
                    queue.push_back(neighbor);
                    visited.insert(neighbor);
                }
            }
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_all_nodes_visited() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 4);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 4, 2, 3]);
    }

    #[test]
    fn test_bfs_different_start() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visited_order = graph.bfs_with_return(2);
        assert_eq!(visited_order, vec![2, 1, 0]);
    }

    #[test]
    fn test_bfs_with_cycle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_bfs_single_node() {
        let mut graph = Graph::new(1);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0]);
    }
}

