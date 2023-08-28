# The Rust Programming Language Notes

* Rust is a compiled statically-typed language
* [Home Page](https://www.rust-lang.org/)
  * [Book](https://doc.rust-lang.org/book/)
  * [Reference Guide](https://doc.rust-lang.org/reference/index.html)
* [Source Code](https://github.com/rust-lang/rust)
* [The Rust community’s crate registry](https://crates.io/)

## Installation

To install the Rust programming language and the related tools, type:
```sh
$ dnf install rust rust-doc cargo rustfmt
```

## Tools

### Cargo

[Cargo](https://doc.rust-lang.org/cargo/index.html) ([source code](https://github.com/rust-lang/cargo))
is a package manager for Rust. A package in the Rust world is called *crate*.
Using `cargo` is a recommended way how to create and maintain Rust projects.

* To create a new project in Rust, type:
  ```sh
  $ cargo new project_slug
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

### Keywords

See [Appendix A: Keywords](https://doc.rust-lang.org/book/appendix-01-keywords.html)
from the [book](https://doc.rust-lang.org/book/).

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
  for more info

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
  for more info

#### Integer Literals

In Rust, these forms of integer literals are possible:
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
* for more info about how integer literals are interpreted, see
  [Integer literals](https://doc.rust-lang.org/reference/tokens.html#integer-literals)
  and [Integer literal expressions](https://doc.rust-lang.org/reference/expressions/literal-expr.html#integer-literal-expressions)

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
* for more info about floating point literals, see
  [Floating-point literals](https://doc.rust-lang.org/reference/tokens.html#floating-point-literals)
  and [Floating-point literal expressions](https://doc.rust-lang.org/reference/expressions/literal-expr.html#floating-point-literal-expressions)

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

#### Character Type

* `char`
* 4 bytes in size

#### Integer Types

| **Signed** | **Unsigned** | **Size** |
| ---------- | ------------ | -------- |
| `i8` | `u8` | 8 bits |
| `i16` | `u16` | 16 bits |
| `i32` | `u32` | 32 bits |
| `i64` | `u64` | 64 bits |
| `i128` | `u128` | 128 bits |
| `isize` | `usize` | machine specific |

#### Floating Point Types

* `f32` and `f64` with 32 bits and 64 bits in size, respectively
* IEEE-754
* default is `f64`

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
* for explanation of `constant_expression`, see
  [Constant evaluation](https://doc.rust-lang.org/reference/const_eval.html)

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

## Expressions

* [Expressions reference](https://doc.rust-lang.org/reference/expressions.html)
* [Operators and symbols list](https://doc.rust-lang.org/book/appendix-02-operators.html) (see also [here](https://doc.rust-lang.org/reference/tokens.html#punctuation))
* [Operator precedence](https://doc.rust-lang.org/reference/expressions.html#expression-precedence)

### `return` Expressions

Grammar:
```
return_expression:
    "return" expression?
```

See [`return` expressions](https://doc.rust-lang.org/reference/expressions/return-expr.html)
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
| `||` | logical or | left-to-right |
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
| `|` | bitwise/logical or | left-to-right |
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

## Library

* [`rand` - random number generators](https://crates.io/crates/rand)
* [`std` - the Rust standard library](https://doc.rust-lang.org/std/index.html)
  * [`std::io` - the I/O module](https://doc.rust-lang.org/std/io/index.html)
    * [`std::io::Stdin` - a handle to the standard input stream of a process](https://doc.rust-lang.org/std/io/struct.Stdin.html)
  * [`std::prelude` - the list of symbols which is preloaded](https://doc.rust-lang.org/std/prelude/index.html)
  * [`std::result` - error handling with the `Result` type](https://doc.rust-lang.org/std/result/index.html)
    * [`std::result::Result`](https://doc.rust-lang.org/std/result/enum.Result.html)
  * [`std::string` - a growable UTF-8 string module](https://doc.rust-lang.org/std/string/index.html)
    * [`std::string::String` - a growable UTF-8 string](https://doc.rust-lang.org/std/string/struct.String.html)
