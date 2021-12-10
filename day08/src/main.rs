use lazy_static::lazy_static;
use itertools::Itertools;

use std::collections::HashMap;
use std::collections::BTreeSet;

use std::iter::FromIterator;

lazy_static! {
    static ref SEGMENTS: [BTreeSet<usize>; 10] = [
        BTreeSet::from_iter([0, 1, 2, 4, 5, 6]),
        BTreeSet::from_iter([2, 5]),
        BTreeSet::from_iter([0, 2, 3, 4, 6]),
        BTreeSet::from_iter([0, 2, 3, 5, 6]),
        BTreeSet::from_iter([1, 2, 3, 5]),
        BTreeSet::from_iter([0, 1, 3, 5, 6]),
        BTreeSet::from_iter([0, 1, 3, 4, 5, 6]),
        BTreeSet::from_iter([0, 2, 5]),
        BTreeSet::from_iter([0, 1, 2, 3, 4, 5, 6]),
        BTreeSet::from_iter([0, 1, 2, 3, 5, 6]),
    ];
    static ref DIGITS: HashMap<BTreeSet<usize>, usize> = {
        SEGMENTS.iter().enumerate().map(|(i, s)| (s.clone(), i))
            .collect()
    };
    static ref DIGITS_BY_LEN: [Vec<usize>; 8] = {
        [vec![], vec![], vec![1], vec![7],
            vec![4], vec![2, 3, 5],
            vec![0, 6, 9], vec![8]]
    };
}

#[derive(Clone, PartialEq, Eq, Default)]
struct Perm {
    to_seg: [Option<usize>; 10],
    to_scrambled: [Option<usize>; 10],
}

impl Perm {
    pub fn new() -> Self {
        Default::default()
    }

    // Insert map a <-> b only if a is not mapped to any b'
    // and there is no a' mapped to be.
    // This includes duplicate entries of the same mapping
    pub fn try_insert(&mut self, scrambled: usize, segment: usize) -> bool {
        if self.to_seg[scrambled].is_some()
                || self.to_scrambled[segment].is_some() {
            false
        } else {
            self.to_seg[scrambled] = Some(segment);
            self.to_scrambled[segment] = Some(scrambled);
            true
        }
    }

    // Map all defined members of input through the permutation
    // Ignore those inputs with no defined value
    pub fn unscramble(&self, input: &BTreeSet<usize>) -> BTreeSet<usize> {
        input.iter().copied().filter_map(|i| self.to_seg[i]).collect()
    }

    pub fn assigned(&self, segments: &BTreeSet<usize>) -> BTreeSet<usize> {
        segments.iter().copied().filter_map(|i| self.to_scrambled[i]).collect()
    }

    // Removes the mapping segment <-> scrambled
    // Panics if the mapping was not previously inserted
    pub fn unmap(&mut self, scrambled: usize, segment: usize) {
        self.to_seg[scrambled] = None;
        self.to_scrambled[segment] = None;
    }
}

fn parse_seven_segment(word: &str) -> BTreeSet<usize> {
    word.bytes().map(|x| (x - b'a') as usize).collect()
}

fn parse(line: &str) -> (Vec<BTreeSet<usize>>, Vec<BTreeSet<usize>>) {
    let (examples, tests) = line.split_once('|').unwrap();
    (examples.split_whitespace().map(parse_seven_segment).collect(),
    tests.split_whitespace().map(parse_seven_segment).collect())
}

fn solve(examples: &[BTreeSet<usize>], perm: &mut Perm) -> bool {
    if examples.len() == 0 {
        return true;
    }
    let (scrambled, cont) = examples.split_first().unwrap();
    let already_mapped = perm.unscramble(scrambled);
    for digit in DIGITS_BY_LEN[scrambled.len()].iter().copied() {
        let segments = &SEGMENTS[digit];
        let assigned = perm.assigned(segments);
        if already_mapped.len() != assigned.len() {
            continue;
        }
        if !segments.is_superset(&already_mapped) {
            continue;
        }
        if &already_mapped == segments {
            return solve(cont, perm);
        }
        for segments_perm in segments.difference(&already_mapped).copied()
            .permutations(segments.len() - already_mapped.len()) {
            let mut stopped_at: Option<usize> = None;
            for (i, (a, b)) in scrambled.difference(&assigned).copied()
                .zip(segments_perm.iter().copied()).enumerate() {
                if !perm.try_insert(a, b) {
                    stopped_at = Some(i);
                    break;
                }
            }
            if stopped_at.is_none() && solve(cont, perm) {
                return true;
            } else {
                let stop = stopped_at.unwrap_or(scrambled.len());
                for (a, b) in scrambled.difference(&assigned).copied()
                    .zip(segments_perm.iter().copied()).take(stop) {
                    perm.unmap(a, b);
                }
            }
        }
    }
    false
}

fn solve_examples(mut examples: Vec<BTreeSet<usize>>) -> Perm {
    let mut ret = Perm::new();
    examples.sort_by_key(|x| x.len());
    if !solve(&examples, &mut ret) {
        panic!("Couldn't solve examples {:?}", examples);
    }
    ret
}

fn decode(tests: &[BTreeSet<usize>], perm: &Perm) -> Vec<usize> {
    tests.iter().map(
        |scr| *DIGITS.get(&perm.unscramble(scr)).unwrap())
    .collect()
}

const PUZZLE: &'static str = include_str!("input08");
const SAMPLE: &'static str = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf\n";

fn main() {
    let input_str =
        if std::env::args().any(|x| x == "sample") { SAMPLE } else { PUZZLE };
    let mut decoded_examples: Vec<Vec<usize>> = Vec::new();
    for line in input_str.lines() {
        let (examples, tests) = parse(line);
        let perm = solve_examples(examples);
        decoded_examples.push(decode(&tests, &perm));
    }
    let part_a: usize = decoded_examples.iter()
        .map(|i| i.iter().copied()
            .filter(|&e| e == 1 || e == 4 || e == 7 || e == 8 ).count())
        .sum();
    println!("Part a: {}", part_a);
    let part_b: usize = decoded_examples.iter()
        .map(|i| i[3] + 10 * i[2] + 100 * i[1] + 1000 * i[0])
        .sum();
    println!("Part b: {}", part_b);

}
