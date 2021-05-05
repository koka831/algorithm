use std::io;


fn main() {
    let (n, m) = {
        let i = read::<usize>();
        (i[0], i[1])
    };
    let mut pm = Vec::new();
    let mut ym = Vec::new();
    let mut order = vec![Vec::new(); n];
    for _ in 0..m {
        let i = read::<usize>();
        pm.push(i[0]);
        ym.push(i[1]);
        order[i[0] - 1].push(i[1]);
    }

    for i in 0..n {
        order[i].sort();
    }

    for i in 0..m {
        print!("{:06}", pm[i]);
        let idx = order[pm[i] - 1].binary_search(&ym[i]).unwrap();
        println!("{:06}", idx + 1);
    }
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
