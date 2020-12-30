use std::collections::HashSet;

type Question = u8;
type PersonYes = HashSet<Question>;
#[derive(Debug)]
struct Group(Vec<PersonYes>);

fn main() {
    //part 1
    println!("There a total of {} yes answers", parse(include_str!("../input.txt"))
            .iter()
            .map(|x| x.uniques().len())
            .sum::<usize>());
    //part 2
    println!("There are a total of {} common yes answers", parse(include_str!("../input.txt"))
            .iter()
            .map(|x| x.common().len())
            .sum::<usize>()
    );
}

fn parse(input: &str) -> Vec<Group> {
    let mut res = Vec::new();
    for g in input.split("\n\n") {
        let mut group = Group::new();
        for p in g.lines() {
            let mut person = PersonYes::new();
            for c in p.bytes() {
                person.insert(c);
            }
            group.push(person);
        }
        res.push(group);
    }
    res
}

impl Group {
    fn uniques(&self) -> Vec<Question> {
        let mut h = HashSet::new();
        self.0.iter()
            .flatten()
            .map(|x| h.insert(*x))
            .for_each(drop);
        //let h = &h;
        h.into_iter().collect()
    }
    fn common(&self) -> Vec<Question> {
        let h = self.0.iter()
            .fold(self.0[0].clone(), |acc, x| &acc & x);
        h.into_iter().collect()
    }
    fn new() -> Group {
        Group(Vec::new())
    }
    fn push(&mut self, arg: PersonYes) {
        self.0.push(arg)
    }
}

#[test]
fn test_parse() {
    assert_eq!(
        parse(include_str!("../sample.txt"))
            .iter()
            .map(|x| x.uniques().len())
            .sum::<usize>(),
        11
    );
    assert_eq!(
        parse(include_str!("../sample.txt"))
            .iter()
            .map(|x| x.common().len())
            .sum::<usize>(),
        6
    )
}
