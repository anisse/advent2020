use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Field {
    Ecl,
    Pid,
    Eyr,
    Hcl,
    Byr,
    Iyr,
    Cid,
    Hgt,
}
use crate::Field::*;

type Passport = HashMap<Field, String>;

fn main() {
    let input = parse(include_str!("../input.txt"));
    let valid: Vec<_> = input
        .iter()
        .filter(|passport| {
            passport.len() == 8 || (passport.len() == 7 && passport.get(&Cid) == None)
        })
        .collect();
    //part1
    dbg!(valid.len());
    let valid: Vec<_> = valid.iter().filter(|p| check(p)).collect();
    dbg!(valid.len());
}

fn parse(input: &str) -> Vec<Passport> {
    let mut v = Vec::new();
    for i in input.split("\n\n") {
        let mut p = HashMap::new();
        for f in i.split_ascii_whitespace() {
            let x: Vec<&str> = f.split(':').collect();
            p.insert(
                match x[0] {
                    "ecl" => Ecl,
                    "pid" => Pid,
                    "eyr" => Eyr,
                    "hcl" => Hcl,
                    "byr" => Byr,
                    "iyr" => Iyr,
                    "cid" => Cid,
                    "hgt" => Hgt,
                    _ => panic!("Unknown field {}", x[0]),
                },
                x[1].to_string(),
            );
        }
        v.push(p)
    }
    v
}

fn check(p: &Passport) -> bool {
    if check_num(p, Byr, 1920, 2002).is_err()
        || check_num(p, Iyr, 2010, 2020).is_err()
        || check_num(p, Eyr, 2020, 2030).is_err()
    {
        return false;
    }
    let hgt = p.get(&Hgt).expect("Height");
    if hgt.len() < 3 {
        return false;
    }
    let (h, min, max) = match &hgt[hgt.len() - 2..] {
        "cm" => (hgt[..hgt.len() - 2].parse::<u32>(), 150, 193),
        "in" => (hgt[..hgt.len() - 2].parse::<u32>(), 59, 76),
        _ => return false,
    };
    match h {
        Err(_) => return false,
        Ok(h) => {
            if h < min || h > max {
                return false;
            }
        }
    }
    for (f, re, name) in [
        (Hcl, r"^#[a-f0-9]{6}$", "hair color"),
        (Ecl, r"^(amb|blu|brn|gry|grn|hzl|oth)$", "eye color"),
        (Pid, r"^[0-9]{9}$", "passport id"),
    ]
    .iter()
    {
        let s = p.get(&f).expect(name);
        let reg = Regex::new(re).unwrap();
        if !reg.is_match(s) {
            return false;
        }
    }
    true
}

fn check_num(p: &Passport, f: Field, min: u32, max: u32) -> Result<(), ()> {
    match p.get(&f).expect("field not found").parse::<u32>() {
        Ok(y) => {
            if y < min || y > max {
                return Err(());
            }
        }
        Err(_) => return Err(()),
    }
    Ok(())
}
