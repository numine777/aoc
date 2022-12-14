use std::str::FromStr;

use anyhow::Result;

// const RELIEF: usize = 1;
const ROUNDS: usize = 10_000;

#[derive(Debug, Clone, Copy)]
enum Term {
    Old,
    Constant(u64),
}

impl Term {
    fn eval(self, old: u64) -> u64 {
        match self {
            Term::Old => old,
            Term::Constant(c) => c,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(Term, Term),
    Mul(Term, Term),
    Noop(),
}

impl Operation {
    fn eval(self, old: u64) -> u64 {
        match self {
            Operation::Add(l, r) => l.eval(old) + r.eval(old),
            Operation::Mul(l, r) => l.eval(old) * r.eval(old),
            Operation::Noop() => 0,
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items_inspected: u64,
    items: Vec<u64>,
    inspector: Operation,
    divisor: u64,
    if_true: usize,
    if_false: usize,
}

fn parse_term(i: &str) -> Term {
    if i == "old" {
        Term::Old
    } else {
        Term::Constant(i.parse::<u64>().unwrap())
    }
}

fn parse_operation(l: &str, op: &str, r: &str) -> Operation {
    match op {
        "+" => Operation::Add(parse_term(l), parse_term(r)),
        "*" => Operation::Mul(parse_term(l), parse_term(r)),
        _ => Operation::Noop(),
    }
}

impl FromStr for Monkey {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut items = Vec::new();
        let mut operation: Option<Operation> = None;
        let mut divisor: u64 = 1;
        let mut if_true = 0;
        let mut if_false = 0;
        for line in s.lines() {
            if line.trim().starts_with("Starting") {
                line.split_once(":").unwrap().1.split(",").for_each(|x| {
                    let i = x.trim().parse::<u64>().unwrap();
                    items.push(i);
                })
            }
            if line.trim().starts_with("Operation") {
                let tokens: Vec<&str> = line.split_once("=").unwrap().1.trim().split(" ").collect();
                let (l, op, r) = (tokens[0], tokens[1], tokens[2]);
                operation = Some(parse_operation(l, op, r));
            }
            if line.trim().starts_with("Test") {
                divisor = line.split(" ").last().unwrap().parse::<u64>().unwrap();
            }
            if line.trim().starts_with("If true") {
                if_true = line.split(" ").last().unwrap().parse::<usize>().unwrap();
            }
            if line.trim().starts_with("If false") {
                if_false = line.split(" ").last().unwrap().parse::<usize>().unwrap();
            }
        }
        if let Some(inspector) = operation {
            return Ok(Monkey {
                items_inspected: 0,
                items,
                inspector,
                divisor,
                if_true,
                if_false,
            });
        } else {
            println!("{:?}", operation);
            panic!("This sucks");
        }
    }
}

fn do_round(monkeys: &mut [Monkey], divisor_product: u64) {
    let num_monkeys = monkeys.len();
    for i in 0..num_monkeys {
        let mc;
        {
            let monkey = &mut monkeys[i];
            mc = monkey.clone();
            monkey.items_inspected += mc.items.len() as u64;
        }
        for mut item in mc.items.iter().copied() {
            item %= divisor_product;
            item = mc.inspector.eval(item);
            if item % mc.divisor == 0 {
                monkeys[mc.if_true].items.push(item);
            } else {
                monkeys[mc.if_false].items.push(item);
            };
        }
        monkeys[i].items.clear();
    }
}

fn main() -> Result<()> {
    let mut monkeys: Vec<Monkey> = std::fs::read_to_string("./input11.prod")?
        .split("\n\n")
        .map(|monk| monk.parse::<Monkey>().unwrap())
        .collect();
    let mut cycles = 0;
    let divisor_product = monkeys.iter().map(|m| m.divisor).product::<u64>();
    for _ in 0..ROUNDS {
        do_round(&mut monkeys, divisor_product);
        cycles += 1;
        if cycles % 100 == 0 {
            println!("At cycle: {:?}", cycles);
        }
    }
    let mut part_one: Vec<u64> = monkeys.iter().map(|monk| monk.items_inspected).collect();
    part_one.sort_by(|a, b| b.cmp(a));
    let mut part_one_iter = part_one.iter().take(2);
    let total = part_one_iter.next().unwrap() * part_one_iter.next().unwrap();
    println!("{:?}", total);
    return Ok(());
}
