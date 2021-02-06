use std::collections::HashMap;

type Differences = HashMap<u8, usize>;

fn main() {
    let input = parse(include_str!("../input.txt"));
    //part 1
    println!("Part 1: {}", result_part1(&count_differences(&input)));
    println!("Part 2: {}", result_part2(&input));
}

fn parse(input: &str) -> Vec<u32> {
    input.lines()
        .map(|l| l.trim().parse::<u32>().expect("Not an int"))
        .collect()
}

fn count_differences(input: &[u32]) -> Differences {
    let mut dif = Differences::new();
    let mut sorted = Vec::from(input);
    sorted.push(0);
    sorted.sort_unstable();
    sorted.iter().enumerate().skip(1).for_each(|(i, x)| *dif.entry((x - sorted[i-1]) as u8).or_insert(0) += 1);
    *dif.entry(3).or_insert(0) += 1;
    dif
}

fn result_part1(dif: &Differences) -> usize {
    dif.get(&1).unwrap() * dif.get(&3).unwrap()
}

// Simple memoized recursion
fn walk_chains_recur(input: &[u32], cache: &mut [Option<u64>]) -> u64 {
    if input.is_empty() {
        return 0;
    }
    if input.len() == 1 {
        return 1
    }
    let mut total = 0;
    for i in 1..=3 {
        if i >= input.len() {
            break
        }
        if input[i] - input[0] > 3 {
            break
        }
        total += walk_chains_memo(&input[i..input.len()], &mut cache[i..input.len()])
    }
    total
}

fn walk_chains_memo(input: &[u32], cache: &mut [Option<u64>]) -> u64 {
    if let Some(x) = cache[0]  {
        return x
    }
    let res = walk_chains_recur(input, cache);
    cache[0] = Some(res);
    res
}

fn result_part2(input: &[u32]) -> u64 {
    let mut sorted = Vec::from(input);
    sorted.push(0);
    sorted.sort_unstable();
    sorted.push(sorted.last().unwrap() + 3);
    let mut cache = vec!(None; sorted.len());
    walk_chains_recur(&sorted, &mut cache)
}

#[test]
fn test() {
    let input = parse(include_str!("../sample.txt"));
    let input2 = parse(include_str!("../sample2.txt"));
    let dif1 = count_differences(&input);
    let dif2 = count_differences(&input2);
    assert_eq!(dif1.get(&1), Some(&7));
    assert_eq!(dif1.get(&3), Some(&5));
    assert_eq!(dif2.get(&1), Some(&22));
    assert_eq!(dif2.get(&3), Some(&10));

    assert_eq!(result_part2(&input), 8);
    assert_eq!(result_part2(&input2), 19208);
}
