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

fn part_a(input: &[Vec<i8>]) -> i32 {
    let m = input.len();
    let n = input[0].len();
    let mut ret = 0;
    for i in 0..m {
        for j in 0..n {
            let here = input[i][j];
            // Walls are taller than anything in the input
            let up = input.get(i - 1).map_or(10, |r| r[j]);
            let down = input.get(i + 1).map_or(10, |r| r[j]);
            let left = input[i].get(j - 1).copied().unwrap_or(10);
            let right = input[i].get(j + 1).copied().unwrap_or(10);
            if here < up && here < down && here < left && here < right {
                ret += (1 + here) as i32;
            }
        }
    }
    ret
}

fn main() {
    let input_str =
        if std::env::args().any(|x| x == "sample") { SAMPLE } else { PUZZLE };
    let input = parse(input_str);
    let soln_a = part_a(&input);
    println!("Part a: {}", soln_a);
}
