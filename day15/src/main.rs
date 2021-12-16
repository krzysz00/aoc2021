use std::collections::BinaryHeap;
use std::cmp::Reverse;

const PUZZLE: &'static str = include_str!("input15");
const SAMPLE: &'static str =
"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

#[derive(Clone, Debug, PartialEq, Eq)]
struct Graph {
    pub weights: Vec<u64>,
    pub neighbors: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new(weights: Vec<u64>) -> Self {
        let len = weights.len();
        let neighbors = (0..len).map(|_| Vec::with_capacity(4)).collect();
        Graph {weights, neighbors}
    }

    pub fn add_edge(&mut self, a: usize, b: usize) {
        self.neighbors[a].push(b);
    }
}

fn to_graph(weights: Vec<u64>, m: usize, n: usize) -> Graph {
    let mut graph = Graph::new(weights);
    for i in 0..m {
        for j in 0..n {
            let here = j + n * i;
            if i > 0 {
                graph.add_edge(here, j + n * (i - 1));
            }
            if i < m - 1 {
                graph.add_edge(here, j + n * (i + 1));
            }
            if j > 0 {
                graph.add_edge(here, here - 1);
            }
            if j < n - 1 {
                graph.add_edge(here, here + 1);
            }
        }
    }
    graph
}

fn extend_map(orig_weights: &Vec<u64>, m: usize, n: usize) -> Vec<u64> {
    let mut ret = Vec::with_capacity(orig_weights.len() * 5 * 5);
    for i in 0..m {
        for dup in 0..5 {
            for j in 0..n {
                let orig = orig_weights[j + n * i];
                let new = orig + dup;
                let new = if new > 9 { new - 9 } else { new };
                ret.push(new);
            }
        }
    }
    for dup in 1..5 {
        for c in 0..(m * n * 5) {
            let orig = ret[c];
            let new = orig + dup;
            let new = if new > 9 { new - 9 } else { new };
            ret.push(new);
        }
    }
    assert_eq!(m * n * 25, ret.len());
    ret
}

fn parse(input: &str) -> (Graph, Graph) {
    let mut m = 0;
    let mut weights = Vec::with_capacity(input.len());
    for line in input.lines() {
        for b in line.bytes() {
            if b >= b'0' && b <= b'9' {
                weights.push((b - b'0') as u64);
            }
        }
        m += 1;
    }
    let n = weights.len() / m;
    let ext_weights = extend_map(&weights, m, n);
    let graph_a = to_graph(weights, m, n);
    let graph_b = to_graph(ext_weights, m * 5, n * 5);
    (graph_a, graph_b)
}

fn dijkstra(g: &Graph, start: usize, end: usize) -> u64 {
    let mut queue = BinaryHeap::<Reverse<(u64, usize)>>::new();
    let mut dists = vec![u64::MAX; g.weights.len()];
    let mut visited = vec![false; g.weights.len()];
    dists[start] = 0;
    queue.push(Reverse((0, start)));
    loop {
        if queue.is_empty() {
            break;
        }
        let Reverse((_, node)) = queue.pop().unwrap();
        if visited[node] {
            // Duplicate entry from distance update
            continue;
        }
        visited[node] = true;
        if node == end {
            break;
        }
        let to_here = dists[node];
        for neighbor in g.neighbors[node].iter().copied() {
            if visited[neighbor] {
                continue;
            }
            let proposed = to_here + g.weights[neighbor];
            if proposed < dists[neighbor] {
                dists[neighbor] = proposed;
                queue.push(Reverse((proposed, neighbor)));
            }
        }
    }
    dists[end]
}

fn solve(graph: &Graph) -> u64 {
    dijkstra(graph, 0, graph.weights.len() - 1)
}

fn main() {
    let input_str =
        if std::env::args().any(|x| x == "sample") { SAMPLE } else { PUZZLE };
    let (graph_a, graph_b) = parse(input_str);
    let soln_a = solve(&graph_a);
    println!("Part a: {}", soln_a);
    let soln_b = solve(&graph_b);
    println!("Part b: {}", soln_b);
}
