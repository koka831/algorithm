use std::io;

fn read<T>() -> Vec<T>
where T:
    std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split(' ')
        .map(|s| s.trim().parse().unwrap())
        .collect()
}

fn read_one<T>() -> T
where T:
    std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().unwrap()
}

fn main() {
    let n = read_one::<usize>();
    let mut a: Vec<(usize, usize)> = Vec::new();
    let mut b: Vec<(usize, usize)> = Vec::new();
    for _ in 0..n {
        let i = read::<usize>();
        a.push((i[0], i[1]));
    }

    for _ in 0..n {
        let i = read::<usize>();
        b.push((i[0], i[1]));
    }

    a.sort();
    a.reverse();
    b.sort();

    let mut cnt = 0;
    while b.len() > 0 {

        let bi = b.pop().unwrap();
        let mut off = 0;
        let mut max_y = 0;
        let mut max_i = 100000;

        while off < a.len() {
            let ai = a[off];
            if bi.0 > ai.0 && bi.1 > ai.1 { 
                if max_y <= ai.1 {
                    max_y = ai.1;
                    max_i = off;
                }
            }
            off += 1;
        }

        if max_i < 10000 {
            cnt += 1;
            a.remove(max_i);
        }
    }

    println!("{}", cnt);
}
