fn parse(data: &str) -> Vec<usize> {
    data.trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
}

pub fn checksum_value(start: usize, len: usize, val: usize) -> usize {
    val * (((start * 2 + len - 1) * len) / 2)
}

pub fn run_one(data: &str) -> String {
    let mut blocks = parse(data);
    let mut checksum: usize = 0;
    let mut pos = 0;
    let mut s_idx = 0;
    let mut e_idx = if blocks.len() % 2 == 0 {
        blocks.len() - 2
    } else {
        blocks.len() - 1
    };
    while s_idx <= e_idx {
        checksum += checksum_value(pos, blocks[s_idx], s_idx / 2);
        pos += blocks[s_idx];
        s_idx += 2;
        let mut space = blocks[s_idx - 1];
        while space > 0 && s_idx <= e_idx {
            let last = blocks[e_idx];
            if space > last {
                checksum += checksum_value(pos, last, e_idx / 2);
                pos += last;
                space -= last;
                e_idx -= 2;
            } else {
                checksum += checksum_value(pos, space, e_idx / 2);
                pos += space;
                blocks[e_idx] -= space;
                space = 0;
            }
        }
    }
    checksum.to_string()
}

pub fn run_two(data: &str) -> String {
    let mut blocks = parse(data);
    let mut labeled = Vec::new();
    let mut idx = 0;
    while idx < blocks.len() {
        labeled.push((idx / 2, blocks[idx]));
        idx += 1;
        if idx < blocks.len() {
            labeled.push((0, blocks[idx]));
            idx += 1;
        }
    }
    if labeled.len() % 2 == 0 {
        labeled.pop();
    }
    let mut file_idx = labeled.len() - 1;
    while file_idx > 0 {
        let (f_val, f_len) = labeled[file_idx];
        let mut moved = false;
        for space_idx in (1..file_idx).step_by(2) {
            let (_, s_len) = labeled[space_idx];
            if s_len >= f_len {
                labeled[file_idx].0 = 0;
                labeled[space_idx] = (0, s_len - f_len);
                labeled.insert(space_idx, (f_val, f_len));
                labeled.insert(space_idx, (0, 0));
                moved = true;
                break;
            }
        }
        if !moved {
            file_idx -= 2;
        }
    }
    let mut checksum = 0;
    let mut pos = 0;
    for (val, len) in labeled {
        checksum += checksum_value(pos, len, val);
        pos += len;
    }
    checksum.to_string()
}
