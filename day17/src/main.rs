#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SimResult {
    Success(i32),
    NotEnoughX,
    NotEnoughY,
    Overshoot,
}

fn simulate(dx: i32, dy: i32,
        x_min: i32, x_max: i32,
        y_min: i32, y_max: i32) -> SimResult {
    let t_plummet = dx;
    let x_plummet = (dx * (dx + 1)) / 2;
    let y_plummet = if dy < 0 {
        ((dy - t_plummet + 1)..=dy).sum()
    }
    else if dy >= t_plummet {
        let remainder = dy - t_plummet;
        (dy * (dy + 1)) / 2 - (remainder * (remainder + 1)) / 2
    } else {
        let remainder = t_plummet - dy - 1;
        (dy * (dy + 1)) / 2 - (remainder * (remainder + 1)) / 2
    };
    if x_plummet < x_min {
        SimResult::NotEnoughX
    }
    else if x_plummet >= x_min && x_plummet <= x_max && y_plummet >= y_min {
        let mut vy = dy - t_plummet;
        let mut y = y_plummet;
        let mut ret = y;
        loop {
            if y >= y_min && y <= y_max {
                break SimResult::Success(ret);
            }
            if y < y_min {
                break SimResult::Overshoot;
            }
            y += vy;
            if vy > 0 {
                ret += vy;
            }
            vy -= 1;
        }
    } else {
        // We're off to the right or under
        let mut t = t_plummet;
        let mut x = x_plummet;
        let mut y = y_plummet;
        let mut vx = 0;
        let mut vy = dy - t_plummet;
        loop {
            if !(t >= 0 && x >= x_min && y <= y_max) {
                break SimResult::Overshoot;
            }
            // Note: Order matter here as we unwind time
            t -= 1;
            vx += 1;
            vy += 1;
            x -= vx;
            y -= vy;
            if x >= x_min && x <= x_max && y >= y_min && y <= y_max {
                // y(t) starts positive and goes negatave
                // so the maximum is at the time t where y'(t) stops growing
                // Or dy < 0 and so whatever
                let x_t = (0..t).map(|i| dx - i).sum();
                let y_t = (0..t).map(|i| dy -i).sum();
                assert_eq!(y, y_t);
                assert_eq!(x, x_t);
                let t_max = dy;
                let y_max = (t_max * (t_max + 1)) / 2;
                break SimResult::Success(y_max);
            }
        }
    }
}

fn simulate_part_a(x_min: i32, x_max: i32, y_min: i32, y_max: i32) -> i32 {
    let mut ret = 0;
    // Parameters chosen somewhat arbitrarily
    'outer: for dx in (0..x_max).rev() {
        'inner: for dy in (0..x_max).rev() {
            match simulate(dx, dy, x_min, x_max, y_min, y_max) {
                SimResult::NotEnoughX => {
                    break 'outer;
                },
                SimResult::NotEnoughY => {
                    break 'inner;
                }
                SimResult::Overshoot => (),
                SimResult::Success(v) => {
                    ret = std::cmp::max(ret, v);
                }
            }
        }
    }
    ret
}

fn part_b(x_min: i32, x_max: i32, y_min: i32, y_max: i32) -> usize{
    let mut ret = 0;
    'outer: for dx in (0..=x_max).rev() {
        'inner: for dy in (y_min..=x_max).rev() {
            match simulate(dx, dy, x_min, x_max, y_min, y_max) {
                SimResult::Success(_) => { ret += 1; },
                //SimResult::Success(_) => { println!("Got {}, {}", dx, dy); ret += 1 },
                SimResult::NotEnoughX => { break 'outer; },
                SimResult::NotEnoughY => { break 'inner; },
                SimResult::Overshoot => {},
            }
        }
    }
    ret
}

fn main() {
    let (x_min, x_max, y_min, y_max) =
        if std::env::args().any(|a| a == "sample") {
            (20, 30, -10, -5)
        } else {
            (277, 318, -92, -53)
        };
    let soln_a = simulate_part_a(x_min, x_max, y_min, y_max);
    println!("Part a: {}", soln_a);
    println!("Debug sample: {:?}", simulate(6, 7, x_min, x_max, y_min, y_max));
    let soln_b = part_b(x_min, x_max, y_min, y_max);
    println!("Part b: {}", soln_b);
}
