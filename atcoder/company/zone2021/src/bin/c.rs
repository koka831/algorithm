use std::io;

type Spec = [usize; 5];

fn main() {
    let n = read_one::<usize>();
    let mut spec = Vec::new();

    for _ in 0..n {
        let (a, b, c, d, e) = {
            let i = read::<usize>();
            (i[0], i[1], i[2], i[3], i[4])
        };

        spec.push([a, b, c, d, e]);
    }

    let mut ok = 0;
    let mut ng = 1_000_000_001;
    while ng - ok > 1 {
        let m = (ok + ng) / 2;
        if satisfy(&spec, m) {
            ok = m;
        } else {
            ng = m;
        }
    }

    println!("{}", ok);
}

fn satisfy(spec: &[Spec], threshold: usize) -> bool {
    let mut cnt = vec![0; 1 << 5];
    for s in spec {
        let m = mask(&s, threshold);
        cnt[m] += 1;
    }

    for i in 0..(1 << 5) { for j in 0..(1 << 5) { for k in 0..(1 << 5) {
        if cnt[i] * cnt[j] * cnt[k] == 0 { continue; }
        if i | j | k == (1 << 5) - 1 { return true; }
    }}}

    false
}

fn mask(s: &Spec, threshold: usize) -> usize {
    let mut bin = 0;
    for i in 0..5 {
        if s[i] >= threshold { bin += 1 << i; }
    }
    bin
}

#[allow(unused)]
fn read<T>() -> Vec<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.split_whitespace()
        .map(|s| s.trim().parse().unwrap())
        .collect()
}


#[allow(unused)]
fn read_one<T>() -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().unwrap()
}
