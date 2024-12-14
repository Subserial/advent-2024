#[derive(Debug)]
struct Machine {
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
    tx: i64,
    ty: i64,
}

fn parse(data: &str) -> Vec<Machine> {
    let parser = regex::Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .unwrap();
    data.split("\n\n")
        .map(|s| {
            let (_, [ax, ay, bx, by, tx, ty]) = parser.captures(s).unwrap().extract();
            Machine {
                ax: ax.parse().unwrap(),
                ay: ay.parse().unwrap(),
                bx: bx.parse().unwrap(),
                by: by.parse().unwrap(),
                tx: tx.parse().unwrap(),
                ty: ty.parse().unwrap(),
            }
        })
        .collect()
}

fn gcf(x: i64, y: i64) -> i64 {
    if y == 0 {
        x
    } else {
        gcf(y, x % y)
    }
}

fn optimum(m: Machine) -> Option<(i64, i64)> {
    let gx = gcf(m.ax, m.bx);
    let gy = gcf(m.ay, m.by);
    if m.tx % gx != 0 || m.ty % gy != 0 {
        return None;
    }
    if m.ax % m.bx == 0 && m.ay % m.by == 0 && m.ax / m.bx == m.ay / m.by {
        return if m.ax / m.bx <= 3 {
            Some((0, m.tx / m.bx))
        } else {
            Some((m.tx / m.ax, 0))
        };
    }
    if m.bx % m.ax == 0 && m.by % m.ay == 0 && m.bx / m.ax == m.by / m.ay {
        return Some((0, m.tx / m.bx));
    }
    let guess_m = m.ax * m.by - m.ay * m.bx;
    let guess_xb = m.tx * m.by - m.ty * m.bx;
    if guess_xb.abs() % guess_m.abs() != 0 {
        return None;
    }
    let x = guess_xb / guess_m;
    let guess_yb = m.tx * m.ay - m.ty * m.ax;
    if guess_yb.abs() % guess_m.abs() != 0 {
        return None;
    }
    let y = -guess_yb / guess_m;
    if x >= 0 && y < 0 || x < 0 && y >= 0 {
        return None;
    }
    if x >= 0 {
        Some((x, y))
    } else {
        Some((-x, -y))
    }
}

pub fn run_one(data: &str) -> String {
    parse(data)
        .into_iter()
        .filter_map(|m| optimum(m))
        .map(|(a, b)| 3 * a + b)
        .sum::<i64>()
        .to_string()
}

const ROUNDING_ERROR: i64 = 10000000000000;

pub fn run_two(data: &str) -> String {
    parse(data)
        .into_iter()
        .map(|m| Machine {
            tx: m.tx + ROUNDING_ERROR,
            ty: m.ty + ROUNDING_ERROR,
            ..m
        })
        .filter_map(|m| optimum(m))
        .map(|(a, b)| 3 * a + b)
        .sum::<i64>()
        .to_string()
}
