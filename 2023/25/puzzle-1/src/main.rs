use std::{fs, collections::{HashSet, HashMap, VecDeque}};
use itertools::iproduct;
use rand::seq::SliceRandom;

fn main() {
    let input: Vec<(String, Vec<String>)> = fs::read_to_string("input.txt").unwrap()
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|split|
            split
                .split(':')
                .map(|s| s.trim())
                .collect::<Vec<&str>>())
        .map(|split| (split[0].to_string(), split[1].split(' ').map(|s| s.trim().to_string()).collect::<Vec<String>>()))
        .collect();

    let mut nodes: HashSet<&str> = HashSet::new();
    let mut edges: HashMap<&str, HashSet<String>> = HashMap::new();

    for (node, connected) in &input {
        nodes.insert(node);
        for n in connected {
            nodes.insert(n);
            edges.entry(node).and_modify(|e| { e.insert(n.clone()); }).or_insert_with(|| {
                let mut neighbours: HashSet<String> = HashSet::new();
                neighbours.insert(n.clone());
                neighbours
            });
            edges.entry(n).and_modify(|e| { e.insert(node.clone()); }).or_insert_with(|| {
                let mut neighbours: HashSet<String> = HashSet::new();
                neighbours.insert(node.clone());
                neighbours
            });
        }
    }

    let mut graph: Graph = Graph::new();

    for node in &nodes {
        graph.add_node(Node { id: node.to_string(), neighbours: edges[node].clone() })
    }


    let mut most_common_edges: HashMap<(String, String), usize> = HashMap::new();

    // Create a significant sample of node pairs to run bfs on and find the most common edges
    // -----------------------------------------------------------------------------------------
    let samples: Vec<(&str, &str)> = iproduct!(nodes.iter(), nodes.iter())
        .map(|(a, b)| (*a, *b))
        .collect::<Vec<(&str, &str)>>()
        .choose_multiple(&mut rand::thread_rng(), 1000)
        .map(|(a, b)| (*a, *b))
        .collect();

    for (from, to) in samples {
        let bfs_result = graph.bfs(from, to);
        for window in bfs_result.windows(2) {
            let mut v = window.to_vec();
            v.sort();

            let key = (v[0].clone(), v[1].clone());
            most_common_edges.entry(key).and_modify(|e| *e += 1).or_insert(1);
        }
    }

    let mut most_common_edges: Vec<((String, String), usize)> = most_common_edges.into_iter().collect();
    most_common_edges.sort_by(|a, b| b.1.cmp(&a.1));

    // The top 3 most common edges should be the ones that need to be cut to create two separate
    // groups. If it doesn't work, increase the sample.
    // -----------------------------------------------------------------------------------------
    let top_3 = most_common_edges.iter().take(3).collect::<Vec<_>>();

    for ((from, to), _) in &top_3 {
        edges.entry(from).and_modify(|e| { e.remove(to); });
        edges.entry(to).and_modify(|e| { e.remove(from); });
    }

    let mut graph = Graph::new();

    for node in &nodes {
        graph.add_node(Node { id: node.to_string(), neighbours: edges[node].clone() })
    }

    let mut result_map: HashMap<&str, HashSet<String>> = HashMap::new();

    // We can find the size of each group by running bfs on each node belonging the cut edges
    // against every other node on the graph and counting the unique nodes that appear on the
    // results.
    for ((from, to), _) in &top_3 {
        for node in &nodes {
            let bfs_result = graph.bfs(from, node);
            for result in bfs_result {
                result_map.entry(from).and_modify(|e| { e.insert(result.clone()); }).or_insert({
                    let mut new = HashSet::new();
                    new.insert(result);
                    new
                });
            }

            let bfs_result = graph.bfs(to, node);
            for result in bfs_result {
                result_map.entry(to).and_modify(|e| { e.insert(result.clone()); }).or_insert({
                    let mut new = HashSet::new();
                    new.insert(result);
                    new
                });
            }
        }
    }

    for (key, value) in result_map {
        println!("{}: {:?}", key, value.len());
    }
}

#[derive(Debug)]
struct Node {
    id: String,
    neighbours: HashSet<String>
}

#[derive(Debug)]
struct Graph {
    nodes: HashMap<String, Node>
}

impl Graph {
    fn new() -> Self {
        Self { nodes: HashMap::new() }
    }

    fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id.clone(), node);
    }

    fn bfs(&self, from: &str, to: &str) -> Vec<String> {
        let mut to_see: VecDeque<(&str, Vec<String>)> = VecDeque::new();
        let mut visited: HashSet<&str> = HashSet::new();

        to_see.push_back((from, vec![from.to_string()]));

        while let Some((node, path)) = to_see.pop_front() {
            visited.insert(node);

            if node == to {
                return path;
            }

            for neighbour in &self.nodes[node].neighbours {
                if !visited.contains(neighbour.as_str()) {
                    visited.insert(neighbour);

                    let mut new_path = path.clone();
                    new_path.push(neighbour.clone());

                    to_see.push_back((neighbour, new_path));
                }
            }
        }
        vec![]
    }
}
