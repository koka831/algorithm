use std::io;

fn main() {
    let s = read_one::<String>().chars()
        .collect::<Vec<_>>();
    let first = s[0];
    let rle = run_length_encode(&s);

    if first == '<' {
        // <<< >>>> -> 0,1,2,4 4,3,2,1,0
        println!("<");
    } else {
        println!(">");
    }
}

#[allow(unused)]
fn factorial_sum(n: usize) -> usize {
    let mut ans: usize = 0;
    for i in 0..n {
        ans += n - i;
    }
    ans
}

#[allow(unused)]
fn run_length_encode<T>(ss: &[T]) -> Vec<usize> {
    vec![]
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
