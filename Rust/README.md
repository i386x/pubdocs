# The Rust Programming Language Notes

* Rust is a compiled statically-typed language
* [Home Page](https://www.rust-lang.org/)
  * [Book](https://doc.rust-lang.org/book/)
  * [Reference Guide](https://doc.rust-lang.org/reference/index.html)
  * [RFC Book](https://rust-lang.github.io/rfcs/)
  * [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
  * [Rust by Example](https://doc.rust-lang.org/rust-by-example/index.html)
  * [The Little Book of Rust Macros](https://veykril.github.io/tlborm/) [[mirror](https://danielkeep.github.io/tlborm/book/)]
* [Learning Rust](https://github.com/danbev/learning-rust)
* [Procedural Macros Workshop](https://github.com/dtolnay/proc-macro-workshop)
* [Source Code](https://github.com/rust-lang/rust)
* [The Rust community’s crate registry](https://crates.io/)

## Installation

To install the Rust programming language and the related tools, type:
```sh
$ dnf install rust rust-doc cargo rustfmt
```

On non-Fedora based systems:
```sh
$ : "${RUST_PROFILE:=defualt}"  # Other possibilities: minimal, complete
$ curl curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh -s -- -y --profile ${RUST_PROFILE}
```

## Tools

### Cargo

[Cargo](https://doc.rust-lang.org/cargo/index.html) ([source code](https://github.com/rust-lang/cargo))
is a package manager for Rust. A package in the Rust world is called *crate*.
Using `cargo` is a recommended way how to create and maintain Rust projects.

* To create a new project in Rust, type:
  ```sh
  $ cargo new --name project_name project_slug
  ```
  This creates the `project_slug` directory with `Cargo.toml` project
  configuration file and `src/` directory containing project sources. The
  content of `Cargo.toml` has the following structure:
  ```toml
  [package]
  # The name of the package, e.g. "foo":
  name = "<name>"
  # The version of the package, e.g. "0.1.0":
  version = "<version>"
  # The Rust language edition used to compile this project, e.g. "2021":
  edition = "<edition>"

  [dependencies]
  # A dependency is of the format `<crate name> = "<version>"`, where <version>
  # follows the semantic versioning scheme. An example of a dependency is
  # `rand = "0.8.5"`, where "0.8.5" is a shorthand for "^0.8.5" which means any
  # version that is at least "0.8.5" but below "0.9.0". Cargo considers that
  # any of these versions are compatible with "0.8.5".
  ```
  When `--name` is omitted, the name of the project is derived from the name of
  the project directory (`project_slug`).
* To build the project, type:
  ```sh
  $ cargo build
  ```
  This will build your project and
  1. puts the resultant executable inside `./target/debug` directory;
  1. creates `Cargo.lock` keeping the track of exact versions of project
     dependencies. When you run `cargo build` first time, Cargo resolves the
     project dependencies and writes the exact versions of crates to
     `Cargo.lock`. Next time Cargo reuse the information from `Cargo.lock`
     instead of resolving dependencies again. If a dependency needs to be
     updated, run `cargo update`.
* To build a release of your project, type:
  ```sh
  $ cargo build --release
  ```
  This will build your project with enabled optimizations and put the resultant
  executable inside `./target/release` directory.
* To build and run the project, type:
  ```sh
  $ cargo run
  ```
* To check whether the project compiles, type:
  ```sh
  $ cargo check
  ```
  This will produce no executable.
* To update the project dependencies, type:
  ```sh
  $ cargo update
  ```
  This will look for new bug fixes of crates, download them and update
  `Cargo.lock`.
* To launch the documentation of your project dependencies, type:
  ```sh
  cargo doc --open
  ```

### Rust Code Format Checker

`rustfmt` formats a Rust code using the Rust code style conventions.

* To check the code style of one file, type:
  ```sh
  $ rustfmt --check file.rs
  ```

### Rust Compiler

[`rustc`](https://doc.rust-lang.org/rustc/index.html) compiles a rust project
into its binary representation.

* To compile a Rust project, type:
  ```sh
  $ rustc main.rs
  ```
  All you need is just to pass your project's root file to `rustc` (here
  `main.rs`) and `rustc` will automatically gather all the necessary source
  files, compiles them and links them together.

## Lexical Elements

* Rust input is viewed as a sequence of UTF-8 characters

### Comments

```rust
// This is a single line comment.
```

See [Comments](https://doc.rust-lang.org/reference/comments.html) for greater
detail.

### Keywords

See [Appendix A: Keywords](https://doc.rust-lang.org/book/appendix-01-keywords.html)
from the [book](https://doc.rust-lang.org/book/) or [Keywords](https://doc.rust-lang.org/reference/keywords.html)
for greater detail.

### Literals

* any literal may end with suffix which is an identifier or keyword
* a suffix can annotate a literal with type or it can serve as syntactical
  sugar in token stream processed during macro expansion

#### Character Literals

Grammar:
```
character_literal: "'" (CHAR | ESCAPE) "'"

CHAR: any Unicode Scalar Value (U+0000 to U+D7FF and U+E000 to U+10FFFF
      inclusive) except single quote (U+0027), backslash (U+005C), new line
      (U+000A), carriage return (U+000D), and tab character (U+0009)
ESCAPE:
    "\'" | "\""
    "\x" ODIGIT XDIGIT
    "\n" | "\r" | "\t" | "\\" | "\0"
    "\u{" (XDIGIT "_"*){1,6} "}"
```

* the type of character literal is `chr`
* see [Character literals](https://doc.rust-lang.org/reference/tokens.html#character-literals)
  for greater detail

#### String Literals

Grammar:
```
string_literal:
    '"' (CHAR | ESCAPE)* '"'
    "r" RAW_STRING
byte_string_literal:
    'b"' (BCHAR | BESCAPE)* '"'
    "br" RAW_BYTE_STRING

RAW_STRING:
    '"' RAW_CHAR* '"'
    "#" RAW_STRING "#"
RAW_BYTE_STRING:
    '"' ASCII* '"'
    "#" RAW_BYTE_STRING "#"
CHAR: any Unicode Scalar Value (U+0000 to U+D7FF and U+E000 to U+10FFFF
      inclusive) except double quote (U+0022), backslash (U+005C), and sole
      carriage return (U+000D); U+000D U+000A is translated to U+000A
RAW_CHAR: any Unicode Scalar Value (U+0000 to U+D7FF and U+E000 to U+10FFFF
          inclusive) except sole carriage return (U+000D)
BCHAR: any ASCII (U+0000 to U+007F) except double quote (U+0022), backslash
       (U+005C), and sole carriage return (U+000D)
ASCII: any ASCII (U+0000 to U+007F)
ESCAPE:
    "\'" | "\""
    "\x" ODIGIT XDIGIT
    "\n" | "\r" | "\t" | "\\" | "\0"
    "\u{" (XDIGIT "_"*){1,6} "}"
    "\" (U+000A | U+000D U+000A) (U+0020 | U+000A | U+000D | U+0009)*
BESCAPE:
    "\x" XDIGIT XDIGIT | "\n" | "\r" | "\t" | "\\" | "\0" | "\'" | "\""
    "\" (U+000A | U+000D U+000A) (U+0020 | U+000A | U+000D | U+0009)*
```

* the last type of escape sequence (`"\" (U+000A ...) ...`) is removed from the
  string literal (all its occurrences)
* `RAW_CHAR` and `ASCII` are interpreted as is, escape sequences have no
  meaning here
* the type of string literal is `&'static str`
* the type of byte string literal of the length `n` is `&'static [u8; n]`
* see [String literals](https://doc.rust-lang.org/reference/tokens.html#string-literals),
  [Raw string literals](https://doc.rust-lang.org/reference/tokens.html#raw-string-literals),
  [Byte string literals](https://doc.rust-lang.org/reference/tokens.html#byte-string-literals),
  and [Raw byte string literals](https://doc.rust-lang.org/reference/tokens.html#raw-byte-string-literals)
  for greater detail

#### Integer Literals

Grammar:
```
integer_literal:
    decimal_integer type_suffix?
    hexadecimal_integer type_suffix?
    octal_integer type_suffix?
    binary_integer type_suffix?
    byte_integer "u8"?

decimal_integer: DIGIT (DIGIT | "_")*
hexadecimal_integer: "0x" (XDIGIT | "_")* XDIGIT (XDIGIT | "_")*
octal_integer: "0o" (ODIGIT | "_")* ODIGIT (ODIGIT | "_")*
binary_integer: "0b" (BDIGIT | "_")* BDIGIT (BDIGIT | "_")*
byte_integer: "b'" (BYTE_CHAR | BYTE_ESC) "'"

type_suffix: "u8" | "i8" | "u16" | "i16" | "u32" | "i32" | "u64" | "i64" |
             "u128" | "i128" | "usize" | "isize"

BDIGIT: "0" | "1"
ODIGIT: BDIGIT | "2".."7"
DIGIT: ODIGIT | "8" | "9"
XDIGIT: DIGIT | "a".."f" | "A".."F"

BYTE_CHAR: any ASCII (U+0000 to U+007F) except single quote (U+0027), backslash
           (U+005C), new line (U+000A), carriage return (U+000D), or tab
           character (U+0009)
BYTE_ESC: "\x" XDIGIT XDIGIT | "\n" | "\r" | "\t" | "\\" | "\0" | "\'" | "\""
```

* "_" works as a digit separator and is ignored (increases number readability)
* if there is no type suffix, "i32" is used
* see [Integer literals](https://doc.rust-lang.org/reference/tokens.html#integer-literals)
  and [Integer literal expressions](https://doc.rust-lang.org/reference/expressions/literal-expr.html#integer-literal-expressions)
  for greater detail

#### Floating Point Literals

Grammar:
```
floating_point_literal:
    decimal_integer "."
    decimal_integer "." decimal_integer type_suffix?
    decimal_integer ("." decimal_integer)? exponent type_suffix?

exponent: ("e" | "E") ("+" | "-")? (DIGIT | "_")* DIGIT (DIGIT | "_")*

type_suffix: "f32" | "f64"
```

* if there is no type suffix, "f64" is used
* see [Floating-point literals](https://doc.rust-lang.org/reference/tokens.html#floating-point-literals)
  and [Floating-point literal expressions](https://doc.rust-lang.org/reference/expressions/literal-expr.html#floating-point-literal-expressions)
  for greater detail

## Data Types

* Rust can infer a variable's data type
  * in case of more than one possible data types, a variable must be annotated
    with a data type

### Never Type

* `!`
* has no values
* represent the result of computations that never complete
* see [Never type](https://doc.rust-lang.org/reference/types/never.html) for
  greater detail

### Scalar Types

#### Boolean Type

* `bool`
* one byte in size
* two possible values: `true` and `false`
* see [Boolean type](https://doc.rust-lang.org/reference/types/boolean.html)
  for greater detail

#### Character Type

* `char`
* 4 bytes in size
* see [Textual types](https://doc.rust-lang.org/reference/types/textual.html)
  for greater detail

#### Integer Types

| **Signed** | **Unsigned** | **Size** |
| ---------- | ------------ | -------- |
| `i8` | `u8` | 8 bits |
| `i16` | `u16` | 16 bits |
| `i32` | `u32` | 32 bits |
| `i64` | `u64` | 64 bits |
| `i128` | `u128` | 128 bits |
| `isize` | `usize` | machine specific |

See [Integer types](https://doc.rust-lang.org/reference/types/numeric.html#integer-types)
and [Machine-dependent integer types](https://doc.rust-lang.org/reference/types/numeric.html#machine-dependent-integer-types)
for greater detail.

#### Floating Point Types

* `f32` and `f64` with 32 bits and 64 bits in size, respectively
* IEEE-754
* default is `f64`
* see [Floating-point types](https://doc.rust-lang.org/reference/types/numeric.html#floating-point-types)
  for greater detail

### Compound Types

#### Tuple Type

* tuples are finite sequences of values, where two values may have distinct
  types
* at the syntactical level, a tuple is a comma-separated list of elements
  (items) enclosed between parentheses:
  ```
  "(" ((item ",")+ item?)? ")"
  ```
* an item can be either type or expression or identifier
* an example of assigning a tuple to the variable:
  ```rust
  let tup = ("foo", 1, 0.5);
  ```
* with type annotations:
  ```rust
  let tup: (str, i32, f64) = ("foo", 1, 0.5);
  ```
* elements can be extracted from a tuple using either dot expression or
  destructuring assignment
* dot expression has a form `tuple "." index`, where `index` is a non-negative
  integer literal in decimal notation not exceeding the tuple size minus one
  (tuple indices are zero-based); example:
  ```rust
  let point = (3, 5);

  let x = point.0;
  let y = point.1;
  ```
* destructuring assignment:
  ```rust
  let point = (3, 5);

  let (x, y) = point;
  ```

##### Unit

* *empty tuple*
* both unit type and unit value are written as `()`
* represent an empty value or an empty return type
* empty value is returned implicitly by an expression

See [Tuple types](https://doc.rust-lang.org/reference/types/tuple.html) and
[Tuple and tuple indexing expressions](https://doc.rust-lang.org/reference/expressions/tuple-expr.html)
for greater detail.

#### Array Type

* arrays are finite sequences of values of same type
* unlike in other programming languages arrays have fixed length
* in Rust, array are allocated on stack
* at the syntactical level, an array is a comma-separated list of elements
  (items) enclosed between brackets:
  ```
  "[" (item ("," item)*)? "]"
  ```
* an example of an array assigned to the variable:
  ```rust
  let arr = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
  ```
* an array annotation consists of a type and the number of elements in the
  array; the exact notation is `"[" type ";" size "]"`, example:
  ```rust
  let arr: [u8; 4] = [1, 2, 4, 8];
  ```
* arrays can be initialized by `"[" value ";" count "]"` expression:
  ```rust
  let arr = [2; 4];  // same as `let arr = [2, 2, 2, 2];`
  ```
* to access the element of an array, use `array "[" index "]"` expression:
  ```rust
  let arr = ["a", "b"];

  let a = arr[0];  // a == "a"
  let b = arr[1];  // b == "b"
  ```
* like tuples, array indices are zero-based
* indexing an array out of its bounds make a program panicking
* see [Array types](https://doc.rust-lang.org/reference/types/array.html) and
  [Array and array index expressions](https://doc.rust-lang.org/reference/expressions/array-expr.html)
  for greater detail

#### Reference Types

* a general signature for a reference types is
  ```
  "&" ("'" identifier | "'" "static" | "'" "_")? "mut"? type_no_bounds
  ```
* if `mut` is not present, a refernce is called a *shared* reference
  * it points to a memory location owned by other value
  * prevents direct mutation of the value (exception to this rule is interior
    mutability)
  * there can be any number of shared references to a value
  * a reference type implements `Copy`
  * referencing a temporal value keeps it alive during the lifetime of the
    reference itself
* if `mut` is present, a reference is called a *mutable* reference
  * like shared reference, it also points to a memory location owned by other
    value
  * allows direct mutation of the value
    * the value must not be borrowed yet
  * only one mutable reference per value can exists
* see [References (& and &mut)](https://doc.rust-lang.org/reference/types/pointer.html#references--and-mut)
  for greater detail

## Declarations

### Variables

* variables are defined using the `let` keyword
* by default, variables are defined as immutable
  * to define a mutable variable use the `mut` keyword
* variables are scoped
* the variable definition statement:
  ```
  "let" "mut"? variable_identifier (":" type)? "=" expression ";"
  ```
* examples:
  ```rust
  let x = 5;      // immutable variable
  let mut y = 7;  // mutable variable
  ```
* variables can be shadowed:
  ```rust
  let x = "foo";    // x is immutable and str
  let x = x.len();  // x is shadowed - still immutable but integer
  ```
* see [Identifiers](https://doc.rust-lang.org/reference/identifiers.html),
  [`let` statements](https://doc.rust-lang.org/reference/statements.html#let-statements)
  and [Variables](https://doc.rust-lang.org/reference/variables.html) for
  greater detail

### Constants

* always immutable
* scoped
* the constant definition statement:
  ```
  "const" constant_identifier ":" type "=" constant_expression ";"
  ```
* example:
  ```rust
  const THREE: u32 = 1 + 2;
  ```
* convention: use upper case and underscores for constant names
* see [Constant items](https://doc.rust-lang.org/reference/items/constant-items.html)
  and [Constant evaluation](https://doc.rust-lang.org/reference/const_eval.html)
  (explains `constant_expression`) for greater detail

### Functions

Grammar:
```
function:
    qualifiers "fn" identifier generic_params? "(" parameters ")"
        return_type? where_clause?
        (block_expression | ";")

qualifiers:
    "const"? "async"? "unsafe"? ("extern" ABI?)?
ABI: string_literal

generic_params:
    "<" ">"
    "<" (generic_param ",")* generic_param ","? ">"
generic_param:
    lifetime_param
    type_param
    const_param
lifetime_param:
    "'" identifier (":" lifetime_bounds)?
lifetime_bounds:
    (lifetime "+")* lifetime?
lifetime:
    "'" identifier
    "'static"
    "'_"
type_param:
    identifier (":" type_param_bounds?)? ("=" type)?
type_param_bounds:
    type_param_bound ("+" type_param_bound)* "+"?
type_param_bound:
    lifetime
    trait_bound
trait_bound:
    _trait_bound
    "(" _trait_bound ")"
_trait_bound:
    "?"? ("for" generic_params)? type_path
const_param:
    "const" identifier ":" type
        ("=" block_expression | identifier | "-"? literal_expression)?

parameters:
    self_param ","?
    (self_param ",")? param ("," param)* ","?
self_param:
    ("&" lifetime?)? "mut"? "self"
    "mut"? "self" ":" type
param:
    pattern ":" (type | "...")
    "..."
    type

return_type:
    "->" type

where_clause:
    "where" (where_clause_item ",")* where_clause_item?
where_clause_item:
    lifetime ":" lifetime_bounds
    ("for" generic_params)? type ":" type_param_bounds?
```

Simple function definition and simple call example:
```rust
fn main() {
    simple_fun();
}

fn simple_fun() {
    println!("Hello!");
}
```

Function with parameters:
```rust
fn fun_with_params(x: i32, y: i32) {
    println!("x: {x}, y: {y}");
}

fn main() {
    fun_with_params(5, 3);
}
```

Function returning value:
```rust
fn max(a: i32, b: i32) -> i32 {
    if (a > b) {
        return a;
    }
    b
}

fn main() {
    let x = max(1, 2);

    println!("max(1, 2): {x}");
}
```

See [Functions](https://doc.rust-lang.org/reference/items/functions.html) for
greater detail.

## Ownership

* ownership is a strategy of keeping of a track of used memory
  * whether the memory is used or not is decided on compile time
* ownership rules
  1. every value in Rust has an owner
  1. there can be only one owner at a time
  1. when the owner goes out of scope, the value will be dropped (Rust calls
     [`drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html#tymethod.drop)
     on it)
* see [Pointer types](https://doc.rust-lang.org/reference/types/pointer.html)
  and [Slice types](https://doc.rust-lang.org/reference/types/slice.html) for
  greater detail

### Moves, Clones, and Copies

When a value is moved from one variable to another, the value of the former
variable is considered invalid and that variable cannot be used:
```rust
let a = String::from("Hey!");
let b = a;  // `a` cannot be used from now since its value is invalid
```

This can be bypassed using the `clone` method:
```rust
let a = String::from("Hello!");
let b = a.clone();  // both `a` and `b` stay valid
```

However, if a type implements the `Copy` trait, a value is not moved but copied
and it stays valid:
```rust
let x = 5;
ley y = x;  // `x` is valid since `i32` implements the `Copy` trait
```

**Note:** If a type or any of its part implements the `Drop` trait it cannot be
annotated with (it cannot implement) the `Copy` trait.

Types that can implement the `Copy` trait are in general:
* any group of simple scalar values
* nothing that requires allocation or is some form of resource

This includes:
* all integer types
* `bool` type
* all floating-point types
* `char` type
* tuples containing only types implementing `Copy`

Moving and copying concepts hold also for functions and other assignment-like
operations:
* `function(x)` moves/copies `x` to its parameter
* `return x` moves/copies `x` outside of function as its return value

### References and Borrowing

* a reference is holding an address to some kind of data/value
* a reference lifetime starts with its definition and ends after its last use
  in the current scope
* a reference is guaranteed to point to a valid data/value during the reference
  lifetime
* a reference points to a value but it not owns it
  * a referenced value cannot be changed and thus there can be many references
    for the same memory location at the same time
* a reference is created using `&` unary operator
  * e.g., `&a` is a reference to the value owned by `a`
* `&` can be also used to declare a reference type
  * e.g., `&i32` is a reference type of `i32` type
* creating a reference is called *borrowing*
* using `mut` together with `&` creates/declares a *mutable* reference
  * `&mut a` is a mutable reference to the value of `a`
  * `&mut i32` is a mutable reference type of `i32` type
* a mutable reference allows to change the value it is referring to
  * no other references to that value are allowed to exist during the mutable
    reference lifetime

### Dangling Pointers

Rust will not allow dangling pointers:
```rust
fn dangle() {
    let s = String::from("Hey!");

    &s  // `s` goes out of scope so it is dropped; `&s` points to freed memory
}
```

### Slices

* a slice is a reference to contiguous sequence of elements in a collection
* a slice is made using index (`&x[r]`) expression, where `&x` is a reference
  to some collection type and `r` has a range type; the value of `&x[r]` is the
  reference to the portion of `x`; some examples:
  ```rust
  let s = String::from("abcdefgh");

  let x = &s[1..4];  // `x` points to `s[1]` ("bcd") and has type `&String`
  let y = &s[..3];   // same as `&s[0..3]`
  let z = &s[2..];   // same as `&s[2..(s.len())]`
  let w = &s[..];    // same as `&s[0..(s.len())]`

  // let t = &s[-1..0];  // error: `a` is not of a type `usize`
  // let t = &s[1..0];   // panic: `a` is not less or equal to `b`
  let t = &s[1..=0];     // ok: `t == ""`, `a <= b` (`1 <= 1`)
  // let t = &s[2..=0];  // panic: `a` is not less or equal to `b` (`2 > 1`)

  let a = [1, 2, 3, 4, 5];

  let x = &a[1..3];  // `assert_eq!(x, &[2, 3]);`, `x` has type `&[i32]`
  ```
* **Note 1:** A range `a..b` used to make a slice must have `a <= b`, where
  both `a` and `b` are of the `usize` type.
* **Note 2:** String (`str`) slice range indices must occur at valid UTF-8
  character boundaries. Otherwise the program panics.
* **Note 3:** String literals have a type `&str` since they are slices of a
  binary data stored in the data section of a program.
* **Note 4:** `String` implements the `Deref` trait that converts `&String` to
  `&str` by calling a `deref` method (the code is generated by Rust during
  compile time). As a consequence, if a function `fun` has a signature
  `fn fun(s: &str)` and `x` has a type `&String` we can `fun(x)` without any
  worries.

## Expressions

* [Expressions reference](https://doc.rust-lang.org/reference/expressions.html)
* [Operators and symbols list](https://doc.rust-lang.org/book/appendix-02-operators.html) (see also [here](https://doc.rust-lang.org/reference/tokens.html#punctuation))
* [Operator precedence](https://doc.rust-lang.org/reference/expressions.html#expression-precedence)

### Loop Expressions

Grammar:
```
loop_expression:
    loop_label? (
        "loop" block_expression
        "while" conditional_expression block_expression
        "while" "let" pattern "=" scrutinee block_expression
        "for" pattern "in" expression block_expression
        block_expression
    )
loop_label: "'" identifier ":"

conditional_expression: expression
scrutinee: expression

break_expression:
    "break" ("'" identifier)? expression?
continue_expression:
    "continue" ("'" identifier)?
```

`loop { body }`:
* execute `body` infinitely
* if `body` does not contain `break`, the type of the expression is `!`;
  otherwise, the type of the expression is the type of the `break` expression
* the type of the expression must be compatible with the type of every `break`
  expression inside `body`
* the value of the expression is the value returned by a `break` expression
  from `body`

`while conditional { body }`:
* if `condition` is true execute `body` and go to the next iteration
* `condition` must not be `struct_expression`
* the type and the value of the expression, `body` and the `break` expression
  follow the same rules as in the `loop` case

`while let pattern = scrutinee { body }`:
* if the value of `scrutinee` matches `pattern` execute `body` and go to the
  next iteration
* `scrutinee` must not be `lazy_boolean_expression`
* the type and the value of the expression, `body` and the `break` expression
  follow the same rules as in the `loop` case

`for pattern in expression { body }`:
* `expression` must not be `struct_expression`
* the value of `expression` must implement
  [`std::iter::IntoIterator`](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html)
* `pattern` must be irrefutable
* if the iterator yield a value, the value is matched against `pattern` and the
  `body` is executed after which the control returns to the next iteration
* a `'label: for pattern in expression { body }` is equivalent to
  ```rust
  {
      let result = match IntoIterator::into_iter(expression) {
          // Don't drop temporaries from `expression` before loop is finished
          mut iter => 'label: loop {
              let mut next;
              match Iterator::next(&mut iter) {
                  Option::Some(val) => next = val,
                  Option::None => break,
              };
              let pattern = next;
              let () = { body };
          },
      };
      result
  }
  ```

Loop labels:
* a loop expression can be optionally labeled
* labels can be shadowed
  ```rust
  'a: loop {         // (1)
      'a: loop {     // (2)
          break 'a;  // exit (2) loop
      }
      break 'a;      // exit (1) loop
  }
  ```

`break` expressions:
* allowed only inside of the body of a loop or a labeled block expression
* `break` immediately exits from the innermost loop or a labeled block
  expression
  * if a label is specified, `break` immediately exits from a loop labeled with
    this label
  * in a labeled block expression the label part is mandatory
* if the expression part is present, `break` returns the value of the
  expression to the associated loop or labeled block expression
  * just `break` returns `()`

`continue` expressions:
* allowed only inside of the body of a loop
* associated with the innermost loop expression
  * if the label part is present, the `continue` expression is associated with
    the loop expression labeled with this label
* `continue` immediately stops the current iteration and returns the control
  back to the associated loop so the next iteration can be started

See [Loops and other breakable expressions](https://doc.rust-lang.org/reference/expressions/loop-expr.html#loops-and-other-breakable-expressions)
for greater detail.

### `if` Expressions

Grammar:
```
if_expression:
    "if" conditional_expression block_expression
        ("else" (block_expression | if_expression | iflet_expression))?

iflet_expression:
    "if" "let" pattern "=" scrutinee block_expression
        ("else" (block_expression | if_expression | iflet_expression))?

conditional_expression: expression
scrutinee: expression
```

* the `conditional_expression` must not be `struct_expression` and must be of
  a type `bool`
* the `scrutinee` must not be `lazy_boolean_expression`
* all block expressions must have the same type
* the value of the `if` expression is the value of its *then* branch if the
  conditional expression evaluates to `true`; otherwise, the value of the `if`
  expression is the value of its *else* branch
* if neither *then* nor *else* branch are evaluated the value of the `if`
  expression is `()`
* in an `if let` expression, if the value of `scrutinee` matches `pattern` the
  whole expression has the value of its *then* branch; otherwise, it has the
  value of its *else* branch or `()` if the *else* branch is missing
  * `if let` expression
    ```rust
    if let PATTERN = EXPRESSION {
        then_branch
    }
    else {
        else_branch
    }
    ```
    is equivalent to
    ```rust
    match EXPRESSION {
        PATTERN => { then_branch },
        _ => { else_branch },
    }
    ```

See [`if` and `if let` expressions](https://doc.rust-lang.org/reference/expressions/if-expr.html)
for greater detail.

### `return` Expressions

Grammar:
```
return_expression:
    "return" expression?
```

See [`return` expressions](https://doc.rust-lang.org/reference/expressions/return-expr.html)
for greater detail.

### Range Expressions

Grammar:
```
range_expression:
    expression ".." expression
    expression ".."
    ".." expression
    ".."
    expression "..=" expression
    "..=" expression
```

* `a..b` constructs [`std::ops::Range`](https://doc.rust-lang.org/std/ops/struct.Range.html)
  object; `(a..b).contains(&x)` is equivalent to `a <= x && x < b`
* `a..` constructs [`std::ops::RangeFrom`](https://doc.rust-lang.org/std/ops/struct.RangeFrom.html)
  object; `(a..).contains(&x)` is equivalent to `a <= x`
* `..b` constructs [`std::ops::RangeTo`](https://doc.rust-lang.org/std/ops/struct.RangeTo.html)
  object; `(..b).contains(&x)` is equivalent to `x < b`
* `..` constructs [`std::ops::RangeFull`](https://doc.rust-lang.org/std/ops/struct.RangeFull.html)
  object; this range has no boundaries
* `a..=b` constructs [`std::ops::RangeInclusive`](https://doc.rust-lang.org/std/ops/struct.RangeInclusive.html)
  object; `(a..=b).contains(&x)` is equivalent to `a <= x && x <= b`
* `..=b` constructs [`std::ops::RangeToInclusive`](https://doc.rust-lang.org/std/ops/struct.RangeToInclusive.html)
  object; `(..=b).contains(&x)` is equivalent to `x <= b`

See [Range expressions](https://doc.rust-lang.org/reference/expressions/range-expr.html)
for greater detail.

### Lazy Boolean Expressions

Grammar:
```
lazy_boolean_expression:
    expression OP expression

OP: "&&" | "||"
```

The meaning and associativity of each operator (operators lower in the table
have higher precedence):
| **Operator** | **Meaning** | **Associativity** |
| ------------ | ----------- | ----------------- |
| `\|\|` | logical or | left-to-right |
| `&&` | logical and | left-to-right |

`||` and `&&` differs from `|` and `&` in a way that the right-hand side
expression is evaluated when its value is needed.

See [Lazy boolean operators](https://doc.rust-lang.org/reference/expressions/operator-expr.html#lazy-boolean-operators)
for greater detail.

### Arithmetic and Logical Expressions

Grammar:
```
arithmetic_or_logical_expression:
    expression OP expression

OP: "+" | "-" | "*" | "/" | "%" | "&" | "|" | "^" | "<<" | ">>"
```

The meaning and associativity of each operator (operators lower in the table
have higher precedence):
| **Operator** | **Meaning** | **Associativity** |
| ------------ | ----------- | ----------------- |
| `\|` | bitwise/logical or | left-to-right |
| `^` | bitwise/logical xor | left-to-right |
| `&` | bitwise/logical and | left-to-right |
| `<<`, `>>` | left shift, right shift | left-to-right |
| `+`, `-` | addition, subtraction | left-to-right |
| `*`, `/`, `%` | multiplication, division, remainder | left-to-right |

See [Arithmetic and Logical Binary Operators](https://doc.rust-lang.org/reference/expressions/operator-expr.html#arithmetic-and-logical-binary-operators)
for greater detail.

#### Handling Integer Overflow

* arithmetic operations with integers may overflow
* in debug mode, overflow causes panic
* in `--release` mode, overflow causes unexpected results (caused by modulo
  arithmetic a.k.a. complement wrapping)
* Rust standard library provides methods for primitive types dealing with
  integer arithmetic:
  ```rust
  // wrapping_* methods do modular arithmetic
  assert_eq!(200u8.wrapping_add(100), 44);

  // checked_* methods return None in case of overflow
  assert_eq!(200u8.checked_add(100), None);

  // overflowing_* methods return the value and boolean indicating overflow
  assert_eq!(200u8.overflowing_add(100), (44, true));

  // saturating_* methods do a saturation
  assert_eq!(200u8.saturating_add(100), u8::MAX);
  ```

### Borrow Expressions

Grammar:
```
borrow_expression:
    ( "&" | "&&" ) "mut"? expression
```

* `&a` produces a reference if `a` is an expression with an associated memory
  location
* memory location associated with `a` is switched to a borrowed state for the
  entire duration of `&a`
  * for `&a` this means that `a` cannot be mutated, but it can be read or
    shared again
  * for `&mut a` this means that `a` cannot be accessed in any other way until
    `&mut a` expires (i.e. having two mutable references to the same place is
    considered invalid)
* if `a` is a value expression (i.e. it has no associated memory location, like
  `3 + 5`), then `&a` or `&mut a` yields a creation of a temporary memory
  location which is then referenced

See [Borrow operators](https://doc.rust-lang.org/reference/expressions/operator-expr.html#borrow-operators)
for greater detail.

### Array Index Expressions

Grammar:
```
array_index_expression:
    expression "[" expression "]"
```

See [Array and slice indexing expressions](https://doc.rust-lang.org/reference/expressions/array-expr.html#array-and-slice-indexing-expressions)
for greater detail.

### Call Expressions

Grammar:
```
call_expression:
    expression "(" arguments? ")"

arguments:
    expression ("," expression)* ","?
```

See [Call expressions](https://doc.rust-lang.org/reference/expressions/call-expr.html)
for greater detail.

### Field Access Expressions

Grammar:
```
field_access_expression:
    expression "." identifier
    expression "." decimal_integer
```

See [Field access expressions](https://doc.rust-lang.org/reference/expressions/field-expr.html)
and [Tuple indexing expressions](https://doc.rust-lang.org/reference/expressions/tuple-expr.html#tuple-indexing-expressions)
for greater detail.

### Block Expressions

Grammar:
```
block_expression:
    "{" statement* expression? "}"
```

The value and type of `block_expression` is the value and type of `expression`
if it is present. Otherwise the value and type of `block_expression` is `()`.

See [Block expressions](https://doc.rust-lang.org/reference/expressions/block-expr.html)
for greater detail.

### Atomic Expressions

Grammar:
```
atomic_expression:
    literal_expression
    tuple
    array

literal_expression:
    character_literal
    string_literal
    byte_string_literal
    integer_literal
    floating_point_literal
    "true" | "false"

tuple: "(" tuple_elements? ")"
tuple_elements: (expression ",")+ expression?

array: "[" array_elements? "]"
array_elements:
    expression ("," expression)* ","?
    expression ";" expression
```

See [Literal expressions](https://doc.rust-lang.org/reference/expressions/literal-expr.html),
[Tuple expressions](https://doc.rust-lang.org/reference/expressions/tuple-expr.html#tuple-expressions),
and [Array expressions](https://doc.rust-lang.org/reference/expressions/array-expr.html#array-expressions)
for greater detail.

## Statements

Following list of rules summarizes what is considered a statement:
* a sole `;` is a statement
* a function definition is a statement
* `let` statement
* an expression followed by `;` is a statement

See [Statements](https://doc.rust-lang.org/reference/statements.html) for
greater detail.

### `let` Statement

Grammar:
```
let_statement:
    "let" pattern (":" type)? ("=" expression ("else" block_expression)?)? ";"
```

* introduces a new set of variables given by a `pattern`
* `pattern` can be annotated with `type`
* variables in `pattern` can be initialized by `expression`
* if `else` is not present, `pattern` must be irrefutable
* if `else` is present
  * `pattern` can be refutable
  * `expression` must not be a lazy boolean expression or end with a `}`
  * `block_expression` must evaluate to never type
* the semantics of `else` part is that if `pattern` fails to match then the
  `block_expression` is executed
* see [`let` statements](https://doc.rust-lang.org/reference/statements.html#let-statements)
  for greater detail

## Macros

## Modules, Crates, and Name Spaces

* a *crate* is viewed as a translation unit
* function `main` is the program's entry point

### Paths

#### Type Paths

Grammar:
```
type_path:
    "::"? type_path_segment ("::" type_path_segment)*
type_path_segment:
    path_ident_segment ("::"? (generic_args | type_path_fn))?
path_ident_segment:
    identifier | "super" | "self" | "Self" | "crate" | "$crate"
generic_args:
    "<" ((generic_arg ",")* generic_arg ","?)? ">"
generic_arg:
    "'" identifier | "'static" | "'_"
    type
    block_expression
    "-"? literal_expression
    identifier | "super" | "self" | "crate" | "$crate"
    identifier "=" type
type_path_fn:
    "(" (type ("," type)* ","?)? ")" ("->" type)?
```

## Libraries (Crates)

* [`clap`](https://crates.io/crates/clap) [[doc](https://docs.rs/clap/latest/clap/)] [[repo](https://github.com/clap-rs/clap)]
* [`proc-macro2`](https://crates.io/crates/proc-macro2) [[doc]](https://docs.rs/proc-macro2/latest/proc_macro2/) [[repo](https://github.com/dtolnay/proc-macro2)]
* [`quote`](https://crates.io/crates/quote) [[doc](https://docs.rs/quote/latest/quote/)] [[repo](https://github.com/dtolnay/quote)]
* [`rand` - random number generators](https://crates.io/crates/rand) [[doc](https://docs.rs/rand/latest/rand/)] [[repo](https://github.com/rust-random/rand)]
* [`std` - the Rust standard library](https://doc.rust-lang.org/std/index.html)
  * [`std::io` - the I/O module](https://doc.rust-lang.org/std/io/index.html)
    * [`std::io::Stdin` - a handle to the standard input stream of a process](https://doc.rust-lang.org/std/io/struct.Stdin.html)
  * [`std::prelude` - the list of symbols which is preloaded](https://doc.rust-lang.org/std/prelude/index.html)
  * [`std::result` - error handling with the `Result` type](https://doc.rust-lang.org/std/result/index.html)
    * [`std::result::Result`](https://doc.rust-lang.org/std/result/enum.Result.html)
  * [`std::string` - a growable UTF-8 string module](https://doc.rust-lang.org/std/string/index.html)
    * [`std::string::String` - a growable UTF-8 string](https://doc.rust-lang.org/std/string/struct.String.html)
* [`syn`](https://crates.io/crates/syn) [[doc](https://docs.rs/syn/latest/syn/)] [[repo](https://github.com/dtolnay/syn)]
