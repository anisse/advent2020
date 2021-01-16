fn main() {
    let input = parse(include_str!("../input.txt"));
    //part 1
    let invalid = first_invalid(&input, 25).unwrap();
    println!("First element of list that does not match rules: {}", invalid);
    //part 2
    let subset = subset_sum(&input, invalid);
    println!("Sum of subset min + max: {}", subset.iter().min().unwrap() + subset.iter().max().unwrap());
}

fn parse(input: &str) -> Vec<u64> {
    input.lines()
        .map(|l| l.trim().parse::<u64>().expect("Not an int"))
        .collect()
}

fn first_invalid(input: &[u64], start: usize) -> Option<u64> {
    'outer: for (i, &v) in input.iter().enumerate().skip(start) {
        for j in (i - start)..i {
            for k in (j+1)..i {
                if input[j] + input[k] == v {
                    continue 'outer
                }
            }
        }
        return Some(v)
    }
    None
}

fn subset_sum(input: &[u64], total: u64) -> &[u64] {
    for i in 0..input.len() {
        let mut acc = 0;
        for j in i..input.len() {
            acc += input[j];
            if acc == total {
                return &input[i..=j]
            }
        }
    }
    &[]
}

#[test]
fn test() {
    let input = parse(include_str!("../sample.txt"));
    assert!(!input.is_empty());
    let invalid = first_invalid(&input, 5);
    assert_eq!(invalid, Some(127));
    let invalid = invalid.unwrap();
    let subset = subset_sum(&input, invalid);
    assert_eq!(subset.first(), Some(&15));
    dbg!(&subset);
    assert_eq!(subset.iter().sum::<u64>(), invalid);
    dbg!(&subset);
    assert_eq!(subset.iter().min().unwrap() + subset.iter().max().unwrap(), 62);
}
