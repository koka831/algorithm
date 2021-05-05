use std::io;


fn main() {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let num = buf.trim().parse::<i32>().unwrap();

    println!("{}", 48 - num);
}
