use std::collections::{HashMap, VecDeque};
use std::ops::BitXor;

fn parse(data: &str) -> Vec<usize> {
    data.lines().map(|line| line.parse().unwrap()).collect()
}

fn next(secret: usize) -> usize {
    let secret = secret.bitxor(secret << 6) % 16777216;
    let secret = secret.bitxor(secret >> 5) % 16777216;
    let secret = secret.bitxor(secret << 11) % 16777216;
    secret
}

pub fn run_one(data: &str) -> String {
    let seq = parse(data);
    seq.iter()
        .map(|&s| {
            let mut secret = s;
            for _ in 0..2000 {
                secret = next(secret);
            }
            secret
        })
        .sum::<usize>()
        .to_string()
}

pub fn run_two(data: &str) -> String {
    let seq = parse(data);
    let mut total = HashMap::new();
    for s in seq {
        let mut hist = VecDeque::new();
        let mut last = (s % 10) as i64;
        let mut secret = s;
        let mut seen = HashMap::new();
        for _ in 0..3 {
            secret = next(secret);
            let next_last = (secret % 10) as i64;
            hist.push_back(next_last - last);
            last = next_last;
        }
        for _ in 3..2000 {
            secret = next(secret);
            let next_last = (secret % 10) as i64;
            hist.push_back(next_last - last);
            if !seen.contains_key(&(hist[0], hist[1], hist[2], hist[3])) {
                seen.insert((hist[0], hist[1], hist[2], hist[3]), next_last);
            }
            hist.pop_front();
            last = next_last;
        }
        for (seq, first) in seen {
            match total.get_mut(&seq) {
                Some(v) => {
                    *v += first;
                }
                None => {
                    total.insert(seq, first);
                }
            }
        }
    }
    total.values().max().unwrap().to_string()
}
