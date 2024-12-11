use std::collections::HashMap;

fn parse(data: &str) -> Vec<usize> {
    data.split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

pub fn run_one(data: &str) -> String {
    let mut stones = parse(data);
    for _ in 0..25 {
        let prev = stones;
        stones = prev
            .into_iter()
            .map(|s| {
                if s == 0 {
                    vec![1]
                } else {
                    let sn = s.to_string();
                    if sn.len() % 2 == 0 {
                        let (left, right) = sn.split_at(sn.len() / 2);
                        vec![left.parse().unwrap(), right.parse().unwrap()]
                    } else {
                        vec![s * 2024]
                    }
                }
            })
            .flatten()
            .collect();
    }
    stones.len().to_string()
}

fn memo_run(memo: &mut HashMap<(usize, usize), usize>, n: usize, remaining: usize) -> usize {
    if let Some(&val) = memo.get(&(n, remaining)) {
        return val;
    };
    let result = {
        if remaining == 0 {
            1
        } else {
            if n == 0 {
                memo_run(memo, 1, remaining - 1)
            } else {
                let s = n.to_string();
                if s.len() % 2 == 0 {
                    let (left, right) = s.split_at(s.len() / 2);
                    memo_run(memo, left.parse().unwrap(), remaining - 1)
                        + memo_run(memo, right.parse().unwrap(), remaining - 1)
                } else {
                    memo_run(memo, n * 2024, remaining - 1)
                }
            }
        }
    };
    memo.insert((n, remaining), result);
    result
}

pub fn run_two(data: &str) -> String {
    let mut stones = parse(data);
    let mut memo = HashMap::new();
    stones
        .into_iter()
        .map(|n| memo_run(&mut memo, n, 75))
        .sum::<usize>()
        .to_string()
}
