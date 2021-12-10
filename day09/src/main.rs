use std::collections::HashSet;

const PUZZLE: &'static str = include_str!("input09");
const SAMPLE: &'static str =
"2199943210
3987894921
9856789892
8767896789
9899965678
";

fn parse(input: &str) -> Vec<Vec<i8>> {
    input.lines().map(|l|
        l.bytes().filter_map(|b| {
            if b >= b'0' && b <= b'9' {
                Some((b - b'0') as i8)
            } else { None }
        }).collect()).collect()
}

fn part_a(input: &[Vec<i8>]) -> (i32, Vec<(usize, usize)>) {
    let m = input.len();
    let n = input[0].len();
    let mut ret = 0;
    let mut low_points = vec![];
    for i in 0..m {
        for j in 0..n {
            let here = input[i][j];
            // Walls are taller than anything in the input
            let up = input.get(i - 1).map_or(10, |r| r[j]);
            let down = input.get(i + 1).map_or(10, |r| r[j]);
            let left = input[i].get(j - 1).copied().unwrap_or(10);
            let right = input[i].get(j + 1).copied().unwrap_or(10);
            if here < up && here < down && here < left && here < right {
                low_points.push((i, j));
                ret += (1 + here) as i32;
            }
        }
    }
    (ret, low_points)
}

fn basin(input: &[Vec<i8>], i: usize, j: usize,
            ret: &mut HashSet<(usize, usize)>) {
    if input[i][j] == 9 {
        return;
    }
    ret.insert((i, j));
    if i > 0 && !ret.contains(&(i - 1, j)) {
        basin(input, i - 1, j, ret);
    }
    if i < input.len() - 1 && !ret.contains(&(i + 1, j)) {
        basin(input, i + 1, j, ret);
    }
    let here = &input[i];
    if j > 0 && !ret.contains(&(i, j - 1)) {
        basin(input, i, j - 1, ret);
    }
    if j < here.len() - 1 && !ret.contains(&(i, j + 1)) {
        basin(input, i, j + 1, ret);
    }
}

fn part_b(input: &[Vec<i8>], low_points: &[(usize, usize)]) -> usize {
    let mut basins = Vec::new();
    for (i, j) in low_points.iter().copied() {
        let mut component = HashSet::new();
        basin(input, i, j, &mut component);
        basins.push(component);
    }
    basins.sort_unstable_by_key(|b| b.len());
    let n_basins = basins.len();
    basins[n_basins-3..].iter().map(|s| s.len()).product()
}

fn main() {
    let input_str =
        if std::env::args().any(|x| x == "sample") { SAMPLE } else { PUZZLE };
    let input = parse(input_str);
    let (soln_a, low_points) = part_a(&input);
    println!("Part a: {}", soln_a);
    let soln_b = part_b(&input, &low_points);
    println!("Part b: {}", soln_b);
}
