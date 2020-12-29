use std::io;

#[derive(Debug)]
enum Square {
    Empty,
    Tree,
}

fn read() -> Vec<Vec<Square>> {
    let mut inp = String::new();
    let mut r: Vec<Vec<Square>> = Vec::new();
    while io::stdin().read_line(&mut inp).expect("read error") != 0 {
        let mut i: Vec<Square> = Vec::with_capacity(inp.len());
        for c in inp.trim().chars() {
            i.push(match c {
                '.' => Square::Empty,
                '#' => Square::Tree,
                _ => panic!("Unexpected char {}", c),
            });
        }
        r.push(i);
        inp.clear();
    }
    r
}

fn main() {
    let r = read();
    // part 1
    println!("{}", slope(&r, 1, 3));
    // part 2
    let mut res = 1;
    for (down, right) in vec![(1, 1), (1, 3),(1, 5), (1, 7), (2, 1)].iter() {
        res = res * slope(&r, *down, *right);
    }
    println!("{}", res);
}

fn slope(r: &Vec<Vec<Square>>, down: usize, right: usize) -> usize {
    let mut pos = 0;
    let mut count = 0;
    for l in r.iter().step_by(down) {
        match l[pos] {
            Square::Tree => {
                count = count + 1;
            }
            Square::Empty => {}
        };
        pos = (pos + right) % l.len();
    }
    count
}
