use std::collections::HashMap;

const KEYPAD: &str = "789\n456\n123\n#0A";

const ARROWS: &str = "#^A\n<v>";

fn parse(data: &str) -> Vec<Vec<char>> {
    data.lines().map(|l| l.trim().chars().collect()).collect()
}

fn build_pairs(code: &[char]) -> Vec<(char, char)> {
    (0..code.len())
        .fold(('A', Vec::new()), |(last, mut vec), i| {
            let next = code[i];
            vec.push((last, next));
            (next, vec)
        })
        .1
}

fn nav_guide(keys: &str) -> HashMap<(char, char), Vec<(char, char)>> {
    let lines = parse(keys);
    let (bad_row, bad_col) = lines
        .iter()
        .enumerate()
        .find_map(|(i, l)| {
            l.iter()
                .enumerate()
                .find(|(_, &c)| c == '#')
                .map(|(j, _)| (i, j))
        })
        .unwrap();
    let mut record = HashMap::new();
    for s_row in 0..lines.len() {
        for s_col in 0..lines[0].len() {
            if lines[s_row][s_col] == '#' {
                continue;
            }
            for e_row in 0..lines.len() {
                for e_col in 0..lines[0].len() {
                    if lines[e_row][e_col] == '#' {
                        continue;
                    }
                    let mut seq = Vec::new();
                    if s_row == bad_row && e_col == bad_col {
                        for _ in 0..s_row.saturating_sub(e_row) {
                            seq.push('^')
                        }
                        for _ in 0..e_row.saturating_sub(s_row) {
                            seq.push('v')
                        }
                        for _ in 0..s_col.saturating_sub(e_col) {
                            seq.push('<')
                        }
                        for _ in 0..e_col.saturating_sub(s_col) {
                            seq.push('>')
                        }
                    } else if s_col == bad_col && e_row == bad_row {
                        for _ in 0..s_col.saturating_sub(e_col) {
                            seq.push('<')
                        }
                        for _ in 0..e_col.saturating_sub(s_col) {
                            seq.push('>')
                        }
                        for _ in 0..s_row.saturating_sub(e_row) {
                            seq.push('^')
                        }
                        for _ in 0..e_row.saturating_sub(s_row) {
                            seq.push('v')
                        }
                    } else {
                        // Perfection
                        for _ in 0..s_col.saturating_sub(e_col) {
                            seq.push('<')
                        }
                        for _ in 0..e_row.saturating_sub(s_row) {
                            seq.push('v')
                        }
                        for _ in 0..s_row.saturating_sub(e_row) {
                            seq.push('^')
                        }
                        for _ in 0..e_col.saturating_sub(s_col) {
                            seq.push('>')
                        }
                    }
                    seq.push('A');
                    record.insert(
                        (lines[s_row][s_col], lines[e_row][e_col]),
                        build_pairs(&seq),
                    );
                }
            }
        }
    }
    record
}

fn seq(code: &[(char, char)], map: &HashMap<(char, char), Vec<(char, char)>>) -> Vec<(char, char)> {
    let mut order = Vec::new();
    for pair in code {
        order.extend(&map[pair]);
    }
    order
}

fn memo_run(
    memo: &mut HashMap<(usize, (char, char)), usize>,
    guide: &HashMap<(char, char), Vec<(char, char)>>,
    order: &[(char, char)],
    count: usize,
) -> usize {
    if count == 0 {
        return order.len();
    }
    let mut total = 0;
    for &pair in order {
        total += match memo.get(&(count, pair)) {
            Some(&n) => n,
            None => {
                let val = memo_run(memo, guide, &guide[&pair], count - 1);
                memo.insert((count, pair), val);
                val
            }
        };
    }
    total
}

fn run(keypad_order: &[(char, char)], count: usize) -> usize {
    let arrows_nav = nav_guide(ARROWS);
    let mut memo = HashMap::<(usize, (char, char)), usize>::new();
    memo_run(&mut memo, &arrows_nav, keypad_order, count)
}

pub fn run_one(data: &str) -> String {
    let codes = parse(data);
    let keypad_nav = nav_guide(KEYPAD);
    codes
        .iter()
        .map(|code| {
            let base_order = seq(&build_pairs(code), &keypad_nav);
            let mult = code[..(code.len() - 1)]
                .iter()
                .collect::<String>()
                .parse::<usize>()
                .unwrap();
            run(&base_order, 2) * mult
        })
        .sum::<usize>()
        .to_string()
}

pub fn run_two(data: &str) -> String {
    let codes = parse(data);
    let keypad_nav = nav_guide(KEYPAD);
    codes
        .iter()
        .map(|code| {
            let base_order = seq(&build_pairs(code), &keypad_nav);
            let mult = code[..(code.len() - 1)]
                .iter()
                .collect::<String>()
                .parse::<usize>()
                .unwrap();
            run(&base_order, 25) * mult
        })
        .sum::<usize>()
        .to_string()
}
