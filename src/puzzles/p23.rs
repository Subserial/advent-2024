use std::collections::{HashMap, HashSet};

fn parse(data: &str) -> Vec<(String, String)> {
    data.lines()
        .map(|line| {
            let (left, right) = line.split_once("-").unwrap();
            (left.to_string(), right.to_string())
        })
        .collect()
}

fn eval(conns: &Vec<(String, String)>) -> (Vec<&String>, HashMap<String, HashSet<String>>) {
    let all_conns = conns
        .iter()
        .map(|(left, right)| [left, right])
        .flatten()
        .collect::<Vec<_>>();
    let conn_map = conns
        .iter()
        .cloned()
        .map(|(left, right)| [(left.clone(), right.clone()), (right, left)])
        .flatten()
        .fold(
            HashMap::<String, HashSet<String>>::new(),
            |mut map, (left, right)| {
                match map.get_mut(&left) {
                    Some(s) => {
                        s.insert(right);
                    }
                    None => {
                        map.insert(left, HashSet::from([right]));
                    }
                };
                map
            },
        );
    (all_conns, conn_map)
}

pub fn run_one(data: &str) -> String {
    let conns = parse(data);
    let (all_conns, conn_map) = eval(&conns);
    let mut seen = HashSet::new();
    for conn in all_conns {
        if !conn.starts_with('t') {
            continue;
        }
        let paired = &conn_map[conn];
        for pair in paired {
            let rings = paired.intersection(&conn_map[pair]);
            for ring in rings {
                let mut group = [conn, pair, ring];
                group.sort();
                seen.insert(format!("{:?}", group));
            }
        }
    }
    seen.len().to_string()
}

pub fn run_two(data: &str) -> String {
    let conns = parse(data);
    let (all_conns, conn_map) = eval(&conns);
    let mut parties: Vec<HashSet<&String>> = Vec::new();
    for conn in all_conns {
        let local = &conn_map[conn];
        for party in &mut parties {
            if party.iter().all(|&com| local.contains(com)) {
                party.insert(conn);
            }
        }
        parties.push(HashSet::from([conn]));
    }
    let mut largest = parties.iter().next().unwrap();
    for party in &parties {
        if party.len() > largest.len() {
            largest = party;
        }
    }
    let mut pinkie = largest.into_iter().cloned().cloned().collect::<Vec<_>>();
    pinkie.sort();
    pinkie.join(",")
}
