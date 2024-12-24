use std::collections::HashMap;

fn parse(data: &str) -> (Vec<(&str, &str)>, HashMap<&str, (&str, &str, &str)>) {
    let (inputs, gates) = data.split_once("\n\n").unwrap();
    let regs = inputs
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(": ").unwrap();
            (left, right)
        })
        .collect();
    let gate_regex = regex::Regex::new(r#"(.*) (AND|OR|XOR) (.*) -> (.*)"#).unwrap();
    let gates = gates
        .lines()
        .map(|line| {
            let (_, [rega, op, regb, regc]) = gate_regex.captures(line).unwrap().extract();
            (regc, (rega, op, regb))
        })
        .collect();
    (regs, gates)
}

fn populate_gate<'s>(
    gates: &'s HashMap<&str, (&str, &str, &str)>,
    map: &mut HashMap<&'s str, bool>,
    key: &'s str,
) {
    if map.contains_key(key) {
        return;
    }
    let (rega, op, regb) = gates[key];
    populate_gate(gates, map, rega);
    populate_gate(gates, map, regb);
    map.insert(
        key,
        match op {
            "AND" => map[rega] & map[regb],
            "OR" => map[rega] | map[regb],
            "XOR" => map[rega] ^ map[regb],
            _ => unreachable!(),
        },
    );
}

pub fn run_one(data: &str) -> String {
    let (inputs, gates) = parse(data);
    let mut out_map: HashMap<&str, bool> = HashMap::from_iter(
        inputs
            .iter()
            .map(|&(reg, val)| (reg, val.parse::<usize>().unwrap() != 0)),
    );
    let mut z_regs = gates
        .iter()
        .filter_map(|(&gate, _)| {
            if gate.starts_with('z') {
                Some(gate)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    z_regs.sort();
    z_regs.reverse();
    for &reg in &z_regs {
        populate_gate(&gates, &mut out_map, reg);
    }
    z_regs
        .iter()
        .fold(0, |accum, &reg| 2 * accum + (out_map[&reg] as usize))
        .to_string()
}

fn find_deps<'s>(gates: &HashMap<&'s str, (&'s str, &str, &'s str)>, key: &str) -> Vec<&'s str> {
    let mut inputs = Vec::new();
    let (rega, _, regb) = gates[key];
    if rega.starts_with('x') || rega.starts_with('y') {
        inputs.push(rega);
    } else {
        inputs.extend(find_deps(gates, rega));
    }
    if regb.starts_with('x') || regb.starts_with('y') {
        inputs.push(regb);
    } else {
        inputs.extend(find_deps(gates, regb));
    }
    inputs
}

pub fn run_two(data: &str) -> String {
    let (inputs, gates) = parse(data);
    let mut viz_out = String::new();
    for &(input, _) in &inputs {
        viz_out.push_str(&format!("start -> {};\n", input));
    }
    for (out, (lg, op, rg)) in gates {
        let gate = format!("{}_{}", out, op);
        viz_out.push_str(&format!("{} -> {};\n{} -> {};\n{} -> {};\n", lg, gate, rg, gate, gate, out));
        if out.starts_with('z') {
            viz_out.push_str(&format!("{} -> end;\n", out));
        }
    }
    format!("\n{}", viz_out)
}
