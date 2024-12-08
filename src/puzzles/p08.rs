use std::collections::{HashMap, HashSet};

fn parse(data: &str) -> (HashMap<char, Vec<(i32, i32)>>, i32, i32) {
    let rows = data.lines().count();
    let cols = data.lines().next().unwrap().len();
    let nodes = data
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(j, c)| {
                    if c != '.' {
                        Some((c, j as i32, i as i32))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();
    let mut node_map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for (c, x, y) in nodes {
        match node_map.get_mut(&c) {
            Some(r) => {
                r.push((x, y));
            }
            None => {
                node_map.insert(c, vec![(x, y)]);
            }
        };
    }
    (node_map, rows as i32, cols as i32)
}

pub fn run_one(data: &str) -> String {
    let (node_map, rows, cols) = parse(data);
    let mut seen = HashSet::new();
    for coords in node_map.values() {
        for i in 0..coords.len() {
            for j in (i + 1)..coords.len() {
                let (x1, y1) = coords[i];
                let (x2, y2) = coords[j];
                let x_delta = x2 - x1;
                let y_delta = y2 - y1;
                let xu = x2 + x_delta;
                let yu = y2 + y_delta;
                let xd = x1 - x_delta;
                let yd = y1 - y_delta;
                if xu >= 0 && xu < cols && yu >= 0 && yu < rows {
                    seen.insert((xu, yu));
                }
                if xd >= 0 && xd < cols && yd >= 0 && yd < rows {
                    seen.insert((xd, yd));
                }
            }
        }
    }
    seen.len().to_string()
}

pub fn run_two(data: &str) -> String {
    let (node_map, rows, cols) = parse(data);
    let mut seen = HashSet::new();
    for coords in node_map.values() {
        for i in 0..coords.len() {
            for j in (i + 1)..coords.len() {
                let (x1, y1) = coords[i];
                let (x2, y2) = coords[j];
                let x_delta = x2 - x1;
                let y_delta = y2 - y1;
                let mut grow = 0;
                let mut found = true;
                while found {
                    found = false;
                    let xu = x2 + x_delta * grow;
                    let yu = y2 + y_delta * grow;
                    if xu >= 0 && xu < cols && yu >= 0 && yu < rows {
                        found = true;
                        seen.insert((xu, yu));
                    }
                    let xd = x1 - x_delta * grow;
                    let yd = y1 - y_delta * grow;
                    if xd >= 0 && xd < cols && yd >= 0 && yd < rows {
                        found = true;
                        seen.insert((xd, yd));
                    }
                    grow += 1;
                }
            }
        }
    }
    seen.len().to_string()
}
