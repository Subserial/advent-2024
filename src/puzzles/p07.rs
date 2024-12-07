
fn parse(data: &str) -> Vec<(u64, Vec<u64>)> {
    data.lines().map(|line| {
        let (left, right) = line.split_once(": ").unwrap();
        let nums = right.split_whitespace().map(|n| n.parse::<u64>().unwrap()).collect();
        (left.parse::<u64>().unwrap(), nums)
    }).collect()
}

fn collect_one(target: u64, fold: u64, data: &[u64]) -> bool {
    if fold > target {
        false
    } else if data.len() == 0 {
        target == fold
    } else {
        collect_one(target, fold + data[0], &data[1..]) || collect_one(target, fold * data[0], &data[1..])
    }
}

pub fn run_one(data: &str) -> String {
    let eqs = parse(data);
    eqs.into_iter().map(|(target, data)| {
        if collect_one(target, data[0], &data[1..]) {
            target
        } else {
            0
        }
    }).sum::<u64>().to_string()
}

fn concat(left: u64, right: u64) -> u64 {
    left * 10u64.pow((right as f64 + 1.0).log10().ceil() as u32) + right
}

fn collect_two(target: u64, fold: u64, data: &[u64]) -> bool {
    if fold > target {
        false
    } else if data.len() == 0 {
        target == fold
    } else {
        collect_two(target, fold + data[0], &data[1..]) ||
            collect_two(target, fold * data[0], &data[1..]) ||
            collect_two(target, concat(fold, data[0]), &data[1..])
    }
}

pub fn run_two(data: &str) -> String {
    let eqs = parse(data);
    eqs.into_iter().map(|(target, data)| {
        if collect_two(target, data[0], &data[1..]) {
            target
        } else {
            0
        }
    }).sum::<u64>().to_string()
}