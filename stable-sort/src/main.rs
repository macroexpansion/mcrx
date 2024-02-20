use stable_sort::stable_sorted;

#[stable_sorted]
struct A {
    a: u32,
    b: u32,
    c: u32,
}

#[stable_sorted]
enum B {
    A,
    B,
    C,
}

fn main() {
    println!("main");
}
