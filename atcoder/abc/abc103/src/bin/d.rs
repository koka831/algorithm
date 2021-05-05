use std::io;

fn main() {
    let (n, m) = {
        let i = read::<usize>();
        (i[0], i[1])
    };

    let mut ab = Vec::new();
    for _ in 0..m {
        let (a, b) = {
            let i = read::<usize>();
            (i[0], i[1])
        };
        ab.push((a, b));
    }

    ab.sort_by_key(|x| x.1);

    let mut ans = 0;
    let mut bnd = 0;

    for (a, b) in ab {
        if a >= bnd {
            ans += 1;
            bnd = ::std::cmp::max(bnd, b);
        }
    }

    println!("{}", ans);
}

#[allow(dead_code)]
fn read<T>() -> Vec<T>
where T:
    std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace()
        .map(|s| s.trim().parse().unwrap())
        .collect()
}

#[allow(dead_code)]
fn read_one<T>() -> T
where T:
    std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().unwrap()
}
