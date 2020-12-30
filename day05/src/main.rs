type Seat = u16;

fn main() {
    let input = parse(include_str!("../input.txt"));
    //part 1
    let max = input.iter().max().unwrap();
    dbg!(max);
    // part 2
    let mut sorted = input;
    sorted.sort_unstable();
    for (i, v) in sorted.iter().enumerate() {
        if i == 0 || i == sorted.len() - 1 {
            continue;
        }
        if v - sorted[i - 1] != 1 {
            println!(
                "Found hole at seat ID {}, between {} and {}",
                v - 1,
                sorted[i - 1],
                v
            );
        }
    }
}

fn parse(input: &str) -> Vec<Seat> {
    let mut v = Vec::new();
    for i in input.lines() {
        let mut x: Seat = 0;
        for c in i.chars() {
            match c {
                'B' | 'R' => {
                    x <<= 1;
                    x |= 1;
                }
                'F' | 'L' => {
                    x <<= 1;
                }
                _ => panic!(format!("Unknown char {}", c)),
            }
        }
        v.push(x);
    }
    v
}

#[test]
fn test_parse() {
    assert_eq!(parse("BFFFBBFRRR")[0], 567);
    assert_eq!(parse("FFFBBBFRRR")[0], 119);
    assert_eq!(parse("BBFFBBFRLL")[0], 820);
}
