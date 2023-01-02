
use fxhash::FxHashMap;

use z3::{Config, Context, Optimize, SatResult};
use z3::ast::{Ast, Int};

fn eval_expression<'a>(var_name: &'a str, expr_map: &FxHashMap<&str, (&'a str, u8, &'a str)>, expr_cache: &mut FxHashMap<&'a str, i64>) -> i64 {
    if !expr_cache.contains_key(var_name) {
        let &(dep_1, op, dep_2) = expr_map.get(var_name).unwrap();
        let dep_1_val = eval_expression(dep_1, expr_map, expr_cache);
        let dep_2_val = eval_expression(dep_2, expr_map, expr_cache);
        let var_val = match op {
            b'+' => {
                dep_1_val + dep_2_val
            },
            b'-' => {
                dep_1_val - dep_2_val
            },
            b'/' => {
                dep_1_val / dep_2_val
            },
            b'*' => {
                dep_1_val * dep_2_val
            },
            op => {
                panic!("Bad op {}", op);
            }
        };

        expr_cache.insert(var_name, var_val);
    }

    *expr_cache.get(var_name).unwrap()
}

fn generate_ast<'a>(context: &'a Context, var_name: &'a str, expr_map: &FxHashMap<&str, (&'a str, u8, &'a str)>, ast_cache: &mut FxHashMap<&'a str, Int<'a>>) -> Int<'a> {
    if !ast_cache.contains_key(var_name) {
        let &(dep_1, op, dep_2) = expr_map.get(var_name).unwrap();
        let dep_1_ast = generate_ast(context, dep_1, expr_map, ast_cache);
        let dep_2_ast = generate_ast(context, dep_2, expr_map, ast_cache);
        let var_ast = match op {
            b'+' => {
                dep_1_ast + dep_2_ast
            },
            b'-' => {
                dep_1_ast - dep_2_ast
            },
            b'/' => {
                dep_1_ast / dep_2_ast
            },
            b'*' => {
                dep_1_ast * dep_2_ast
            },
            op => {
                panic!("Bad op {}", op);
            }
        };

        ast_cache.insert(var_name, var_ast);
    }

    ast_cache.get(var_name).unwrap().clone()
}

pub fn part1(input: &str) -> i64 {
    let mut expr_map = FxHashMap::default();
    let mut expr_cache = FxHashMap::default();
    for line in input.lines() {
        let var_name = &line[..4];
        let expr = &line[6..];
        if expr.as_bytes()[0] >= b'0' && expr.as_bytes()[0] <= b'9' {
            let val = i64::from_str_radix(expr, 10).unwrap();
            expr_cache.insert(var_name, val);

        } else {
            let dep_1 = &expr[..4];
            let dep_2 = &expr[7..];
            let op = expr.as_bytes()[5];
            expr_map.insert(var_name, (dep_1, op, dep_2));
        }
    }

    eval_expression("root", &expr_map, &mut expr_cache)
}

pub fn part2(input: &str) -> i64 {
    let config = Config::new();
    let context = Context::new(&config);
    let solver = Optimize::new(&context);

    let mut expr_map = FxHashMap::default();
    let mut ast_cache = FxHashMap::default();
    let mut root_deps = ("", "");
    for line in input.lines() {
        let var_name = &line[..4];
        if var_name == "humn" {
            continue;
        }

        let expr = &line[6..];
        if expr.as_bytes()[0] >= b'0' && expr.as_bytes()[0] <= b'9' {
            let val = i64::from_str_radix(expr, 10).unwrap();
            ast_cache.insert(var_name, Int::from_i64(&context, val));

        } else {
            let dep_1 = &expr[..4];
            let dep_2 = &expr[7..];
            let op = expr.as_bytes()[5];
            if var_name == "root" {
                root_deps = (dep_1, dep_2);

            } else {
                expr_map.insert(var_name, (dep_1, op, dep_2));
            }
        }
    }

    ast_cache.insert("humn", Int::new_const(&context, "humn"));
    let dep_1_ast = generate_ast(&context, root_deps.0, &expr_map, &mut ast_cache);
    let dep_2_ast = generate_ast(&context, root_deps.1, &expr_map, &mut ast_cache);
    solver.assert(&dep_1_ast._eq(&dep_2_ast));
    
    assert_eq!(solver.check(&[]), SatResult::Sat);
    let model = solver.get_model().unwrap();
    let humn_val = model.eval(&ast_cache["humn"], true).unwrap();
    humn_val.as_i64().unwrap()
}