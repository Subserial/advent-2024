use std::ops::BitXor;

pub fn parse(data: &str) -> (usize, usize, usize, Vec<usize>) {
    let (reg, seq) = data.split_once("\n\n").unwrap();
    let [a, b, c] = reg.lines().map(|l| l.split_once(": ").unwrap().1.parse().unwrap()).collect::<Vec<_>>()[..] else {
        panic!()
    };
    let seq = seq.trim().strip_prefix("Program: ").unwrap().split(',').map(|n| n.parse().unwrap()).collect();
    (a, b, c, seq)
}

fn combo(literal: usize, a: usize, b: usize, c: usize) -> usize {
    match literal {
        0..=3 => literal,
        4 => a,
        5 => b,
        6 => c,
        _ => panic!(),
    }
}

pub fn run(seq: &[usize], a: usize, b: usize, c: usize) -> Vec<usize> {
    let mut a = a;
    let mut b = b;
    let mut c = c;
    let mut idx = 0;
    let mut out = Vec::new();
    while idx < seq.len() {
        let (op, literal) = (seq[idx], seq[idx+1]);
        match op {
            0 => {
                let d = combo(literal, a, b, c);
                a = a >> d;
            },
            1 => {
                b = b.bitxor(literal);
            },
            2 => {
                let d = combo(literal, a, b, c);
                b = d % 8;
            },
            3 => {
                if a != 0 {
                    idx = literal;
                    continue
                }
            },
            4 => {
                b = b.bitxor(c);
            },
            5 => {
                let d = combo(literal, a, b, c);
                out.push(d % 8);
            },
            6 => {
                let d = combo(literal, a, b, c);
                b = a >> d;
            },
            7 => {
                let d = combo(literal, a, b, c);
                c = a >> d;
            },
            _ => panic!(),
        }
        idx += 2;
    }
    out
}

pub fn run_one(data: &str) -> String {
    let (a, b, c, seq) = parse(data);
    let out = run(&seq, a, b, c);
    out.iter().map(|d| d.to_string()).collect::<Vec<_>>().join(",")
}

fn build(seq: &[usize], alg: &[usize], idx: usize, leading: usize) -> Option<usize> {
    let want = seq[idx];
    for guess in 0..8 {
        let a = leading * 8 + guess;
        if run(alg, a, 0, 0).pop().unwrap() == want {
            if idx == 0 {
                return Some(a)
            }
            if let Some(val) = build(seq, alg, idx - 1, a) {
                return Some(val)
            }
        }
    }
    None
}

pub fn run_two(data: &str) -> String {
    let (_, _, _, seq) = parse(data);
    match seq.iter().skip(seq.len() - 4).collect::<Vec<_>>()[..] {
        [5,_,3,0] => (),
        _ => return "Sequence doesn't end in print and loop".to_string(),
    };
    let alg = &seq[..seq.len() - 2];
    build(&seq, alg, seq.len() - 1, 0).unwrap_or(0).to_string()
}