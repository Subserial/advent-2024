enum State {
    Start,
    First(u32),
    Increasing(u32),
    Decreasing(u32),
    Fault,
}

fn fold_state(state: State, next: u32) -> State {
    match state {
        State::Start => State::First(next),
        State::First(prev) => {
            if next > prev && next - prev <= 3 {
                State::Increasing(next)
            } else if next < prev && prev - next <= 3 {
                State::Decreasing(next)
            } else {
                State::Fault
            }
        }
        State::Increasing(prev) => {
            if next > prev && next - prev <= 3 {
                State::Increasing(next)
            } else {
                State::Fault
            }
        }
        State::Decreasing(prev) => {
            if next < prev && prev - next <= 3 {
                State::Decreasing(next)
            } else {
                State::Fault
            }
        }
        State::Fault => State::Fault,
    }
}

pub fn run_one(data: &str) -> String {
    data.lines()
        .map(|line| {
            match line
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .fold(State::Start, fold_state)
            {
                State::Fault => 0,
                _ => 1,
            }
        })
        .sum::<u32>()
        .to_string()
}

pub fn run_two(data: &str) -> String {
    data.lines()
        .map(|line| {
            let data = line
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            for i in 0..data.len() {
                match data
                    .iter()
                    .enumerate()
                    .filter(|(j, n)| i != *j)
                    .map(|(j, n)| *n)
                    .fold(State::Start, fold_state)
                {
                    State::Fault => (),
                    _ => return 1,
                }
            }
            return 0;
        })
        .sum::<u32>()
        .to_string()
}
