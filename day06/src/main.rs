use std::error::Error as StdError;

type Error = Box<dyn StdError>;
type Result<T> = std::result::Result<T, Error>;

const ITERATIONS_A: usize = 80;
const ITERATIONS_B: usize = 256;
const PUZZLE: &'static str = include_str!("input06");
const SAMPLE: &'static str = "3,4,3,1,2";

fn parse(input: &str) -> Result<Vec<usize>> {
    input.split(',').map(|l| l.parse().map_err(|_| Error::from(format!("{:?} failed to parse", l)))).collect()
}

fn preprocess(raw_start_state: &[usize]) -> [usize; 9] {
    let mut ret = [0; 9];
    for i in raw_start_state.iter().copied() {
        ret[i] += 1;
    }
    return ret
}

fn step(state: &mut [usize; 9]) {
    state.rotate_left(1);
    state[6] += state[8];
}

fn total_fish(state: &[usize; 9]) -> usize {
    state.iter().sum()
}

fn solve(raw_input: &[usize], iterations: usize) -> usize {
    let mut state = preprocess(raw_input);
    for _ in 0..iterations {
        step(&mut state);
    }
    total_fish(&state)
}

fn main() -> Result<()> {
    let input_str =
        if std::env::args().any(|x| x == "sample") { SAMPLE } else { PUZZLE };
    let input = parse(input_str)?;
    let soln_a = solve(&input, ITERATIONS_A);
    println!("Part a: {}", soln_a);
    let soln_b = solve(&input, ITERATIONS_B);
    println!("Part b: {}", soln_b);
    Ok(())
}
