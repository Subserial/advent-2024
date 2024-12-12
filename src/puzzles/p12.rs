use std::collections::{HashMap, HashSet};

fn parse(data: &str) -> Vec<Vec<char>> {
    data.lines().map(|line| line.chars().collect()).collect()
}

fn run(garden: &Vec<Vec<char>>) -> (HashMap<usize, HashSet<(usize, usize)>>) {
    let mut gardens: HashMap<usize, HashSet<(usize, usize)>> = HashMap::new();
    let mut plot_map: HashMap<(usize, usize), usize> = HashMap::new();
    let mut next_id = 0;
    for row in 0..garden.len() {
        for col in 0..garden[0].len() {
            let c = garden[row][col];
            let left_match = row > 0 && garden[row - 1][col] == c;
            let up_match = col > 0 && garden[row][col - 1] == c;
            if left_match && up_match && plot_map[&(row - 1, col)] != plot_map[&(row, col - 1)] {
                let first_id = plot_map[&(row - 1, col)];
                let other_id = plot_map[&(row, col - 1)];
                let other_garden = gardens.remove(&other_id).unwrap();
                for other in &other_garden {
                    plot_map.insert(*other, first_id);
                }
                gardens.get_mut(&first_id).unwrap().extend(other_garden);
            }
            let matching = if left_match {
                plot_map[&(row - 1, col)]
            } else if up_match {
                plot_map[&(row, col - 1)]
            } else {
                let id = next_id;
                gardens.insert(id, HashSet::new());
                next_id += 1;
                id
            };
            plot_map.insert((row, col), matching);
            gardens.get_mut(&matching).unwrap().insert((row, col));
        }
    }
    gardens
}

pub fn run_one(data: &str) -> String {
    let garden = parse(data);
    let plots = run(&garden);
    let mut total = 0;
    for (_, plot) in plots {
        let area = plot.len();
        let mut peri = 0;
        for (row, col) in plot {
            let c = garden[row][col];
            if row == 0 || garden[row - 1][col] != c {
                peri += 1;
            }
            if col == 0 || garden[row][col - 1] != c {
                peri += 1;
            }
            if row == garden.len() - 1 || garden[row + 1][col] != c {
                peri += 1;
            }
            if col == garden[0].len() - 1 || garden[row][col + 1] != c {
                peri += 1;
            }
        }
        total += area * peri;
    }
    total.to_string()
}

pub fn run_two(data: &str) -> String {
    let garden = parse(data);
    let plots = run(&garden);
    let mut total = 0;
    for (_, plot) in plots {
        let (row_lo, row_hi, col_lo, col_hi) = plot.iter().fold(
            (usize::MAX, usize::MIN, usize::MAX, usize::MIN),
            |(row_lo, row_hi, col_lo, col_hi), (row, col)| {
                let row_lo = row_lo.min(*row);
                let row_hi = row_hi.max(*row);
                let col_lo = col_lo.min(*col);
                let col_hi = col_hi.max(*col);
                (row_lo, row_hi, col_lo, col_hi)
            },
        );
        let mut walls = 0;
        for row in row_lo..=row_hi {
            let mut on_top = false;
            let mut on_bot = false;
            for col in col_lo..=col_hi {
                let this_top =
                    plot.contains(&(row, col)) && !(row > 0 && plot.contains(&(row - 1, col)));
                let this_bot = plot.contains(&(row, col))
                    && !(row < garden.len() - 1 && plot.contains(&(row + 1, col)));
                if on_top && !this_top {
                    on_top = false;
                    walls += 1;
                } else if !on_top && this_top {
                    on_top = true;
                }
                if on_bot && !this_bot {
                    on_bot = false;
                    walls += 1;
                } else if !on_bot && this_bot {
                    on_bot = true;
                }
            }
            if on_top {
                walls += 1;
            }
            if on_bot {
                walls += 1;
            }
        }
        for col in col_lo..=col_hi {
            let mut on_left = false;
            let mut on_right = false;
            for row in row_lo..=row_hi {
                let this_left =
                    plot.contains(&(row, col)) && !(col > 0 && plot.contains(&(row, col - 1)));
                let this_right = plot.contains(&(row, col))
                    && !(col < garden[0].len() - 1 && plot.contains(&(row, col + 1)));
                if on_left && !this_left {
                    on_left = false;
                    walls += 1;
                } else if !on_left && this_left {
                    on_left = true;
                }
                if on_right && !this_right {
                    on_right = false;
                    walls += 1;
                } else if !on_right && this_right {
                    on_right = true;
                }
            }
            if on_left {
                walls += 1;
            }
            if on_right {
                walls += 1;
            }
        }
        total += plot.len() * walls;
    }
    total.to_string()
}
