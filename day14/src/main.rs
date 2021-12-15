use fxhash::FxHashMap;

const PUZZLE: &'static str = include_str!("input14");
const SAMPLE: &'static str =
"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

type PolyRules = FxHashMap<(u8, u8), u8>;

fn parse(input: &str) -> (Vec<u8>, PolyRules) {
    let (first, rest) = input.split_once("\n\n").expect("Missing break in input");
    let init = first.bytes().collect();
    let mut rules = PolyRules::default();
    for line in rest.lines() {
        let bytes = line.as_bytes();
        rules.insert((bytes[0], bytes[1]), bytes[6]);
    }
    (init, rules)
}

fn step(prev: Vec<u8>, rules: &PolyRules) -> Vec<u8> {
    let mut ret = Vec::with_capacity(prev.len());
    let len = prev.len();
    for i in 0..(len - 1) {
        ret.push(prev[i]);
        if let Some(v) = rules.get(&(prev[i], prev[i+1])).copied() {
            ret.push(v);
        }
    }
    ret.push(prev[len - 1]);
    ret
}


fn buckets(string: &[u8]) -> FxHashMap<u8, usize> {
    let mut ret = FxHashMap::default();
    for v in string.iter().copied() {
        ret.entry(v).and_modify(|e| *e += 1).or_insert(1);
    }
    ret
}

const A_STEPS: usize = 10;
const B_STEPS: usize = 40;
fn solve(mut string: Vec<u8>, rules: &PolyRules, steps: usize) -> usize {
    for _ in 0..steps {
        string = step(string, rules);
    }
    let elem_counts = buckets(&string);
    let max_count = elem_counts.values().copied().max().expect("Nonempty map");
    let min_count = elem_counts.values().copied().min().expect("Nonempty map");
    max_count - min_count
}

fn main() {
    let input_str =
        if std::env::args().any(|x| x == "sample") { SAMPLE } else { PUZZLE };
    let (initial, rules) = parse(input_str);
    let soln_a = solve(initial.clone(), &rules, A_STEPS);
    println!("Part a: {}", soln_a);
    let soln_b = solve(initial, &rules, B_STEPS);
    println!("Part b: {}", soln_b);
}
