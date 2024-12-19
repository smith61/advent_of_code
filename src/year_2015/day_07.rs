use fxhash::FxHashMap;

#[derive(Debug, Clone, Copy)]
enum Value<'a> {
    Wire(&'a str),
    Constant(u16)
}

impl<'a> Value<'a> {

    pub fn from_str(value: &'a str) -> Self {
        if let Ok(constant) = value.parse::<u16>() {
            Value::Constant(constant)

        } else {
            Value::Wire(value)
        }
    }

}

#[derive(Debug, Clone, Copy)]
enum Operation<'a> {
    Constant(Value<'a>),
    And(Value<'a>, Value<'a>),
    Or(Value<'a>, Value<'a>),
    Not(Value<'a>),
    RShift(Value<'a>, Value<'a>),
    LShift(Value<'a>, Value<'a>)
}

fn eval_operator<'a>(connections: &FxHashMap<&'a str, Operation<'a>>, known_values: &mut FxHashMap<&'a str, u16>, wire: &'a str) -> u16 {
    match *connections.get(wire).unwrap() {
        Operation::Constant(c) => get_value(connections, known_values, c),
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
        Operation::Not(value) => {
            let value = get_value(connections, known_values, value);

            !value
        },
        Operation::RShift(left, right) => {
            let left = get_value(connections, known_values, left);
            let right = get_value(connections, known_values, right);

            left >> right
        }
        Operation::LShift(left, right) => {
            let left = get_value(connections, known_values, left);
            let right = get_value(connections, known_values, right);

            left << right
        }
    }
}

fn get_value<'a>(connections: &FxHashMap<&'a str, Operation<'a>>, known_values: &mut FxHashMap<&'a str, u16>, value: Value<'a>) -> u16 {
    match value {
        Value::Constant(c) => c,
        Value::Wire(wire) => {
            if !known_values.contains_key(wire) {
                let value = eval_operator(connections, known_values, wire);
                known_values.insert(wire, value);
            }

            *known_values.get(wire).unwrap()
        }
    }
}

pub fn part1(input: &str) -> u64 {
    let mut connections = FxHashMap::default();
    let mut known_values = FxHashMap::default();
    for line in input.trim().lines() {
        let mut parts = line.split(" -> ");

        let mut left_parts = parts.next().unwrap().split(" ");
        let right = parts.next().unwrap();

        let first = left_parts.next().unwrap();
        let operation = if first == "NOT" {
            let value = Value::from_str(left_parts.next().unwrap());
            Operation::Not(value)

        } else if let Some(op) = left_parts.next() {
            let left = Value::from_str(first);
            let right = Value::from_str(left_parts.next().unwrap());
            if op == "AND" {
                Operation::And(left, right)

            } else if op == "OR" {
                Operation::Or(left, right)

            } else if op == "RSHIFT" {
                Operation::RShift(left, right)

            } else if op == "LSHIFT" {
                Operation::LShift(left, right)

            } else {
                panic!("Unknown op '{}'", op);
            }

        } else {
            Operation::Constant(Value::from_str(first))
        };

        connections.insert(right, operation);
    }

    get_value(&connections, &mut known_values, Value::Wire("a")) as u64
}

pub fn part2(input: &str) -> u64 {
    let mut connections = FxHashMap::default();
    let mut known_values = FxHashMap::default();
    for line in input.trim().lines() {
        let mut parts = line.split(" -> ");

        let mut left_parts = parts.next().unwrap().split(" ");
        let right = parts.next().unwrap();

        let first = left_parts.next().unwrap();
        let operation = if first == "NOT" {
            let value = Value::from_str(left_parts.next().unwrap());
            Operation::Not(value)

        } else if let Some(op) = left_parts.next() {
            let left = Value::from_str(first);
            let right = Value::from_str(left_parts.next().unwrap());
            if op == "AND" {
                Operation::And(left, right)

            } else if op == "OR" {
                Operation::Or(left, right)

            } else if op == "RSHIFT" {
                Operation::RShift(left, right)

            } else if op == "LSHIFT" {
                Operation::LShift(left, right)

            } else {
                panic!("Unknown op '{}'", op);
            }

        } else {
            Operation::Constant(Value::from_str(first))
        };

        connections.insert(right, operation);
    }

    let new_b = get_value(&connections, &mut known_values, Value::Wire("a"));
    known_values.clear();
    known_values.insert("b", new_b);
    get_value(&connections, &mut known_values, Value::Wire("a")) as u64
}