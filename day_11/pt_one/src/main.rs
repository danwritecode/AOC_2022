use anyhow::Result;
use scan_fmt::scan_fmt_some;
use std::ops::Div;
// use itertools::Itertools;

#[derive(Debug)]
struct Monkey {
    items: Vec<i32>,
    operation: char,
    operation_value: i32,
    worry_level: i32,
    old_worry_level: i32,
    mod_test: i32,
    true_target: i32,
    false_target: i32
}

fn main() -> Result<()> {
    let input = include_str!("../input").split("\n\n");
    let input:Vec<Vec<&str>> = input
        .map(|l| {
            return l
                .lines()
                .collect();
        })
        .collect();

    let mut monkeys: Vec<Monkey> = parse_monkeys(input);
    let num_rounds = 1;

    for _round in 0..num_rounds {
        for mut monkey in &mut monkeys {
            for item in &monkey.items {
                if monkey.operation == '*' {
                    monkey.worry_level = item * monkey.operation_value;
                }
                if monkey.operation == '+' {
                    monkey.worry_level = item + monkey.operation_value;
                }

                let worry_div = (&monkey.worry_level/3).div(1);
                let worry_mod = worry_div % monkey.mod_test;

                if worry_mod == 0 {
                    monkeys[monkey.true_target as usize].items.push(worry_div);
                } else {
                    monkeys[monkey.false_target as usize].items.push(worry_div);
                }
            }
        }
    };

    dbg!(monkeys);
    return Ok(());
}

fn parse_monkeys(input:Vec<Vec<&str>>) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = vec![];

    for (b, block) in input.iter().enumerate() {
        monkeys.push(create_blank_monkey());
        for (l, line) in block.iter().enumerate() {
            match l {
                1 => {
                    monkeys[b].items = line
                        .split(":")
                        .skip(1)
                        .flat_map(|x| {
                            return x
                                .split(",")
                                .map(|n| n.trim().parse::<i32>().unwrap())
                        })
                        .collect();
                },
                2 => {
                    let (op, value) = scan_fmt_some!(line, "  Operation: new = old {} {}", char, i32);
                    match value {
                        Some(_) => monkeys[b].operation_value = value.unwrap(),
                        None => monkeys[b].operation_value = 0
                    }

                    monkeys[b].operation = op.unwrap();
                }
                3 => monkeys[b].mod_test = scan_fmt_some!(line, "  Test: divisible by {}", i32).unwrap(),
                4 => monkeys[b].true_target = scan_fmt_some!(line, "    If true: throw to monkey {}", i32).unwrap(),
                5 => monkeys[b].false_target = scan_fmt_some!(line, "    If false: throw to monkey {}", i32).unwrap(),
                _ => ()
            }
        }
    };

    return monkeys;
}

fn create_blank_monkey() -> Monkey {
    return Monkey {
        items: vec![],
        operation: 'a',
        operation_value: 0,
        worry_level: 0,
        old_worry_level: 0,
        mod_test: 0,
        true_target: 0,
        false_target: 0
    }
}

