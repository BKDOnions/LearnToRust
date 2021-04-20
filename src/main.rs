mod learning;
fn main() {
    let mut a = 123;
    let mut b = 321;
    println!("a = {0}, b = {1}", a, b);
    a = 345;
    b = 543;
    println!("a = {0}, b = {1}", a, b);
    let c = 567;
    println!("{}", c);
    let c = c + 10;
    println!("{}", c);
    println!("Hello, world!");
    learning::data_types::show_data_types();
}
