use std::collections::{HashMap, HashSet};

fn parse(data: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let (first, second) = data.split_once("\n\n").unwrap();
    let orders = first
        .lines()
        .map(|line| {
            let mut vals = line.split('|').map(|n| n.parse::<u32>().unwrap());
            (vals.next().unwrap(), vals.next().unwrap())
        })
        .collect();
    let sequences = second
        .lines()
        .map(|line| {
            line.split(',')
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    (orders, sequences)
}

fn extract_good(orders: &Vec<(u32, u32)>, sequences: &Vec<Vec<u32>>) -> Vec<usize> {
    let mut good = Vec::new();
    for (i, sequence) in sequences.iter().enumerate() {
        let mut banned = Vec::new();
        let mut ordered = true;
        for &val in sequence {
            if banned.contains(&val) {
                ordered = false;
                break;
            }
            for order in orders {
                if val == order.1 {
                    banned.push(order.0);
                }
            }
        }
        if ordered {
            good.push(i);
        }
    }
    good
}

pub fn run_one(data: &str) -> String {
    let (orders, sequences) = parse(data);
    let good = extract_good(&orders, &sequences)
        .into_iter()
        .collect::<HashSet<_>>();
    sequences
        .iter()
        .enumerate()
        .filter(|(i, _)| good.contains(i))
        .map(|(_, sequence)| sequence[sequence.len() / 2])
        .sum::<u32>()
        .to_string()
}

pub fn run_two(data: &str) -> String {
    let (orders, sequences) = parse(data);
    let good = extract_good(&orders, &sequences)
        .into_iter()
        .collect::<HashSet<_>>();
    sequences
        .into_iter()
        .enumerate()
        .filter(|(i, _)| !good.contains(i))
        .map(|(_, mut sequence)| {
            let mut ordered = false;
            while !ordered {
                ordered = true;
                let mut seen = HashMap::<u32, usize>::new();
                for i in 0..sequence.len() {
                    for order in &orders {
                        if sequence[i] == order.0 {
                            if let Some(prev_idx) = seen.get(&order.1) {
                                sequence.swap(i, *prev_idx);
                                ordered = false;
                                break;
                            }
                        }
                        seen.insert(sequence[i], i);
                    }
                }
            }
            sequence[sequence.len() / 2]
        })
        .sum::<u32>()
        .to_string()
}
