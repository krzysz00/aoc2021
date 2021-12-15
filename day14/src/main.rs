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

type PolyRules = FxHashMap<(u8, u8), ((u8, u8), (u8, u8))>;
type Clusters = FxHashMap<(u8, u8), usize>;

fn parse(input: &str) -> (Clusters, PolyRules, u8) {
    let (first, rest) = input.split_once("\n\n").expect("Missing break in input");
    let mut init = Clusters::default();
    for w in first.as_bytes().windows(2) {
        *init.entry((w[0], w[1])).or_insert(0) += 1;
    }
    let input_last = first.as_bytes()[first.len() - 1];
    let mut rules = PolyRules::default();
    for line in rest.lines() {
        let bytes = line.as_bytes();
        let left = bytes[0];
        let right = bytes[1];
        let mid = bytes[6];
        rules.insert((left, right), ((left, mid), (mid, right)));
    }
    (init, rules, input_last)
}

fn step(prev: Clusters, rules: &PolyRules) -> Clusters {
    let mut ret = Clusters::with_capacity_and_hasher(prev.len(),
        fxhash::FxBuildHasher::default());
    for (pair, size) in prev {
        if let Some((l, r)) = rules.get(&pair).copied() {
            *ret.entry(l).or_insert(0) += size;
            *ret.entry(r).or_insert(0) += size;
        } else {
            *ret.entry(pair).or_insert(0) += size;
        }
    }
    ret
}


fn uncluster(clusters: &Clusters, input_last: u8) -> FxHashMap<u8, usize> {
    let mut ret = FxHashMap::default();
    for ((a, _b), n) in clusters {
        *ret.entry(*a).or_insert(0) += n;
    }
    // Account for the fact that the last item doesn't get counted above
    *ret.entry(input_last).or_insert(0) += 1;
    ret
}

const A_STEPS: usize = 10;
const B_STEPS: usize = 40;
fn solve(mut clusters: Clusters, rules: &PolyRules, input_last: u8, steps: usize) -> usize {
    for _ in 0..steps {
        clusters = step(clusters, rules);
    }
    let elem_counts = uncluster(&clusters, input_last);
    let max_count = elem_counts.values().copied().max().expect("Nonempty map");
    let min_count = elem_counts.values().copied().min().expect("Nonempty map");
    max_count - min_count
}

fn main() {
    let input_str =
        if std::env::args().any(|x| x == "sample") { SAMPLE } else { PUZZLE };
    let (initial, rules, input_last) = parse(input_str);
    let soln_a = solve(initial.clone(), &rules, input_last, A_STEPS);
    println!("Part a: {}", soln_a);
    let soln_b = solve(initial, &rules, input_last, B_STEPS);
    println!("Part b: {}", soln_b);
}
