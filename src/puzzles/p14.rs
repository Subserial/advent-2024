pub fn parse(data: &str) -> Vec<((i64, i64), (i64, i64))> {
    let reg = regex::Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    data.lines()
        .map(|line| {
            let (_, [px, py, vx, vy]) = reg.captures(line).unwrap().extract();
            (
                (px.parse().unwrap(), py.parse().unwrap()),
                (vx.parse().unwrap(), vy.parse().unwrap()),
            )
        })
        .collect()
}

#[cfg(feature = "test-input")]
const X: i64 = 11;
#[cfg(feature = "test-input")]
const Y: i64 = 7;
#[cfg(not(feature = "test-input"))]
const X: i64 = 101;
#[cfg(not(feature = "test-input"))]
const Y: i64 = 103;

pub fn run_one(data: &str) -> String {
    let paths = parse(data);
    let pos = paths
        .iter()
        .map(|((px, py), (vx, vy))| {
            let mut fx = (px + vx * 100) % X;
            let mut fy = (py + vy * 100) % Y;
            if fx < 0 {
                fx += X
            }
            if fy < 0 {
                fy += Y
            }
            if fx == X / 2 || fy == Y / 2 {
                (0, 0, 0, 0)
            } else if fx < X / 2 && fy < Y / 2 {
                (1, 0, 0, 0)
            } else if fx < X / 2 && fy > Y / 2 {
                (0, 1, 0, 0)
            } else if fx > X / 2 && fy < Y / 2 {
                (0, 0, 1, 0)
            } else {
                (0, 0, 0, 1)
            }
        })
        .fold((0, 0, 0, 0), |acc, pos| {
            (acc.0 + pos.0, acc.1 + pos.1, acc.2 + pos.2, acc.3 + pos.3)
        });
    (pos.0 * pos.1 * pos.2 * pos.3).to_string()
}

fn step(data: &Vec<((i64, i64), (i64, i64))>, n: i64) -> Vec<(i64, i64)> {
    data.iter()
        .map(|((px, py), (vx, vy))| {
            let mut fx = (px + vx * n) % X;
            let mut fy = (py + vy * n) % Y;
            if fx < 0 {
                fx += X
            }
            if fy < 0 {
                fy += Y
            }
            (fx, fy)
        })
        .collect()
}

fn print(steps: &Vec<(i64, i64)>, val: i64) {
    let mut buf = [['.'; X as usize]; Y as usize];
    for &(x, y) in steps {
        buf[y as usize][x as usize] = '#';
    }
    let printed = buf
        .iter()
        .map(|line| line.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n");
    println!("{} {}", printed, val);
}

pub fn run_two(data: &str) -> String {
    let paths = parse(data);
    let mut val = 0;
    loop {
        val += 1;
        let step_pos = step(&paths, val);
        let score = step_pos
            .iter()
            .filter(|&&(px, py)| px == 42 || py == 42)
            .count();
        if score > 50 {
            // print(&step_pos, val);
            // std::io::stdin().read_line(&mut String::new()).unwrap();
            return val.to_string();
        }
    }
}
