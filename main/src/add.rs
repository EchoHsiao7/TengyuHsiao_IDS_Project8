use add::add;

fn main() {
    let a = 2;
    let b = 3;
    let result = add(&a, &b);
    println!("Adding {} and {} is {}", a, b, result);
}