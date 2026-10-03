fn main() {
    let year: u16 = 1978;
    let result: u16 = (year / 100) % 4;

    println!("On its own: \t\t {}", year);
    println!("After dividing: \t {}", 2026 / 100);
    println!("After modulo: \t\t {}", result);
}
