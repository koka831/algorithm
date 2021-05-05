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

    let (a, b, k) = {
        let n = read::<usize>();
        (n[0], n[1], n[2])
    };

    let mut nums = Vec::new();
    for i in a..(b + 1) {
        if ((i < a + k) || (i > b - k)) {
            nums.push(i);
        }
    }
    nums.sort();
    for i in nums {
        println!("{}", i);
    }
}
