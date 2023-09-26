fn print_slice(s: &str, a: usize, b: usize) {
    let s = &s[a..b];

    println!("slice: \"{}\"", s);
}

fn print_slice2(s: &str, a: usize, b: usize) {
    let s = &s[a..=b];

    println!("slice: \"{}\"", s);
}

fn main() {
    let s = String::from("abcdefgh");

    print_slice(&s, 0, 0);
    print_slice2(&s, 0, 0);
}
