use std::collections::HashSet;

fn parse(data: &str) -> Vec<(usize, usize)> {
    data.lines().map(|n| {
        let (l, r) = n.split_once(',').unwrap();
        (l.parse().unwrap(), r.parse().unwrap())
    }).collect()

}

#[cfg(feature = "test-input")]
const BOUNDS: usize = 6;
#[cfg(not(feature = "test-input"))]
const BOUNDS: usize = 70;
#[cfg(feature = "test-input")]
const FALL: usize = 12;
#[cfg(not(feature = "test-input"))]
const FALL: usize = 1024;

pub fn run(corrupt: &[(usize, usize)]) -> Option<usize> {
    let corrupt = HashSet::<(usize, usize)>::from_iter(corrupt.iter().cloned());
    let mut seen = HashSet::new();
    let mut queue = Vec::from([(0, 0)]);
    let mut iters = 0;
    while !queue.is_empty() {
        let search = queue;
        queue = Vec::new();
        for (row, col) in search {
            if corrupt.contains(&(row, col)) || seen.contains(&(row, col)) {
                continue
            }
            if row == BOUNDS && col == BOUNDS {
                return Some(iters)
            }
            seen.insert((row, col));
            if row > 0 {
                queue.push((row - 1, col));
            }
            if col > 0 {
                queue.push((row, col - 1));
            }
            if row < BOUNDS {
                queue.push((row + 1, col));
            }
            if col < BOUNDS {
                queue.push((row, col + 1));
            }
        }
        iters += 1;
    }
    None
}

pub fn run_one(data: &str) -> String {
    let coords = parse(data);
    run(&coords[..FALL]).unwrap().to_string()
}

pub fn run_two(data: &str) -> String {
    let coords = parse(data);
    let mut iter = FALL;
    while let Some(_) = run(&coords[..=iter]) {
        iter += 1;
    }
    let (row, col) = coords[iter];
    format!("{},{}", row, col)
}