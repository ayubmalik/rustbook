use std::io;

fn main() {
    println!("Hello, world!");

    let mut x = 7;
    x = x + 1;
    println!("x = {}", x);

    let mut buf = String::new();

    println!("gizza text");
    match io::stdin().read_line(&mut buf) {
        Ok(_) => println!("yay"),
        Err(_) => print!("nay"),
    };
    println!("READED = {}", buf.trim());

    let tup = (1, "hello", "world", 99);
    println!("tup = {:?}", tup);
    let (a, b, _, _) = tup;
    println!("The value of a,b  is: {}, {}", a, b);

    let days = ["monday", "tuesday", "wednesday"];
    println!("{:?}", days);

    let mut idx = String::new();
    println!("index");
    io::stdin().read_line(&mut idx).expect("boom!");
    let n = idx.trim().parse::<usize>().expect("piss");
    println!("elem {} = {}", n, days[n]);
}
