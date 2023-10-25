fn main() {
    scope();
}

fn scope() {
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // this will print `hello, world!`
}
