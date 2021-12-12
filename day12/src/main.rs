use fxhash::{FxHashMap,FxHashSet};
use std::collections::hash_map::Entry;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Node {
    key: usize,
    repeatable: bool,
    pub neighbors: Vec<usize>,
}

impl Node {
    pub fn new(key: usize, repeatable: bool) -> Self {
        Node { key, repeatable, neighbors: vec![] }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Graph {
    names: FxHashMap<String, usize>,
    pub nodes: Vec<Node>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RevisitStatus {
    Unused,
    Waiting(usize),
    Used,
}

impl Graph {
    pub fn new() -> Self {
        let mut ret = Graph { names: FxHashMap::default(), nodes: vec![] };
        ret.get_node_idx("start".into()); // start is node 0
        ret.get_node_idx("end".into()); // end is node 1
        ret
    }

    fn get_node_idx(&mut self, key: String) -> usize {
        match self.names.entry(key) {
            Entry::Occupied(o) => {
                *o.get()
            },
            Entry::Vacant(v) => {
                let new_key = self.nodes.len();
                let first_letter = v.key().as_bytes()[0];
                let is_multi_hop = first_letter >= b'A' && first_letter <= b'Z';
                let node = Node::new(new_key, is_multi_hop);
                println!("Adding node {} as {} repeatable={}", v.key(), new_key, is_multi_hop);
                self.nodes.push(node);
                v.insert(new_key);
                new_key
            }
        }
    }

    pub fn add_edge(&mut self, a: String, b: String) {
        let idx_a = self.get_node_idx(a);
        let idx_b = self.get_node_idx(b);
        self.nodes[idx_a].neighbors.push(idx_b);
        self.nodes[idx_b].neighbors.push(idx_a);

        if self.nodes[idx_a].repeatable && self.nodes[idx_b].repeatable {
            panic!("Adding infinite cycle {} <-> {}", idx_a, idx_b);
        }
    }
}

fn create_graph(input_str: &str) -> Graph {
    let mut ret = Graph::new();
    for edge in input_str.lines() {
        let (a, b) = edge.trim().split_once('-').expect("String to have a dash");
        ret.add_edge(a.into(), b.into())
    }
    ret
}

fn paths_to_end(graph: &Graph, node: usize, visited: &mut FxHashSet<usize>,
                   mut revisitable: RevisitStatus) -> usize {
    if node == 1 {
        return match revisitable {
            RevisitStatus::Waiting(_) => 0,
            _ => 1,
        }
    }
    let mut ret = 0;
    let this_node = &graph.nodes[node];
    let can_revisit = !this_node.repeatable &&
        revisitable == RevisitStatus::Unused && node != 0;
    for do_revisitable in [true, false] {
        if !can_revisit && do_revisitable {
            continue;
        }
        if !this_node.repeatable {
            if do_revisitable {
                revisitable = RevisitStatus::Waiting(node);
            } else {
                visited.insert(node);
                if revisitable == RevisitStatus::Waiting(node) {
                    revisitable = RevisitStatus::Used;
                }
            }
        }
        for neighbor in this_node.neighbors.iter().copied() {
            if !visited.contains(&neighbor) {
                ret += paths_to_end(graph, neighbor, visited, revisitable);
            }
        }
        if !this_node.repeatable {
            if do_revisitable {
                // Can do this since we wouldn't have tried the revisitable branch
                // if the initial status wasn't "unused"
                revisitable = RevisitStatus::Unused;
            } else {
                visited.remove(&node);
            }
        }
    }
    ret
}

fn part_a(graph: &Graph) -> usize {
    let mut visited = FxHashSet::with_capacity_and_hasher(graph.nodes.len(),
        fxhash::FxBuildHasher::default());
    paths_to_end(graph, 0, &mut visited, RevisitStatus::Used)
}

fn part_b(graph: &Graph) -> usize {
    let mut visited = FxHashSet::with_capacity_and_hasher(graph.nodes.len(),
        fxhash::FxBuildHasher::default());
    paths_to_end(graph, 0, &mut visited, RevisitStatus::Unused)
}

fn main() {
    let input_str =
        if std::env::args().any(|x| x == "sample") { SAMPLE } else { PUZZLE };
    let input = create_graph(input_str);
    let soln_a = part_a(&input);
    println!("Part a: {}", soln_a);
    let soln_b = part_b(&input);
    println!("Part b: {}", soln_b);
}

const PUZZLE: &'static str = include_str!("input12");
const SAMPLE: &'static str =
"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";
/*const SAMPLE: &'static str =
"start-A
start-b
A-c
A-b
b-d
A-end
b-end";*/

