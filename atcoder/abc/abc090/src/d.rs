use std::io;
use std::cmp;

/*
 *
b を固定して考えましょう。b = 1, 2, · · · N に対し、a を b で割った余りが K 以上であるような 1 ≤ a ≤ N
の個数が高速に求められれば良いです。簡単のため、a = 0 も許すことにして、あとで a = 0 の場合 (これは
簡単に求められます) を引くことにしましょう。
さて、整数 p, q を用いて N = pb + r(0 ≤ r < b) という形で N を一意的に表したとき、a を 0 から N ま
で順に動かせば、a を b で割った余りを順に並べたものは、0, 1, 2, · · · b − 1 という列が p 回繰り返され、最
後に 0, 1, 2, · · · r という列が付け加わったものになります。
0, 1, 2, · · · b − 1 という列が p 回繰り返される部分には条件を満たす a の個数は p × max(0, b − K) 個、最
後の部分には max(0, r − K + 1) 個あるので、条件を満たす a の個数が O(1) 時間で求められ、O(N) 時間
でこの問題を解くことができました。
*/

#[allow(dead_code)]
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

#[allow(dead_code)]
fn read_one<T>() -> T
where T:
    std::str::FromStr,
    T::Err: std::fmt::Debug {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().unwrap()
}

fn main() {

    let (n, k) = {
        let a = read::<isize>();
        (a[0], a[1])
    };

    let mut cnt = 0;
    for b in 1..(n + 1) {
        let p = n / b;
        let q = n % b;
        cnt += p * cmp::max(0, b - k) + cmp::max(0, q - k + 1);
        if k == 0 { cnt -= 1; }
    }

    println!("{}", cnt);
}
