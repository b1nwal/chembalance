use std::io::stdin;
fn main() {
    println!("Chemical Equation Balancer in {{RUST}} (v1.0)");
    println!("For help, type .help");
    println!("Input Chemical Equation: ");
    let x: i8 = 2;
    let mut s = String::new();
    let _ = stdin().read_line(&mut s);
    s.pop();
}
