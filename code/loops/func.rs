fn main() {
    hello();
    println!("ar eita main function er ongsho");
    add(20, 20);
}

fn hello() {
    println!("Eita hoilo hello function");
}

fn add(x: i8, y: i8) {
    println!("Addition result hoilo {}", x + y);
}
