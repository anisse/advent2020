use std::io;

#[derive(Debug, Clone)]
struct PasswordRule {
    min: u32,
    max: u32,
    letter: char,
}

#[derive(Debug, Clone)]
struct Line {
    r: PasswordRule,
    p: String,
}

fn read() -> Vec<Line> {
    let mut inp = String::new();
    let mut r: Vec<Line> = Vec::new();
    while io::stdin().read_line(&mut inp).expect("read error") != 0 {
        let mut i = inp.split(|c| c == '-' || c == ' ' || c == ':');
        r.push(Line {
            r: PasswordRule {
                min: i
                    .next()
                    .expect("missing min token")
                    .clone()
                    .parse()
                    .expect("not int"),
                max: i
                    .next()
                    .expect("missing max token")
                    .clone()
                    .parse()
                    .expect("not int"),
                letter: i
                    .next()
                    .expect("missing letter token")
                    .clone()
                    .trim()
                    .chars()
                    .nth(0)
                    .expect("no char"),
            },
            p: i.last()
                .expect("missing pw token")
                .clone()
                .trim()
                .to_string(),
        });
        inp.clear();
    }
    r
}
fn main() {
    let r = read();
    //part 1
    let mut correct = 0;
    for i in r.iter() {
        let count = i.p.chars().filter(|c| *c == i.r.letter).count() as u32;
        if count >= i.r.min && count <= i.r.max {
            correct = correct + 1;
        }
    }
    //part 2
    let mut correct = 0;
    for i in r.iter() {
        if (i
            .p
            .chars()
            .nth(i.r.min as usize - 1)
            .expect(format!("no element {} in {:?}", i.r.min + 1, i).as_str())
            == i.r.letter)
            ^ (i.p
                .chars()
                .nth(i.r.max as usize - 1)
                .expect(format!("no element {}", i.r.min + 1).as_str())
                == i.r.letter)
        {
            correct = correct + 1;
        }
    }
    println!("{} / {}", correct, r.len())
}
