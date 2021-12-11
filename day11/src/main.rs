use ndarray::prelude::*;

use fxhash::FxHashSet;

type CoordSet = FxHashSet<(usize, usize)>;

fn parse_digit(d: u8) -> Option<u8> {
    if d >= b'0' && d <= b'9' {
        Some(d - b'0')
    } else {
        None
    }
}

fn parse(input: &str) -> Array2<u8> {
    let mut m = 0;
    let mut elems = Vec::with_capacity(input.len());
    for l in input.lines() {
        elems.extend(l.bytes().filter_map(parse_digit));
        m += 1;
    }
    let arr = Array2::from_shape_vec([m, elems.len() / m], elems).unwrap();
    arr
}

fn get(v: ArrayView2<u8>, (i, j): (usize, usize),
        di: isize, dj: isize) -> Option<((usize, usize), u8)> {
    if i == 0 && di < 0 {
        return None;
    }
    if j == 0 && dj < 0 {
        return None;
    }
    let idx_i = ((i as isize) + di) as usize;
    let idx_j = ((j as isize) + dj) as usize;
    v.get([idx_i, idx_j]).copied()
        .map(move |v| ((idx_i, idx_j), v))
}

fn get_flashers(arr: ArrayView2<u8>, to_test: &CoordSet, flashed: &CoordSet,
        new_flashes: &mut CoordSet) {
    for coord in to_test.iter().copied() {
        if arr[coord] > 9 && !flashed.contains(&coord) {
            new_flashes.insert(coord);
        }
    }
}

fn propagate_flash(coord: (usize, usize), mut arr: ArrayViewMut2<u8>,
        flashed: &CoordSet, to_test: &mut CoordSet) {
    for di in -1..=1 {
        for dj in [-1, 0, 1] {
            if di != 0 || dj != 0 {
                if let Some((neighbor, value)) = get(arr.view(), coord, di, dj) {
                    arr[neighbor] += 1;
                    if value >= 9 && !flashed.contains(&neighbor) {
                        to_test.insert(neighbor);
                    }
                }
            }
        }
    }
}

// Invariant: new_flashes starts and ends empty
// to_test came from the previous iteration
fn small_step(mut arr: ArrayViewMut2<u8>, flashed: &mut CoordSet,
        to_test: &mut CoordSet, new_flashes: &mut CoordSet) {
    get_flashers(arr.view(), &to_test, flashed, new_flashes);
    to_test.clear();
    for coord in new_flashes.iter().copied() {
        propagate_flash(coord, arr.view_mut(), flashed, to_test);
    }
    flashed.extend(new_flashes.drain());
}

fn large_step(mut arr: ArrayViewMut2<u8>, flashed: &mut CoordSet,
    to_test: &mut CoordSet, new_flashes: &mut CoordSet) {

    azip!((index (i, j), e in &mut arr) {
        *e += 1;
        if *e > 9 {
            to_test.insert((i, j));
        }
    });

    while !to_test.is_empty() {
        small_step(arr.view_mut(), flashed, to_test, new_flashes);
    }
}

fn new_coord_set() -> CoordSet {
    CoordSet::with_hasher(fxhash::FxBuildHasher::default())
}

const STEPS_A: usize = 100;
fn part_a(mut arr: ArrayViewMut2<u8>) -> usize {
    let mut flashed = new_coord_set();
    let mut new_flashes = new_coord_set();
    let mut to_test = new_coord_set();
    let mut ret = 0;

    for _i in 0..STEPS_A {
        //println!("Before i = {}\n{:?}", i, arr);
        large_step(arr.view_mut(), &mut flashed, &mut to_test,
                &mut new_flashes);
        ret += flashed.len();
        for coord in flashed.drain() {
            arr[coord] = 0;
        }
    }
    ret
}

fn main() {
    let input_str =
        if std::env::args().any(|x| x == "sample") { SAMPLE } else { PUZZLE };
    let parsed = parse(input_str);
    let mut input_for_a = parsed.clone();
    let soln_a = part_a(input_for_a.view_mut());
    println!("Part a: {}", soln_a);
}

const PUZZLE: &'static str = include_str!("input11");
const SAMPLE: &'static str =
"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";