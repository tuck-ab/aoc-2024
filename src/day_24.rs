use std::collections::{BTreeMap, BTreeSet};

use itertools::Itertools;
use regex::Regex;

use crate::tools::load_input;
use crate::Solution;

pub(crate) struct Day24;

impl Solution for Day24 {
    fn part_1() -> String {
        let data = load_input(24);
        let (mut values, ops, zregisters, _) = parse_data(data);

        let mut num = 0;
        for zreg in zregisters.iter().rev() {
            num = (num << 1) + get_value(zreg, &mut values, &ops) as u64;
        }

        num.to_string()
    }

    fn part_2() -> String {
        let data = load_input(24);
        let (_, ops, _, occs) = parse_data(data);

        for (result, (lhs, rhs, op)) in ops.iter() {
            if *op == Op::And {
                if *occs.get(result).unwrap() != 2 {
                    println!("{} {:?} {} {}", lhs, *op, rhs, result)
                }
            }

            if *op != Op::Xor {
                if result[..1] == *"z" {
                    println!("{} {:?} {} {}", lhs, *op, rhs, result)
                }
            }
        }

        /*

        Look at the ones that are deffo wrong from the above, then manually
        work out which value should be there to replace and get:

        nbd <-> kbs
        z20 <-> tqq

        z06, z39 <-> ksv, ckb (one swaps the other don't remember which)

        */

        let answer: BTreeSet<&str> =
            BTreeSet::from(["nbd", "kbs", "z20", "tqq", "ksv", "ckb", "z39", "z06"]);

        answer.iter().join(",").to_string()
    }
}

fn get_value(
    reg: &String,
    values: &mut BTreeMap<String, bool>,
    ops: &BTreeMap<String, (String, String, Op)>,
) -> bool {
    if values.contains_key(reg) {
        return values.get(reg).unwrap().clone();
    }

    let (lhs_reg, rhs_reg, op) = ops.get(reg).unwrap();

    let lhs = get_value(lhs_reg, values, ops);
    let rhs = get_value(rhs_reg, values, ops);

    let val = op.apply(lhs, rhs);
    values.insert(reg.clone(), val);

    val
}

fn parse_data(
    data: String,
) -> (
    BTreeMap<String, bool>,
    BTreeMap<String, (String, String, Op)>,
    BTreeSet<String>,
    BTreeMap<String, usize>,
) {
    let parts: Vec<&str> = data.split("\n\n").collect();

    let mut values: BTreeMap<String, bool> = BTreeMap::new();
    let mut ops: BTreeMap<String, (String, String, Op)> = BTreeMap::new();
    let mut zregisters: BTreeSet<String> = BTreeSet::new();
    let mut occs: BTreeMap<String, usize> = BTreeMap::new();

    let val_re = Regex::new(r"([xy][0-9]{2}): ([01])").unwrap();
    let ops_re =
        Regex::new(r"([a-z0-9]{3}) ((?:XOR|OR|AND)) ([a-z0-9]{3}) -> ([a-z0-9]{3})").unwrap();

    for [reg, val] in val_re.captures_iter(parts[0]).map(|x| x.extract::<2>().1) {
        let val_parsed: bool = val == "1";
        values.insert(reg.to_string(), val_parsed);
    }

    for [reg1, op_str, reg2, target_reg] in
        ops_re.captures_iter(parts[1]).map(|x| x.extract::<4>().1)
    {
        let op = Op::from_str(op_str);
        if reg1 < reg2 {
            ops.insert(
                target_reg.to_string(),
                (reg1.to_string(), reg2.to_string(), op),
            );
        } else {
            ops.insert(
                target_reg.to_string(),
                (reg2.to_string(), reg1.to_string(), op),
            );
        }

        *occs.entry(reg1.to_string()).or_default() += 1;
        *occs.entry(reg2.to_string()).or_default() += 1;
        *occs.entry(target_reg.to_string()).or_default() += 1;

        if reg1.chars().next().unwrap() == 'z' {
            zregisters.insert(reg1.to_string());
        }
        if reg2.chars().next().unwrap() == 'z' {
            zregisters.insert(reg2.to_string());
        }
        if target_reg.chars().next().unwrap() == 'z' {
            zregisters.insert(target_reg.to_string());
        }
    }

    (values, ops, zregisters, occs)
}

#[derive(Debug, PartialEq, Eq)]
enum Op {
    And,
    Or,
    Xor,
}

impl Op {
    fn apply(&self, lhs: bool, rhs: bool) -> bool {
        match *self {
            Op::And => lhs & rhs,
            Op::Or => lhs | rhs,
            Op::Xor => lhs ^ rhs,
        }
    }

    fn from_str(raw: &str) -> Self {
        match raw {
            "AND" => Self::And,
            "OR" => Self::Or,
            "XOR" => Self::Xor,
            _ => panic!("Unexpected op str '{}'", raw),
        }
    }
}
