use std::io;

fn read() -> Vec<u32> {
    let mut inp = String::new();
    let mut r : Vec<u32> = Vec::new();
    while io::stdin()
        .read_line(&mut inp)
        .expect("read error") != 0 {
        let i: u32 = inp.trim().parse().expect("not int");
        r.push(i);
        inp.clear();
    }
    r
}

fn main() {
    let r = read();
    for (i, x) in r.iter().enumerate() {
        for (j, y) in r[i+1..].iter().enumerate() {
            for z in r[j+1..].iter() {
                if x + y + z == 2020 {
                    println!("{}", x*y*z);
                }
            }
        }
    }
}
