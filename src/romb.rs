#[test]
fn main() {
    const SIZE: usize = 5; // Константа визначена всередині main

    let mut output = String::new();
    for i in 0..SIZE {
        let spaces = " ".repeat(SIZE - i - 1);
        let stars = "* ".repeat(i + 1);
        output.push_str(&format!("{}{}\n", spaces, stars));
    }
    for i in (0..SIZE - 1).rev() {
        let spaces = " ".repeat(SIZE - i - 1);
        let stars = "* ".repeat(i + 1);
        output.push_str(&format!("{}{}\n", spaces, stars));
    }
    println!("{}", output);
}
