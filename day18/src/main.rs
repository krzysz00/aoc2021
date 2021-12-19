use std::sync::Arc;
use parking_lot::Mutex;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Relation {
    Left, Right, Root,
}

type Node = Arc<Mutex<Body>>;
#[derive(Clone, Debug)]
struct Body {
    pub value: Option<u32>,
    pub left: Option<Node>,
    pub right: Option<Node>,
    pub parent: Option<Node>,
    pub relation: Relation,
}

fn leaf(value: u32) -> Node {
    Arc::new(Mutex::new(
        Body { value: Some(value), left: None, right: None,
                parent: None, relation: Relation::Root }))
}

fn append(left: Node, right: Node) -> Node {
    let ret = Arc::new(Mutex::new(
        Body { value: None,
            left: Some(left), right: Some(right), parent: None,
            relation: Relation::Root }));
    {
        let ret_lock = ret.lock();
        {
            let mut left = ret_lock.left.as_ref().unwrap().lock();
            left.parent = Some(ret.clone());
            left.relation = Relation::Left;
        }
        {
            let mut right = ret_lock.right.as_ref().unwrap().lock();
            right.parent = Some(ret.clone());
            right.relation = Relation::Right;
        }
    }
    ret
}

fn predecessor(node: &Node) -> Option<Node> {
    let node_lock = node.lock();
    match node_lock.left.as_ref() {
        Some(r) => Some(rightmost_child(r)),
        None => {
            if node_lock.relation == Relation::Left {
                drop(node_lock);
                find_parent_not_on_right(node)
            } else {
                node_lock.parent.as_ref().cloned()
            }
        }
    }
}

fn find_parent_not_on_right(node: &Node) -> Option<Node> {
    let node_lock = node.lock();
    match node_lock.relation {
        Relation::Root => None,
        Relation::Left =>
            find_parent_not_on_right(node_lock.parent.as_ref().unwrap()),
        Relation::Right =>
            node_lock.parent.as_ref().cloned(),
    }
}

fn rightmost_child(node: &Node) -> Node {
    match node.lock().right.as_ref() {
        Some(n) => rightmost_child(n),
        None => node.clone(),
    }
}

fn prev_leaf(node: &Node) -> Option<Node> {
    predecessor(node).and_then(|pred| {
        let value = {
            let lock = pred.lock();
            lock.value
        };
        if value.is_some() {
            Some(pred)
        } else {
            prev_leaf(&pred)
        }
    })
}

fn successor(node: &Node) -> Option<Node> {
    let node_lock = node.lock();
    match node_lock.right.as_ref() {
        Some(r) => Some(leftmost_child(r)),
        None => {
            if node_lock.relation == Relation::Right {
                drop(node_lock);
                find_parent_not_on_left(node)
            } else {
                node_lock.parent.as_ref().cloned()
            }
        }
    }
}

fn find_parent_not_on_left(node: &Node) -> Option<Node> {
    let node_lock = node.lock();
    match node_lock.relation {
        Relation::Root => None,
        Relation::Left => node_lock.parent.as_ref().cloned(),
        Relation::Right =>
            find_parent_not_on_left(node_lock.parent.as_ref().unwrap())
    }
}

fn leftmost_child(node: &Node) -> Node {
    match node.lock().left.as_ref() {
        Some(n) => leftmost_child(n),
        None => node.clone(),
    }
}

fn next_leaf(node: &Node) -> Option<Node> {
    successor(node).and_then(|succ| {
        let value = {
            let lock = succ.lock();
            lock.value
        };
        if value.is_some() {
            Some(succ)
        } else {
            next_leaf(&succ)
        }
    })
}

fn replace(to_remove: &Node, replacement: Node) {
    let (parent, relation) = {
        let mut replacement_lock = replacement.lock();
        assert_eq!(replacement_lock.relation, Relation::Root);
        let (parent, relation) = {
            let lock = to_remove.lock();
            let parent = lock.parent.clone();
            if parent.as_ref().map(|p| p.is_locked()).unwrap_or(false) {
                panic!("Parent locked on acquisition");
            }
            let relation = lock.relation;
            (parent, relation)
        };
        replacement_lock.parent = parent.clone();
        replacement_lock.relation = relation;
        (parent, relation)
    };
    parent.as_ref().map(|p| {
        let mut p = p.lock();
        match relation {
            Relation::Left => p.left = Some(replacement.clone()),
            Relation::Right => p.right = Some(replacement.clone()),
            Relation::Root => panic!("Cant happen, roto with parent"),
        }
    });
}

// Stops and returns true when a reduction step occurs
fn reduce_explode(node: &Node, depth: u32) -> bool {
    if depth >= 4 && node.lock().value.is_none() {
        if node.lock().parent.as_ref().map(|p| p.is_locked()).unwrap_or(false) {
            panic!("Went into explosion with locked parent");
        }
        let (left, right) = {
            let lock = node.lock();
            let left = lock.left.as_ref().unwrap().clone();
            let right = lock.right.as_ref().unwrap().clone();
            (left, right)
        };
        let left_val = left.lock().value.expect("Left leaf in explosion");
        let right_val = right.lock().value.expect("Right leaf in explosion");
        let pred = prev_leaf(&left);
        let succ = next_leaf(&right);
        pred.map(|pred| {
            *pred.lock().value.as_mut().unwrap() += left_val;
        });
        succ.map(|succ| {
            *succ.lock().value.as_mut().unwrap() += right_val;
        });
        let new_node = leaf(0);
        replace(node, new_node);
        return true;
    }

    let left = {
        let lock = node.lock();
        lock.left.as_ref().cloned()
    };
    if let Some(l) = left {
        if reduce_explode(&l, depth + 1) {
            return true;
        }
    }

    let right = {
        let lock = node.lock();
        lock.right.as_ref().cloned()
    };
    if let Some(r) = right {
        if reduce_explode(&r, depth + 1) {
            return true;
        }
    }
    false
}

fn reduce_split(node: &Node) -> bool {
    let left = {
        let lock = node.lock();
        lock.left.as_ref().cloned()
    };
    if let Some(l) = left {
        if reduce_split(&l) {
            return true;
        }
    }

    let value = {
        let lock = node.lock();
        lock.value
    };
    if let Some(v) = value {
        if v >= 10 {
            let left = leaf(v / 2);
            let right = leaf((v + 1) / 2);
            let new_node = append(left, right);
            replace(node, new_node);
            return true;
        }
    }

    let right = {
        let lock = node.lock();
        lock.right.as_ref().cloned()
    };
    if let Some(r) = right {
        if reduce_split(&r) {
            return true;
        }
    }
    false
}

fn reduce(node: &Node) -> bool {
    reduce_explode(node, 0) || reduce_split(node)
}

fn add(left: Node, right: Node) -> Node {
    let ret = append(left, right);
    while reduce(&ret) {};
    ret
}

fn parse(line: &[u8], idx: usize) -> (Node, usize) {
    let byte = line[idx];
    if byte >= b'0' && byte <= b'9' {
        let node = leaf((byte - b'0') as u32);
        (node, idx + 1)
    } else if byte == b'[' {
        let (left, after_left) = parse(line, idx + 1);
        if line[after_left] != b',' {
            panic!("Parse error at character {}, expected ','", after_left);
        }
        let (right, after_right) = parse(line, after_left + 1);
        if line[after_right] != b']' {
            panic!("Parse error at character {}, expected ']'", after_right);
        }
        (append(left, right), after_right + 1)
    } else {
        panic!("Unexpected start of pair {}", byte);
    }
}

#[allow(dead_code)]
fn print_tree(node: &Node) {
    let node = node.lock();
    if let Some(v) = node.value {
        print!("{}", v)
    } else {
        print!("[");
        print_tree(node.left.as_ref().unwrap());
        print!(",");
        print_tree(node.right.as_ref().unwrap());
        print!("]");
    }
}

fn magnitude(node: &Node) -> u32 {
    let node = node.lock();
    if let Some(v) = node.value {
        v
    } else {
        3 * magnitude(node.left.as_ref().unwrap())
            + 2 * magnitude(node.right.as_ref().unwrap())
    }
}

fn part_a(input_str: &str) -> Node {
    let mut iter = input_str.lines();
    let mut ret = parse(iter.next().unwrap().as_bytes(), 0).0;
    for succ in iter {
        ret = add(ret, parse(succ.as_bytes(), 0).0);
        print_tree(&ret);
        println!();
    }
    ret
}

fn main() {
    let input_str =
        if std::env::args().any(|a| a == "sample") { SAMPLE } else { PUZZLE };
    let soln_a_node = part_a(input_str);
    print_tree(&soln_a_node);
    println!();
    println!("Part a: {}", magnitude(&soln_a_node));
}

const PUZZLE: &'static str = include_str!("input18");
const SAMPLE: &'static str =
/*"[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]";*/
"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
