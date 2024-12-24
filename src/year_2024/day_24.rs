use fxhash::FxHashMap;

#[derive(Debug, Clone, Copy)]
enum Operation<'a> {
    And(&'a str, &'a str),
    Or(&'a str, &'a str),
    Xor(&'a str, &'a str)
}

fn eval_operator<'a>(connections: &FxHashMap<&'a str, Operation<'a>>, known_values: &mut FxHashMap<&'a str, u8>, wire: &'a str) -> u8 {
    match *connections.get(wire).unwrap() {
        Operation::And(left, right) => {
            let left = get_value(connections, known_values, left);
            let right = get_value(connections, known_values, right);

            left & right
        },
        Operation::Or(left, right) => {
            let left = get_value(connections, known_values, left);
            let right = get_value(connections, known_values, right);

            left | right
        },
        Operation::Xor(left, right) => {
            let left = get_value(connections, known_values, left);
            let right = get_value(connections, known_values, right);

            left ^ right
        }
    }
}

fn get_value<'a>(connections: &FxHashMap<&'a str, Operation<'a>>, known_values: &mut FxHashMap<&'a str, u8>, wire: &'a str) -> u8 {
    if !known_values.contains_key(wire) {
        let value = eval_operator(connections, known_values, wire);
        known_values.insert(wire, value);
    }

    *known_values.get(wire).unwrap()
}

pub fn part1(input: &str) -> u64 {
    let mut lines = input.trim().lines();

    let mut line_values = FxHashMap::default();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        let mut parts = line.split(": ");
        let name = parts.next().unwrap();
        let value = parts.next().unwrap().parse::<u8>().unwrap();
        line_values.insert(name, value);
    }

    let mut connections = FxHashMap::default();
    while let Some(line) = lines.next() {
        let mut parts = line.split(" -> ");

        let mut left_parts = parts.next().unwrap().split(" ");
        let e = parts.next().unwrap();

        let left = left_parts.next().unwrap();
        let op = left_parts.next().unwrap();
        let right = left_parts.next().unwrap();
        let op = if op == "AND" {
                Operation::And(left, right)

            } else if op == "OR" {
                Operation::Or(left, right)

            } else if op == "XOR" {
                Operation::Xor(left, right)

            } else {
                panic!("Unknown op '{}'", op);
            };

        connections.insert(e, op);
    }

    let mut result = 0;
    for &wire in connections.keys() {
        if !wire.starts_with("z") {
            continue;
        }

        let bit_index = (&wire[1..]).parse::<u8>().unwrap();

        let value = get_value(&connections, &mut line_values, wire);
        result |= (value as u64) << bit_index;
    }

    result
}

#[derive(Clone, Copy, Debug)]
struct Gate<'a> {
    left_input: &'a str,
    right_input: &'a str,
    op: &'a str,
    output: &'a str
}

impl<'a> Gate<'a> {

    pub fn from_line(line: &'a str) -> Self {
        let mut parts = line.split(" -> ");

        let mut left_parts = parts.next().unwrap().split(" ");
        let output = parts.next().unwrap();

        let left_input = left_parts.next().unwrap();
        let op = left_parts.next().unwrap();
        let right_input = left_parts.next().unwrap();

        Self {
            left_input,
            right_input,
            op,
            output
        }
    }

    pub fn has_input(&self, input: &str) -> bool {
        self.left_input == input || self.right_input == input
    }

    pub fn is_direct_input(&self) -> bool {
        self.left_input.starts_with("x") ||
        self.right_input.starts_with("x")
    }

    pub fn is_or(&self) -> bool {
        self.op == "OR"
    }

    pub fn is_xor(&self) -> bool {
        self.op == "XOR"
    }

    pub fn outputs_to_z(&self) -> bool {
        self.output.starts_with("z")
    }

}

pub fn part2(input: &str) -> String {
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
    }

    let mut gates = Vec::new();
    while let Some(line) = lines.next() {
        gates.push(Gate::from_line(line));
    }

    let max_z_wire =
        gates.iter()
             .filter(|g| g.outputs_to_z())
             .map(|g| g.output)
             .max()
             .unwrap();

    let mut errors = Vec::new();

    //
    // Root add gates are xor gates in which both inputs are an x or y wire.
    // Only the gate with input x00 and y00 should output to z00. All other
    // gates should not output to any z wire.
    //

    let root_add_gates =
        gates.iter()
             .filter(|g| g.is_direct_input() && g.is_xor())
             .collect::<Vec<_>>();

    for &gate in &root_add_gates {
        if gate.has_input("x00") {
            assert!(gate.has_input("y00"));
            if gate.output != "z00" {
                errors.push(gate.output);
            }

            continue;

        } else if gate.outputs_to_z() {
            errors.push(gate.output);
        }
    }

    //
    // Carry add gates are xor gates in which both inputs are not an x or y wire.
    // These gates should add the output of a root add gate, with the carry of
    // the previous bit. All of these gates should output to a z wire.
    //

    let carry_add_gates =
        gates.iter()
             .filter(|g| !g.is_direct_input() && g.is_xor())
             .collect::<Vec<_>>();

    for &gate in &carry_add_gates {
        if !gate.outputs_to_z() {
            errors.push(gate.output);
        }
    }

    //
    // All gates that write to a z wire should be an xor gate, except for the last
    // z wire which should be an or gate.
    //

    for gate in gates.iter().filter(|g| g.outputs_to_z()) {
        if gate.output == max_z_wire {
            if !gate.is_or() {
                errors.push(gate.output);
            }

        } else if !gate.is_xor() {
            errors.push(gate.output);
        }
    }

    //
    // Try to find any root add gates who's output is not an input to a carry add gate.
    //

    let mut unmatched_root_gates = Vec::new();
    for &root_add_gate in &root_add_gates {
        if errors.contains(&root_add_gate.output) {
            continue;
        }

        if root_add_gate.output == "z00" {
            continue;
        }

        if carry_add_gates.iter().find(|g| g.has_input(root_add_gate.output)).is_none() {
            unmatched_root_gates.push(root_add_gate);
            errors.push(root_add_gate.output);
        }
    }

    for unmatched_root_gate in unmatched_root_gates {
        assert!(unmatched_root_gate.left_input.starts_with("x") || unmatched_root_gate.right_input.starts_with("x"));
        assert!(unmatched_root_gate.left_input.starts_with("y") || unmatched_root_gate.right_input.starts_with("y"));

        //
        // Find the expected carry gate that this unmatched root gate should output to.
        //

        let intended_zwire = format!("z{}", &unmatched_root_gate.left_input[1..]);
        let intended_carry_gate = {
            let mut matches =
                carry_add_gates
                    .iter()
                    .filter(|g| g.output == &intended_zwire);

            if let Some(matched_gate) = matches.next() {
                assert!(matches.next().is_none());

                matched_gate

            } else {
                panic!("Failed to find match for {}", intended_zwire);
            }
        };

        let expected_gate = {
            let mut matches =
                gates.iter()
                     .filter(|g| g.is_or())
                     .filter(|g| intended_carry_gate.has_input(g.output));

            if let Some(expected_gate) = matches.next() {
                assert!(matches.next().is_none());

                expected_gate

            } else {
                panic!("Failed to find match for {:?}", intended_carry_gate);
            }

        };

        if expected_gate.output == intended_carry_gate.left_input {
            errors.push(intended_carry_gate.right_input);

        } else {
            errors.push(intended_carry_gate.left_input);
        }
    }

    errors.sort();
    errors.dedup();
    assert_eq!(errors.len(), 8);
    errors.join(",")
}
