use std::io::stdin;
fn main() {
    println!("Chemical Equation Balancer in {{RUST}} (v1.0)");
    println!("For help, type .help");
    println!("Input Chemical Equation: ");
    let mut s = String::new();
    let _ = stdin().read_line(&mut s);
    s.pop();
    let v: Vec<&str> = s.split(" -> ").collect();
    let reactants = v[0];
    let products = v[1];
}
