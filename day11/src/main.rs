#[derive(PartialEq, Clone, Copy, Debug)]
enum Tile {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

type Layout = Vec<Vec<Tile>>;
type LayoutSlice = [Vec<Tile>];

fn main() {
    let input = parse(include_str!("../input.txt"));
    println!("Part 1 empty seats after stabilization: {}", result(&input, iteration_part1));

    println!("Part 2 empty seats after stabilization: {}", result(&input, iteration_part2));
}

fn parse(input: &str) -> Layout {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => Tile::Floor,
                    'L' => Tile::EmptySeat,
                    '#' => Tile::OccupiedSeat,
                    _ => panic!("Unknown tile {}", c),
                })
                .collect()
        })
        .collect()
}

fn result(l: &LayoutSlice, iteration: fn(&LayoutSlice) -> Layout) -> usize {
    let mut prev = l.to_owned();
    let mut current;
    loop {
        current = iteration(&prev);
        if cmp(&prev, &current) {
            break;
        }
        prev = current;
    }
    // count occupied
    current
        .iter()
        .flatten()
        .filter(|&&x| x == Tile::OccupiedSeat)
        .count()
}

fn iteration_part1(l: &LayoutSlice) -> Layout {
    iteration(l, count_adjacent_occupied, 4)
}

fn iteration_part2(l: &LayoutSlice) -> Layout {
    iteration(l, count_sightline_occupied, 5)
}

fn iteration(l: &LayoutSlice, count: fn(&LayoutSlice, usize, usize) -> u8, max: u8) -> Layout {
    l.iter()
        .enumerate()
        .map(|(i, r)| {
            r.iter()
                .enumerate()
                .map(|(j, &t)| {
                    let seen = count(l, i, j);
                    if t == Tile::OccupiedSeat && seen >= max {
                        return Tile::EmptySeat;
                    }
                    if t == Tile::EmptySeat && seen == 0 {
                        return Tile::OccupiedSeat;
                    }
                    t
                })
                .collect()
        })
        .collect()
}

fn count_adjacent_occupied(l: &LayoutSlice, i: usize, j: usize) -> u8 {
    let mut count = 0;
    let ilen = l.len();
    let jlen = l[0].len();
    let istart = std::cmp::max(0, i as isize - 1) as usize;
    let iend = std::cmp::min(i + 1, ilen - 1);
    let jstart = std::cmp::max(0, j as isize - 1) as usize;
    let jend = std::cmp::min(j + 1, jlen - 1);
    for (ii, row) in l.iter().enumerate().take(iend + 1).skip(istart) {
        for (jj, &tile) in row.iter().enumerate().take(jend + 1).skip(jstart) {
            if i == ii && j == jj {
                continue; //skip self
            }
            if tile == Tile::OccupiedSeat {
                count += 1
            }
        }
    }
    count
}

fn count_sightline_occupied(l: &LayoutSlice, i: usize, j: usize) -> u8 {
    let mut count = 0;
    let ilen = l.len() as isize;
    let jlen = l[0].len() as isize;
    for iinc in -1..=1 {
        for jinc in -1..=1 {
            let mut ipos = i as isize;
            let mut jpos = j as isize;
            if iinc == 0 && jinc == 0 {
                continue
            }
            loop {
                ipos += iinc;
                jpos += jinc;
                if ipos < 0 || ipos >= ilen || jpos < 0 || jpos >= jlen {
                    break;
                }
                match l[ipos as usize][jpos as usize] {
                    Tile::OccupiedSeat => {
                        count += 1;
                        break;
                    }
                    Tile::EmptySeat => break,
                    Tile::Floor => {}
                }
            }
        }
    }
    count
}

fn cmp(l1: &LayoutSlice, l2: &LayoutSlice) -> bool {
    l1.iter()
        .zip(l2)
        .map(|(row1, row2)| row1.iter().zip(row2).map(|(t1, t2)| t1 == t2).all(|t| t))
        .all(|r| r)
}

#[test]
fn test() {
    let input = parse(include_str!("../sample.txt"));
    assert!(cmp(&input, &input));
    let input1 = parse(include_str!("../sample2.txt"));
    let input2 = parse(include_str!("../sample3.txt"));
    let it0 = iteration_part1(&input);
    assert!(!cmp(&input, &it0));
    assert_eq!(it0[0][2], Tile::OccupiedSeat);
    assert_eq!(count_adjacent_occupied(&it0, 0, 2), 4);
    let it1 = iteration_part1(&it0);
    let it2 = iteration_part1(&iteration_part1(&iteration_part1(&it1)));
    assert!(cmp(&input1, &it1));
    assert!(cmp(&input2, &it2));
    assert_eq!(result(&input, iteration_part1), 37);

    //part 2
    assert_eq!(
        count_sightline_occupied(&parse(include_str!("../sample4.txt")), 4, 3),
        8
    );
    assert_eq!(
        count_sightline_occupied(&parse(include_str!("../sample5.txt")), 1, 1),
        0
    );
    assert_eq!(
        count_sightline_occupied(&parse(include_str!("../sample6.txt")), 3, 3),
        0
    );

    assert!(cmp(&parse(include_str!("../sample7.txt")), &iteration_part2(&iteration_part2(&input))));
    assert_eq!(result(&input, iteration_part2), 26);
}
