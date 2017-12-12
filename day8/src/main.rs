extern crate regex;

use std::collections::HashMap;
use regex::Regex;

#[derive(Debug, Clone)]
struct Instruction<'s> {
    reg: &'s str,
    change: Change,
    amount: isize,
    condition: Condition<'s>,
}

impl<'s> Instruction<'s> {
    fn exec(&self, registers: &mut HashMap<&'s str, isize>, known_highest: &mut isize) {
        if !self.condition.test(registers) {
            return;
        }

        let reg = registers.entry(self.reg).or_insert(0);
        self.change.apply(reg, self.amount);
        *known_highest = std::cmp::max(*known_highest, *reg);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Change {
    Inc,
    Dec,
}

impl Change {
    fn apply(&self, value: &mut isize, amount: isize) {
        match *self {
            Change::Inc => { *value += amount; },
            Change::Dec => { *value -= amount; },
        }
    }
}

#[derive(Debug, Clone)]
struct Condition<'s> {
    reg: &'s str,
    comparison: Comparison,
    value: isize,
}

impl<'s> Condition<'s> {
    fn test(&self, registers: &HashMap<&str, isize>) -> bool {
        let &reg_value = registers.get(self.reg).unwrap_or(&0);
        self.comparison.test(reg_value, self.value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Comparison {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

impl Comparison {
    fn test(&self, left: isize, right: isize) -> bool {
        match *self {
            Comparison::Eq => left == right,
            Comparison::Ne => left != right,
            Comparison::Lt => left < right,
            Comparison::Le => left <= right,
            Comparison::Gt => left > right,
            Comparison::Ge => left >= right,
        }
    }
}

fn parse_instructions<'s>(input: &'s str) -> Vec<Instruction<'s>> {
    let regex = Regex::new(
        r"([a-z]+) (inc|dec) (-?[0-9]+) if ([a-z]+) (==|!=|<=?|>=?) (-?[0-9]+)"
    ).unwrap();

    let mut result = Vec::new();
    for line in input.lines() {
        let m = regex.captures(line).unwrap();
        let reg = m.get(1).unwrap().as_str();
        let change = match m.get(2).unwrap().as_str() {
            "inc" => Change::Inc,
            "dec" => Change::Dec,
            s => panic!("Invalid instruction: {}", s),
        };
        let amount = m.get(3).unwrap().as_str().parse::<isize>().unwrap();

        let cond_reg = m.get(4).unwrap().as_str();
        let comparison = match m.get(5).unwrap().as_str() {
            "==" => Comparison::Eq,
            "!=" => Comparison::Ne,
            "<" => Comparison::Lt,
            "<=" => Comparison::Le,
            ">" => Comparison::Gt,
            ">=" => Comparison::Ge,
            s => panic!("Invalid condition operator: {}", s),
        };
        let cond_value = m.get(6).unwrap().as_str().parse::<isize>().unwrap();

        let instr = Instruction {
            reg,
            change,
            amount,
            condition: Condition {
                reg: cond_reg,
                comparison,
                value: cond_value,
            },
        };
        result.push(instr);
    }
    result
}

fn main() {
    let input = include_str!("input.txt");
    let instructions = parse_instructions(input);

    let mut part2 = 0;
    let mut registers = HashMap::new();
    for instr in instructions.iter() {
        instr.exec(&mut registers, &mut part2);
    }
    let part1 = registers.values().fold(0, |a, &b| std::cmp::max(a, b));
    println!("Part 1 = {}", part1);
    println!("Part 2 = {}", part2);
}
