use std::collections::HashMap;
use std::fmt::{self, Debug, Formatter};

#[allow(clippy::vec_init_then_push)]
fn vector_new_demo() {
    println!("`Vec::new()` and `Vec::push()` demonstration:");

    let mut v: Vec<i32> = Vec::new();

    v.push(2);
    v.push(3);
    v.push(5);
    v.push(7);

    println!("    {v:?}");
}

fn vector_vec_macro_demo() {
    println!("`vec!` macro demonstration:");

    let v = vec![2, 3, 5, 7];

    println!("    {v:?}");
}

#[allow(clippy::useless_vec)]
fn vector_index_operator_demo() {
    println!("Index operator on vectors demonstration:");

    let v = vec![2, 3, 5, 7];

    let x = v[2];

    println!("    `v[2]` is {x:?}");
}

fn get_and_print<T: std::fmt::Debug>(v: &Vec<T>, i: usize) {
    let x = v.get(i);

    match x {
        Some(x) => println!("    `v.get({i})` is {x:?}"),
        None => println!("    There is no {i}th element in `v`"),
    }
}

fn vector_get_demo() {
    println!("`Vec::get()` demonstration:");

    let v = vec![2, 3, 5, 7];

    get_and_print(&v, 2);
    get_and_print(&v, 5);
}

fn vector_iterating_over_elements_demo() {
    println!("Iterating over vector elements demonstration:");

    let mut v = vec![2, 3, 5, 7];

    for i in &mut v {
        *i *= 10;
    }

    println!("    {v:?}");
}

fn vector_enum_demo() {
    println!("Vector with enumeration elements demonstration:");

    #[derive(Debug)]
    enum Foo {
        A(i32),
        B(String),
    }

    let v = vec![Foo::A(42), Foo::B(String::from("abc"))];

    if let (Foo::A(x), Foo::B(y)) = (&v[0], &v[1]) {
        println!("    {x} :: {y}");
    }

    println!("    {v:?}");
}

fn vector_demo() {
    vector_new_demo();
    vector_vec_macro_demo();
    vector_index_operator_demo();
    vector_get_demo();
    vector_iterating_over_elements_demo();
    vector_enum_demo();
}

fn string_creation_demo() {
    println!("String creation demonstration:");

    let empty = String::new();
    let s1 = String::from("abc");
    let s2 = "def".to_string();

    println!(r#"    "{empty}" :: "{s1}" :: "{s2}""#);
}

fn string_push_demo() {
    println!("String push operations demonstration:");

    let s1 = "abc";
    let c = '-';
    let s2 = "def";

    let mut s = String::from(s1);
    s.push(c);
    s.push_str(s2);

    println!(r#"    "{s1}" + "{c}" + "{s2}" == "{s}""#);
}

fn string_add_demo() {
    println!("String add operator demonstration:");

    let s1 = String::from("abc");
    let s2 = String::from("def");
    let s = s1.clone() + &s2;

    println!(r#"    "{s1}" + "{s2}" == "{s}""#);
}

fn string_format_demo() {
    println!("`format!` macro demonstration:");

    let s1 = String::from("abc");
    let s2 = String::from("def");
    let s = format!("{s1}-{s2}");

    println!(r#"    format!("{{s1}}-{{s2}}") == "{s}", s1 == "{s1}", s2 == "{s2}""#);
}

fn string_iterating_demo() {
    println!("Iteration over a string demonstration:");

    let s = "Зд";

    println!(r#"    Characters in "{s}":"#);
    for c in s.chars() {
        println!(r#"        "{c}""#);
    }

    println!(r#"    Bytes in "{s}":"#);
    for b in s.bytes() {
        println!(r#"        "{b}""#);
    }
}

fn string_demo() {
    string_creation_demo();
    string_push_demo();
    string_add_demo();
    string_format_demo();
    string_iterating_demo();
}

fn hashmap_new_demo() {
    println!("Creating a hash map demonstration:");

    let mut data = HashMap::new();
    data.insert(String::from("hello"), "hello".len());
    data.insert(String::from("world"), "world".len());
    println!("    Hash map: {data:?}");
}

fn hashmap_access_demo() {
    println!("Accessing hash map items demonstration:");

    let mut data = HashMap::new();
    data.insert(String::from("foo"), 42);
    data.insert(String::from("bar"), 84);
    println!("    Hash map: {data:?}");

    let key = String::from("foo");
    let value = data.get(&key).copied().unwrap_or(0);
    println!(r#"    Hash map get("{key}"): {value}"#);

    let key = String::from("foobar");
    let value = data.get(&key).copied().unwrap_or(0);
    println!(r#"    Hash map get("{key}"): {value}"#);

    println!("    Hash map elements:");
    for (key, value) in &data {
        println!(r#"        "{key}": {value}"#);
    }
}

#[derive(Clone)]
struct Color(u8, u8, u8);

impl Debug for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let red = self.0;
        let green = self.1;
        let blue = self.2;
        write!(f, "#{red:02x}{green:02x}{blue:02x}")
    }
}

fn hashmap_update_demo() {
    println!("Updating a hash map demonstration:");

    let mut colors = HashMap::new();
    colors.insert(String::from("red"), Color(255, 0, 0));
    println!("    Hash map: {colors:?}");

    let blue = (String::from("blue"), Color(0, 0, 255));
    colors.insert(blue.0.clone(), blue.1.clone());
    println!("    After inserting {blue:?}: {colors:?}");

    let red = (String::from("red"), Color(0xf0, 0, 0));
    colors.insert(red.0.clone(), red.1.clone());
    println!("    After inserting {red:?}: {colors:?}");

    let yellow = (String::from("yellow"), Color(255, 255, 0));
    colors.entry(yellow.0.clone()).or_insert(yellow.1.clone());
    println!("    After inserting {yellow:?} if there is no one yet: {colors:?}");

    let blue = (String::from("blue"), Color(0, 0, 0xf0));
    colors.entry(blue.0.clone()).or_insert(blue.1.clone());
    println!("    After inserting {blue:?} if there is no one yet: {colors:?}");
}

fn hashmap_histogram_demo() {
    println!("Counting words demonstration:");

    let words = "apples and bananas and lemons";
    let mut map = HashMap::new();

    for word in words.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!(r#"    Histogram of "{words}": {map:?}"#);
}

fn hashmap_demo() {
    hashmap_new_demo();
    hashmap_access_demo();
    hashmap_update_demo();
    hashmap_histogram_demo();
}

fn main() {
    vector_demo();
    println!("");
    string_demo();
    println!("");
    hashmap_demo();
}
