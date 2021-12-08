use std::error::Error as StdError;

type Error = Box<dyn StdError>;
type Result<T> = std::result::Result<T, Error>;

const PUZZLE: &'static str = include_str!("input07");
const SAMPLE: &'static str = "16,1,2,0,4,2,7,1,2,14";

fn parse(input: &str) -> Result<Vec<i32>> {
    input.split(',').map(|l| l.parse().map_err(Error::from)).collect()
}

fn l1_distance(xi: &[i32], y: i32) -> i32 {
    xi.iter().copied().map(move |x| (x - y).abs()).sum()
}

#[inline]
fn part_b_metric(x: i32, y: i32) -> i32 {
    let dist = (x - y).abs();
    ((dist) * (dist + 1)) / 2
}

fn part_b_distance(xi: &[i32], y: i32) -> i32 {
    xi.iter().copied().map(move |x| part_b_metric(x, y)).sum()
}

fn solve(input: &[i32], distance: fn(&[i32], i32) -> i32) -> i32 {
    let average = input.iter().sum::<i32>() / (input.len() as i32);
    let mut candidate = average;
    let mut candidate_dist = distance(input, candidate);
    loop {
        let left_cand = candidate - 1;
        let right_cand = candidate + 1;
        let left_cand_dist = distance(input, left_cand);
        let right_cand_dist = distance(input, right_cand);
        if left_cand_dist < candidate_dist {
            candidate = left_cand;
            candidate_dist = left_cand_dist;
        } else if right_cand_dist < candidate_dist {
            candidate = right_cand;
            candidate_dist = right_cand_dist;
        } else { // We've converged
            break;
        }
    }
    candidate_dist
}

fn main() -> Result<()> {
    let input_str =
        if std::env::args().any(|x| x == "sample") { SAMPLE } else { PUZZLE };
    let input = parse(input_str)?;
    let soln_a = solve(&input, l1_distance);
    println!("Part a: {}", soln_a);
    let soln_b = solve(&input, part_b_distance);
    println!("Part b: {}", soln_b);
    Ok(())
}
