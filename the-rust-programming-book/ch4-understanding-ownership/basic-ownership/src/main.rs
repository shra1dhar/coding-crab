// Each value in Rust has a variable that’s called its owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.

fn main() {
    let s: String = String::from("What up?");
    takes_ownership(s);
    println!("{}", s);

    let x: i32 = 23;
    makes_copy(x);
    println!("{}", x)
}

fn makes_copy(num: i32) {
    println!("{}", num)
}

fn takes_ownership(s: String) {
    println!("{}", s)
}