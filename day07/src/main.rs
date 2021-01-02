use std::collections::HashMap;

//#[derive(Debug,Hash,PartialEq)]
type Bag = String;
type Container = HashMap<Bag, u32>;

type Rules = HashMap<Bag, Container>;

fn main() {
    let rules = parse(include_str!("../input.txt"));
    let count = count_has(&rules, "shiny gold");
    //part 1
    println!("bags that can have a shiny gold: {}", count);
    //part 2
    println!(
        "bags that can fit in a shiny gold: {}",
        count_contains(&rules, "shiny gold")
    );
}

fn parse(input: &str) -> Rules {
    let mut r = Rules::new();
    for l in input.lines() {
        let mut words = l.split_ascii_whitespace(); // our tokenizer
                                                    // first the name
        let mut b = Bag::new();
        b.push_str(words.next().expect("first bag word"));
        b.push(' ');
        b.push_str(words.next().expect("color word"));
        assert_eq!(words.next(), Some("bags"), "should bags");
        assert_eq!(words.next(), Some("contain"), "should contain");
        let mut c = Container::new();
        let mut num = 0;
        loop {
            let t = words.next().expect("Expected another token");
            match t {
                "no" => {
                    assert_eq!(words.next(), Some("other"));
                }
                "bags." | "bag." => break,
                "bags," | "bag," => continue,
                _ if (t.as_bytes()[0] as u8).is_ascii_digit() => {
                    num = t.parse().expect("number");
                }
                _ => {
                    let mut b = Bag::from(t);
                    b.push(' ');
                    b.push_str(words.next().expect("color word"));
                    c.insert(b, num);
                    num = 0;
                }
            };
        }

        r.insert(b, c);
    }
    r
}

fn has(rules: &Rules, c: &Container, bag: &str) -> bool {
    if c.contains_key(bag) {
        return true;
    }
    for b in c.keys() {
        if has(rules, &rules[b], bag) {
            return true;
        }
    }
    false
}

fn count(rules: &Rules, c: &Container) -> u32 {
    let mut n = 0;
    for (bag, has) in c.iter() {
        n += has;
        let x = count(rules, &rules[bag]);
        n += has * x;
    }
    n
}

fn count_has(rules: &Rules, bag: &str) -> u32 {
    let mut count = 0;
    for c in rules.values() {
        if has(&rules, c, bag) {
            count += 1
        }
    }
    count
}

fn count_contains(rules: &Rules, bag: &str) -> u32 {
    count(rules, &rules[bag])
}

#[test]
fn test_parse() {
    let rules = parse(include_str!("../sample.txt"));
    assert!(!rules.is_empty());
    assert_eq!(count_has(&rules, "shiny gold"), 4);
    assert_eq!(count_contains(&rules, "shiny gold"), 32);
    let rules2 = parse(include_str!("../sample2.txt"));
    assert_eq!(count_contains(&rules2, "shiny gold"), 126);
}
