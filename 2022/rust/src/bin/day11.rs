use std::str::FromStr;

use anyhow::Result;
use num_bigint::BigUint;

// const RELIEF: usize = 1;
const ROUNDS: usize = 10_000;

#[derive(Debug, Clone)]
enum Term {
    Old,
    Constant(BigUint),
}

impl Term {
    fn eval(&self, old: &BigUint) -> BigUint {
        match self {
            Term::Old => old.clone(),
            Term::Constant(c) => c.clone(),
        }
    }
}

#[derive(Debug, Clone)]
enum Operation {
    Add(Term, Term),
    Mul(Term, Term),
    Noop(),
}

impl Operation {
    fn eval(&self, old: &BigUint) -> BigUint {
        match self {
            Operation::Add(l, r) => l.eval(old) + r.eval(old),
            Operation::Mul(l, r) => l.eval(old) * r.eval(old),
            Operation::Noop() => 0_u64.into(),
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items_inspected: u64,
    items: Vec<BigUint>,
    inspector: Operation,
    divisor: BigUint,
    if_true: usize,
    if_false: usize,
}

fn parse_term(i: &str) -> Term {
    if i == "old" {
        Term::Old
    } else {
        Term::Constant(i.parse::<BigUint>().unwrap())
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
        let mut divisor: BigUint = 1_u64.into();
        let mut if_true = 0;
        let mut if_false = 0;
        for line in s.lines() {
            if line.trim().starts_with("Starting") {
                line.split_once(":").unwrap().1.split(",").for_each(|x| {
                    let i = x.trim().parse::<BigUint>().unwrap();
                    items.push(i);
                })
            }
            if line.trim().starts_with("Operation") {
                let tokens: Vec<&str> = line.split_once("=").unwrap().1.trim().split(" ").collect();
                let (l, op, r) = (tokens[0], tokens[1], tokens[2]);
                operation = Some(parse_operation(l, op, r));
            }
            if line.trim().starts_with("Test") {
                divisor = line.split(" ").last().unwrap().parse::<BigUint>().unwrap();
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

fn main() -> Result<()> {
    let mut monkeys: Vec<Monkey> = std::fs::read_to_string("./input11.prod")?
        .split("\n\n")
        .map(|monk| monk.parse::<Monkey>().unwrap())
        .collect();
    let mut cycles = 0;
    let zero: BigUint = 0_u64.into();
    while cycles < ROUNDS {
        for i in 0..monkeys.len() {
            let mc;
            {
                let monkey = &mut monkeys[i];
                mc = monkey.clone();
                monkey.items_inspected += mc.items.len() as u64;
            }
            for item in mc.items.iter() {
                let item = mc.inspector.eval(item);
                if item.clone() % mc.divisor.clone() == zero {
                    monkeys[mc.if_true].items.push(item);
                } else {
                    monkeys[mc.if_false].items.push(item);
                };
            }
            monkeys[i].items.clear();
        }
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
