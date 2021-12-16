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

fn parse(input: &str) -> Graph {
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

fn part_a(graph: &Graph) -> u64 {
    dijkstra(graph, 0, graph.weights.len() - 1)
}

fn main() {
    let input_str =
        if std::env::args().any(|x| x == "sample") { SAMPLE } else { PUZZLE };
    let graph = parse(input_str);
    let soln_a = part_a(&graph);
    println!("Part a: {}", soln_a);
}
