fn main() {
    message("hello there sir");
    println!("{}", check(12));
    println!("{}", check(21));
    println!("{}", check(32));
    loopit();
    whiley();
    forit();
}

/*
big message
*/
fn message(msg: &str) {
    println!("{}", msg)
}

fn check(age: i32) -> bool {
    if age > 18 {
        true
    } else {
        false
    }
}

fn loopit() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn whiley() {
    let mut number = 10;
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
    println!("LIFTOFF!!!");
}

fn forit() {
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
