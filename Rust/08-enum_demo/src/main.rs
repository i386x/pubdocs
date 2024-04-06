#[derive(Clone, Debug)]
enum Object {
    Null,
    Scalar(u8),
    Tuple(u8, u8),
}

use crate::Object::{Null, Scalar, Tuple};

fn test_match(obj: &Object) {
    match *obj {
        Null => {
            println!("Null");
        }
        Scalar(z @ 1) | Scalar(z @ 2) => {
            println!("Scalar({z})");
        }
        Scalar(a) | Tuple(_, a) if a >= 3 => {
            println!("a = {a}");
        }
        Scalar(x) => {
            println!("x = {x}");
        }
        Tuple(a, b) => {
            println!("Tuple({a}, {b})");
        }
    }
}

fn decide(x: u8, y: &Option<u8>, z: u8) -> u8 {
    if x > 127 {
        x
    } else if let Some(x) = *y {
        x
    } else {
        z
    }
}

fn main() {
    let a = Some(8);
    let b = None;
    let objs = vec![
        Null,
        Scalar(0),
        Scalar(1),
        Scalar(2),
        Scalar(3),
        Scalar(4),
        Tuple(0, 0),
        Tuple(1, 3),
        Tuple(3, 1),
    ];

    for x in objs {
        test_match(&x);
    }
    println!("{}", decide(200, &a, 42));
    println!("{}", decide(20, &a, 42));
    println!("{}", decide(20, &b, 42));
}
