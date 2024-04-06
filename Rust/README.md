# The Rust Programming Language Notes

* Rust is a compiled statically-typed language
* [Being Rusty: Discovering Rust's design axioms](https://smallcultfollowing.com/babysteps/blog/2023/12/07/rust-design-axioms/)
* [Dump C++ and in Rust you should trust, Five Eyes agencies urge](https://www.theregister.com/2023/12/07/memory_correction_five_eyes/)
* [Exercism](https://exercism.org/tracks/rust)
* [Home Page](https://www.rust-lang.org/)
  * [Book](https://doc.rust-lang.org/book/)
  * [Command Line Applications in Rust](https://rust-cli.github.io/book/)
  * [Learn Rust](https://www.rust-lang.org/learn)
  * [Reference Guide](https://doc.rust-lang.org/reference/index.html)
  * [RFC Book](https://rust-lang.github.io/rfcs/)
  * [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
  * [Rust by Example](https://doc.rust-lang.org/rust-by-example/index.html)
  * [Rust Conversions](https://nicholasbishop.github.io/rust-conversions/)
  * [Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/)
  * [Rust Design Patterns](https://rust-unofficial.github.io/patterns/)
  * [Secure Rust Guidelines](https://anssi-fr.github.io/rust-guide/)
  * [The Little Book of Rust Macros](https://veykril.github.io/tlborm/) [[mirror](https://danielkeep.github.io/tlborm/book/)]
  * [The `rustdoc` book](https://doc.rust-lang.org/rustdoc/)
  * [The `rustup` book](https://rust-lang.github.io/rustup/)
* [How to Handle Errors in Rust: A Comprehensive Guide](https://dev.to/nathan20/how-to-handle-errors-in-rust-a-comprehensive-guide-1cco)
* [Learning Rust](https://github.com/danbev/learning-rust)
* [Memory safety for the Internet's most critical infrastructure](https://www.memorysafety.org/)
  * [What is memory safety and why does it matter?](https://www.memorysafety.org/docs/memory-safety/)
* [Procedural Macros Workshop](https://github.com/dtolnay/proc-macro-workshop)
* [Source Code](https://github.com/rust-lang/rust)
* [The Rust community’s crate registry](https://crates.io/)

## Installation

Using a distribution independent method (recommended):
```sh
$ : "${RUST_PROFILE:=default}"  # Other possibilities: minimal, complete
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh -s -- -y --profile ${RUST_PROFILE}
```
* this installs Rust under user's home directory, isolated from distribution
  installation
  * `PATH` environment variable is updated to include `~/.cargo/bin`
  * needs a shell to be restarted or `source ~/.cargo/env` to be run
* some components are installed via `rustup`:
  ```sh
  rustup component add ...
  ```
* to use binaries installed using `rustup component add ...`, additional
  location need to be added to `PATH`:
  ```sh
  $ X=$(rustc --print target-libdir)
  $ PATH=${X%/*}/bin:${PATH} ...
  ```

### GitHub Actions

* [`actions-rust-lang/setup-rust-toolchain`](https://github.com/actions-rust-lang/setup-rust-toolchain)

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
  $ cargo doc --open
  ```
* To find unused dependencies:
  * using [`cargo machete`](https://github.com/bnjbvr/cargo-machete)
  * using [`cargo +nightly udeps`](https://github.com/est31/cargo-udeps)
  * using `RUSTFLAGS=-Wunused-crate-dependencies`:
    ```sh
    $ export RUSTFLAGS=-Wunused-crate-dependencies
    $ cargo build
    $ cargo check --all --all-targets
    ```
  * see more [here](https://gist.github.com/helio-frota/f6a48303aefdc22578661babacb153dd)
* To DRY up `Cargo.toml` manifests:
  * [`cargo autoinherit`](https://github.com/mainmatter/cargo-autoinherit)
* To run your custom command/task, see:
  * [`cargo xtask`](https://github.com/matklad/cargo-xtask/)
  * [Custom tasks in Cargo](http://aturon.github.io/tech/2018/04/05/workflows/)
* To install a crate from its source, type:
  ```sh
  $ cargo install
  ```
  * to download and install binary build of crate use
    [`cargo-binstall`](https://github.com/cargo-bins/cargo-binstall) extension:
    ```sh
    $ cargo binstall
    ```
* To prune crate dependencies in target folder:
  * [`cargo prune`](https://github.com/ustulation/cargo-prune) [[crate](https://crates.io/crates/cargo-prune)]
* To cleanup `${CARGO_HOME}` cache:
  * [`cargo trim`](https://github.com/iamsauravsharma/cargo-trim) [[crate](https://crates.io/crates/cargo-trim)]

### Rust Analyzer

[`rust-analyzer`](https://github.com/rust-lang/rust-analyzer)
([home](https://rust-analyzer.github.io/)) is an implementation of Language
Server Protocol for the Rust programming language.

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

#### Caching

* [Is a shared build cache a good fit for Cargo?](https://internals.rust-lang.org/t/is-a-shared-build-cache-a-good-fit-for-cargo/)
* [`sccache` - shared compilation cache](https://github.com/mozilla/sccache) [[doc](https://docs.rs/sccache/latest/sccache/)] [[crate](https://crates.io/crates/sccache)]

#### CI & Code Coverage

Examples:
* [atty](https://github.com/softprops/atty)
* [fantoccini](https://github.com/jonhoo/fantoccini)
* [Rust CI with GitHub Actions](https://github.com/BamPeers/rust-ci-github-actions-workflow)
* [Tantivy](https://github.com/quickwit-oss/tantivy)

References:
* [How to do code coverage in Rust](https://blog.rng0.io/how-to-do-code-coverage-in-rust)
* [Instrumentation-based Code Coverage](https://doc.rust-lang.org/rustc/instrument-coverage.html)
* [Source-based Code Coverage](https://blog.rust-lang.org/2022/04/07/Rust-1.60.0.html#source-based-code-coverage)
  * [LLVM Source-Based Code Coverage](https://rustc-dev-guide.rust-lang.org/llvm-coverage-instrumentation.html)
* [Upstream Issues](https://github.com/rust-lang/rust/labels/A-code-coverage)

Tools:
* [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov) [[doc](https://docs.rs/cargo-llvm-cov/latest/cargo_llvm_cov/)] [[crate](https://crates.io/crates/cargo-llvm-cov)]
* [grcov](https://github.com/mozilla/grcov) [[doc](https://docs.rs/grcov/latest/grcov/)] [[crate](https://crates.io/crates/grcov)]
* [kcov](https://github.com/SimonKagstrom/kcov)

#### Dynamic Linking

* [Is there a way to dynamic link with all dependencies?](https://www.reddit.com/r/rust/comments/11xc6c4/is_there_a_way_to_dynamic_link_with_all/)

#### GitHub Actions

* [Rust Cache Action](https://github.com/Swatinem/rust-cache)

### Miri

[Miri](https://github.com/rust-lang/miri) is a mid-level intermediate
representation interpreter. It can run Rust programs and detect certain classes
of undefined behavior.

### Nextest

[Nextest](https://github.com/nextest-rs/nextest)
([documentation](https://nexte.st/)) is a next generation test runner for Rust.

## Lexical Elements

Grammar:
```
token:
    keyword
    weak_keyword
    identifier
    char_literal
    byte_literal
    string_literal
    raw_string_literal
    byte_string_literal
    raw_byte_string_literal
    integer_literal
    float_literal
    lifetime_token
    lifetime_or_label
    punctuation
    delimiters
    reserved_token_double_quote
    reserved_token_single_quote
    reserved_token_pound

lifetime_token:
    "'" identifier_or_keyword
    "'" "_"
lifetime_or_label:
    "'" non_keyword_identifier

punctuation:
    "+" | "-" | "*" | "/" | "%" | "^" | "!" | "&" | "|" | "&&" | "||" | "<<"
    ">>" | "+=" | "-=" | "*=" | "/=" | "%=" | "^=" | "&=" | "|=" | "<<="
    ">>=" | "=" | "==" | "!=" | ">" | "<" | ">=" | "<=" | "@" | "_" | "."
    ".." | "..." | "..=" | "," | ";" | ":" | "::" | "->" | "=>" | "#" | "$"
    "?" | "~"
delimiters:
    "(" | ")" | "[" | "]" | "{" | "}"

reserved_token_double_quote:
    ((identifier_or_keyword - ("b" | "r" | "br")) | "_") '"'
reserved_token_single_quote:
    ((identifier_or_keyword - "b") | "_") "'"
reserved_token_pound:
    ((identifier_or_keyword - ("r" | "br")) | "_") "#"

reserved_number:
    bin_literal ("2" | "3" | "4" | "5" | "6" | "7" | "8" | "9")
    oct_literal ("8" | "9")
    (bin_literal | oct_literal | hex_literal)
        "."{not-followed-by ("." | "_" | <XID start Unicode character>)}
    (bin_literal | oct_literal) ("e" | "E")
    "0b" "_"* (<end of input> | !bin_digit)
    "0o" "_"* (<end of input> | !oct_digit)
    "0x" "_"* (<end of input> | !hex_digit)
    dec_literal ("." dec_literal)? ("e" | "E") ("+" | "-")?
        (<end of input> | !dec_digit)

suffix:
    identifier_or_keyword
suffix_no_e:
    suffix - (("e" | "E").*)
isolated_cr:
    <a U+000D not followed by a U+000A>

utf8bom:
    U+FEFF
shebang:
    "#!" (!U+000A)+
```
* Rust input is viewed as a sequence of UTF-8 characters
* a `reserved_number` is rejected by the tokenizer instead of tokenized to
  separate tokens
* see [Tokens](https://doc.rust-lang.org/reference/tokens.html) for greater
  detail

### Whitespace

Grammar:
```
whitespace:
    U+0009  # horizontal tab
    U+000A  # line feed
    U+000B  # vertical tab
    U+000C  # form feed
    U+000D  # carriage return
    U+0020  # space
    U+0085  # next line
    U+200E  # left-to-right mark
    U+200F  # right-to-left mark
    U+2028  # line separator
    U+2029  # paragraph separator
    <unicode character that have the pattern white space property>
```
* `whitespace` characters are ignored
* see [Whitespace](https://doc.rust-lang.org/reference/whitespace.html) for
  greater detail

### Comments

Grammar:
```
line_comment:
    "//" (!("/" | "!" | U+000A) | "//") (!U+000A)*
    "//"
block_comment:
    "/*" (!("*" | "!") | "**" | block_comment_or_doc)
        (block_comment_or_doc | !"*/")*
        "*/"
    "/**/"
    "/***/"
inner_line_doc:
    "//!" (!(U+000A | isolated_cr))*
inner_block_doc:
    "/*!" (block_comment_or_doc | !("*/" | isolated_cr))* "*/"
outer_line_doc:
    "///" (!"/" (!(U+000A | isolated_cr))*)?
outer_block_doc:
    "/**" (!"*" | block_comment_or_doc)
        (block_comment_or_doc | !("*/" | isolated_cr))*
        "*/"

block_comment_or_doc:
    block_comment
    outer_block_doc
    inner_block_doc
```
* comments are ignored by `rustc` but not by particular tools (`cargo doc`
  etc.)

Examples:
```rust
// This is a single line comment.

/*
 * This is a block comment.
 */

//! This inner doc line comment.

/*!
 * This is inner doc block comment.
 */

/// This is outer doc line comment.

/**
 * This is outer doc block comment.
 */
```

See [Comments](https://doc.rust-lang.org/reference/comments.html) for greater
detail.

### Keywords

Grammar:
```
keyword:
    "as" | "break" | "const" | "continue" | "crate" | "else" | "enum"
    "extern" | "false" | "fn" | "for" | "if" | "impl" | "in" | "let"
    "loop" | "match" | "mod" | "move" | "mut" | "pub" | "ref" | "return"
    "self" | "Self" | "static" | "struct" | "super" | "trait" | "true"
    "type" | "unsafe" | "use" | "where" | "while"
    # 2018+:
    "async" | "await" | "dyn"
    # Reserved:
    "abstract" | "become" | "box" | "do" | "final" | "macro" | "override"
    "priv" | "typeof" | "unsized" | "virtual" | "yield"
    # 2018+:
    "try"
weak_keyword:
    "macro_rules" | "union" | "'static"
    # 2015:
    "dyn"
```

See [Appendix A: Keywords](https://doc.rust-lang.org/book/appendix-01-keywords.html)
from the [book](https://doc.rust-lang.org/book/) or [Keywords](https://doc.rust-lang.org/reference/keywords.html)
for greater detail.

### Identifiers

Grammar:
```
identifier_or_keyword:
    <XID start Unicode character> <XID continue Unicode character>*
    "_" <XID continue Unicode character>+
raw_identifier:
    "r#" (identifier_or_keyword - ("crate" | "self" | "super" | "Self"))
non_keyword_identifier:
    identifier_or_keyword - keyword

identifier:
    non_keyword_identifier
    raw_identifier
```

See [Identifiers](https://doc.rust-lang.org/reference/identifiers.html) for
greater detail.

### Literals

* any literal may end with suffix which is an identifier or keyword
* a suffix can annotate a literal with type or it can serve as syntactical
  sugar in token stream processed during macro expansion

#### Character Literals

Grammar:
```
char_literal:
    "'" (
        !("'" | r"\" | U+000A | U+000D | U+0009) |
        quote_escape | ascii_escape | unicode_escape
    ) "'" suffix?

quote_escape:
    r"\'" | r'\"'
ascii_escape:
    r"\x" oct_digit hex_digit
    r"\n" | r"\r" | r"\t" | r"\\" | r"\0"
unicode_escape:
    r"\u{" (hex_digit "_"*){1,6} "}"
```
* a character between quotes is any Unicode Scalar Value (U+0000 to U+D7FF and
  U+E000 to U+10FFFF inclusive) except single quote (U+0027), backslash
  (U+005C), new line (U+000A), carriage return (U+000D), and tab character
  (U+0009)
* the type of character literal is `chr`
* see [Character literals](https://doc.rust-lang.org/reference/tokens.html#character-literals)
  for greater detail

#### String Literals

Grammar:
```
string_literal:
    '"' (
        !('"' | r"\" | isolated_cr) |
        quote_escape | ascii_escape | unicode_escape | string_continue
    )* '"' suffix?
string_continue:
    <r"\" followed by U+000A>

raw_string_literal:
    "r" raw_string_content suffix?
raw_string_content:
    '"' (!isolated_cr){non-greedy *} '"'
    "#" raw_string_content "#"

byte_string_literal:
    'b"' (
        ascii_for_string | byte_escape | string_continue
    )* '"' suffix?
ascii_for_string:
    <any ASCII (i.e. '\0' to '\x7f'), except '"', '\\' and isolated_cr>

raw_byte_string_literal:
    "br" raw_byte_string_content suffix?
raw_byte_string_content"
    '"' ascii{non-greedy *} '"'
    "#" raw_byte_string_content "#"
ascii:
    <any ASCII (i.e. '\0' to '\x7f')>
```
* a character in a `string_literal` is any Unicode Scalar Value (U+0000 to
  U+D7FF and U+E000 to U+10FFFF inclusive) except double quote (U+0022),
  backslash (U+005C), and sole carriage return (U+000D); U+000D U+000A is
  translated to U+000A
* in a `string_literal`, if `(U+000D? U+000A)` immediatelly follows a backslash
  character, then the backslash character, the `(U+000D? U+000A)` and the
  following string containing only U+0020, U+000A, U+000D and U+0009 characters
  are removed from the `string_literal`
* a character in a `raw_string_literal` is any Unicode Scalar Value (U+0000 to
  U+D7FF and U+E000 to U+10FFFF inclusive) except sole carriage return (U+000D)
* `raw_string_literal` and `raw_byte_string_literal` do not process any escape
  sequence
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
byte_literal:
    "b'" (ascii_for_char | byte_escape) "'" suffix?
ascii_for_char:
    <any ASCII (i.e. '\0' to '\x7f') except '\'', '\\', '\n', '\r' or '\t'>
byte_escape:
    r"\x" hex_digit hex_digit
    r"\n" | r"\r" | r"\t" | r"\\" | r"\0" | r"\'" | r'\"'

integer_literal:
    dec_literal suffix_no_e?
    bin_literal suffix_no_e?
    oct_literal suffix_no_e?
    hex_literal suffix_no_e?

dec_literal:
    dec_digit (dec_digit | "_")*
bin_literal:
    "0b" (bin_digit | "_")* bin_digit (bin_digit | "_")*
oct_literal:
    "0o" (oct_digit | "_")* oct_digit (oct_digit | "_")*
hex_literal:
    "0x" (hex_digit | "_")* hex_digit (hex_digit | "_")*

bin_digit:
    "0" | "1"
oct_digit:
    bin_digit | "2" | "3" | "4" | "5" | "6" | "7"
dec_digit:
    oct_digit | "8" | "9"
hex_digit:
    dec_digit
    "a" | "b" | "c" | "d" | "e" | "f"
    "A" | "B" | "C" | "D" | "E" | "F"
```
* `_` works as a digit separator and is ignored (increases number readability)
* after macro expansion, `suffix` should be one of `u8`, `i8`, `u16`, `i16`,
  `u32`, `i32`, `u64`, `i64`, `u128`, `i128`, `usize` or `isize`
  * `byte_literal` `suffix` should be `u8`
* if there is no type suffix, `i32` is used
* see [Byte literals](https://doc.rust-lang.org/reference/tokens.html#byte-literals),
  [Integer literals](https://doc.rust-lang.org/reference/tokens.html#integer-literals)
  and [Integer literal expressions](https://doc.rust-lang.org/reference/expressions/literal-expr.html#integer-literal-expressions)
  for greater detail

#### Floating Point Literals

Grammar:
```
float_literal:
    dec_literal "."{not-followed-by ("." | "_" | <XID start Unicode character>)}
    dec_literal "." dec_literal suffix_no_e?
    dec_literal ("." dec_literal)? float_exponent suffix?

float_exponent:
    ("e" | "E") ("+" | "-")? (dec_digit | "_")* dec_digit (dec_digit | "_")*
```
* after macros are expanded, `suffix` should be either `f32` or `f64`
* if there is no type suffix, `f64` is used
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

#### Tuple Types

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

#### Array Types

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

#### Struct Types

* a `struct` type is a heterogeneous product of other types
* `struct`s have no specified memory layout
  * to specify one, use [`repr`](https://doc.rust-lang.org/reference/type-layout.html#representations)
    attribute
* see [Struct types](https://doc.rust-lang.org/reference/types/struct.html) for
  greater detail

#### Enumerated Types

* nominal, heterogeneous disjoint union types
* any `enum` value consumes as much memory as the largest variant for its
  corresponding `enum` type plus the size of discriminant
* see [Enumerated types](https://doc.rust-lang.org/reference/types/enum.html)
  for greater detail

## Declarations

Grammar:
```
declaration:
    let_statement
    declaration_item

declaration_item:
    use_declaration
    constant_item
    static_item
    type_alias
    struct
    union
    enumeration
    function
    extern_block
```

### Use Declarations

Grammar:
```
use_declaration:
    "use" use_tree ";"

use_tree:
    (simple_path? "::")? ("*" | "{" (use_tree ("," use_tree)* ","?)? "}")
    simple_path ("as" (identifier | "_"))?
```
* a `use` declaration creates one or more local name bindings synonymous with
  some other path
* `use` declarations may appear in modules or blocks
* `use` declarations are resolved after macro expansion, e.g. this will not
  produce an error:
  ```rust
  macro_rules! m {
      ($x: item) => { $x $x }
  }

  m!(use std as _;);
  ```

Forms of a `use` declaration:
* `use a::b::c;` makes `c` to be an alias for `a::b::c` in the current scope
* `use a::b::c as foo;` makes `foo` to be an alias for `a::b::c` in the current
  scope
* `use a::b::*;` brings all the items defined under the `a::b` in the current
  scope
* `use a::{self, b::c, d, e::*, f::g as h};` is the equivalent of
  ```rust
  // `self` refers to the common parent module, hence `use a;`
  use a;
  use a::b::c;
  use a::d;
  use a::e::*;
  use a::f::g as h;
  ```
* nesting is also supported, e.g. `use a::b::{self as c, d::{self, *}};` is the
  equivalent of
  ```rust
  use a::b as c;
  use a::b::d;
  use a::b::d::*;
  ```
* `use path as _;` imports `path` without binding it to a name
  * `use path::to::trait as _;` imports trait's methods but not the `trait`
    symbol itself (recall that that to use trait's method it must be first
    imported)
* `use path::* as _;` imports all the items under the `path` in their
  unnameable form

See [Use declarations](https://doc.rust-lang.org/reference/items/use-declarations.html),
[Paths](https://doc.rust-lang.org/reference/paths.html),
[Modules](https://doc.rust-lang.org/reference/items/modules.html), and
[Block expressions](https://doc.rust-lang.org/reference/expressions/block-expr.html)
for greater detail.

### Variables

Grammar:
```
let_statement:
    outer_attribute* "let" pattern_no_top_alt (":" type)?
        ("=" expression ("else" block_expression)?)? ";"
```
* introduces a new set of variables given by a `pattern_no_top_alt`
* `pattern_no_top_alt` can be annotated with `type`
* variables in `pattern_no_top_alt` can be initialized by `expression`
* if `else` is not present, `pattern_no_top_alt` must be irrefutable
* if `else` is present
  * `pattern_no_top_alt` can be refutable
  * `expression` must not be a `lazy_boolean_expression` or end with a `}`
  * `block_expression` must evaluate to never type
* the semantics of `else` part is that if `pattern_no_top_alt` fails to match
  then the `block_expression` is executed

Variables:
* are allocated on stack frame, i.e. a variable can be
  * a named local variable
  * a named function parameter
  * an anonymous temporary (e.g. created during an expression evaluation)
* are defined as immutable by default
  * to define a mutable variable, use the `mut` keyword

  ```rust
  let x = 5;      // immutable variable
  let mut y = 7;  // mutable variable
  ```
* are scoped
* are not initialized
  * all variables must be initialized before their first use
* can be shadowed:
  ```rust
  let x = "foo";    // x is immutable and str
  let x = x.len();  // x is shadowed - still immutable but integer
  ```

See [Identifiers](https://doc.rust-lang.org/reference/identifiers.html),
[`let` statements](https://doc.rust-lang.org/reference/statements.html#let-statements),
[Variables](https://doc.rust-lang.org/reference/variables.html) and
[Temporaries](https://doc.rust-lang.org/reference/expressions.html#temporaries)
for greater detail.

### Constants

Grammar:
```
constant_item:
    "const" (identifier | "_") ":" type ("=" expression)? ";"
```

Constants are scoped and always immutable.

Example:
```rust
const THREE: u32 = 1 + 2;
```
* convention: use upper case and underscores for constant names

See [Constant items](https://doc.rust-lang.org/reference/items/constant-items.html)
and [Constant evaluation](https://doc.rust-lang.org/reference/const_eval.html)
for greater detail.

### Statics

Grammar:
```
static_item:
    "static" "mut"? identifier ":" type ("=" expression)? ";"
```

See [Static items](https://doc.rust-lang.org/reference/items/static-items.html)
for greater detail.

### Type Aliases

Grammar:
```
type_alias:
    "type" identifier generic_params? (":" type_param_bounds)? where_clause?
        ("=" type where_clause?)? ";"
```

See [Type aliases](https://doc.rust-lang.org/reference/items/type-aliases.html)
for greater detail.

### Structs

Grammar:
```
struct:
    "struct" identifier generic_params? where_clause?
        ("{" struct_fields? "}" | ";")
    "struct" identifier generic_params? "(" tuple_fields? ")" where_clause? ";"

struct_fields:
    struct_field ("," struct_field)* ","?
struct_field:
    outer_attribute* visibility? identifier ":" type

tuple_fields:
    tuple_field ("," tuple_field)* ","?
tuple_field:
    outer_attribute* visibility? type
```

#### Structs with Named Fields

Declaration:
```rust
struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}
```
* declare a new type `Point3D` as a struct containing three `f64` items named
  `x`, `y` and `z`, respectively

Making an instance:
```rust
let p1 = Point3D {x: 0.5, y: -1.2, z: 1.0};      // (1)
let mut p2 = Point3D {z: 1.0, y: -1.2, x: 0.5};  // (2)
```
* at `(1)` an instance of `Point3D` is created and assigned to `p1`
* at `(2)` happens the same but this time the instance is assigned to `p2`
* the order of initializers does not matter in Rust, thus `p1` and `p2` are
  equal

Accessing an element:
```rust
p2.x += p1.x;
```
* an element of a struct is accessed by its name using dot operator (`.`)

Field init shorthand:
```rust
fn x_axis_point(x: f64) -> Point3D {
    Point {y: 0.0, z: 0.0, x}
}
```
* if the name of a variable coincides with the name of a field then initializer
  `name: name` can be shortened to just `name`

Struct update syntax:
```rust
let origin = Point3D {x: 0.0, y: 0.0, z: 0.0};
let z1 = Point3D {z: 1.0, ..origin};
```
* missing initializers are taken from `origin`, so the `z1` is equal to
  `Point3D {x: 0.0, y: 0.0, z: 1.0}`
* note that in case `Point3D` contains a field that can be only moved (e.g. a
  field of `String` type), then `origin` cannot be used after the assignment to
  `z1` is finished
* `z1` must be of the same type as `origin`

#### Tuple-like Structs

Declaration:
```rust
struct Color(u8, u8, u8);
```
* declare a new type `Color` as a struct containing three `u8` elements

Making an instance:
```rust
let red = Color(255, 0, 0);
let ctor = Color;
let blue = ctor(0, 0, 255);
```
* create an instance of `Color` and assign it to `red`
* `Color` behaves like function/constructor

Accessing an element:
```rust
let green = red.1;
```
* elements are accessed like in tuples

Struct update:
```rust
let black = Color(0, 0, 0);
let red = Color {0: 255, ..black};
```
* decimal integer literal as a field name specifies which field is updated
* `..origin` sets the rest of fields from `origin` using the copy/borrow
  strategy

#### Unit-like Structs

Declaration:
```rust
struct Ground;
struct Sink;
```
* declare two new distinct types, `Ground` and `Sink`, with no elements
* unit-like structs become useful when used together with traits

Making an instance:
```rust
let ground = Ground;
let ground2 = Ground {};
```
* create an instance of `Ground` and assign it to `ground`
* `Ground` can be optionally followed by `{}` to explicitly denote there are no
  fields

See [Structs](https://doc.rust-lang.org/reference/items/structs.html) and
[Struct expressions](https://doc.rust-lang.org/reference/expressions/struct-expr.html)
for greater detail.

### Unions

Grammar:
```
union:
    "union" identifier generic_params? where_clause? "{" struct_fields "}"
```

See [Unions](https://doc.rust-lang.org/reference/items/unions.html) for greater
detail.

### Enumerations

Grammar:
```
enumeration:
    "enum" identifier generic_params? where_clause? "{" enum_items? "}"

enum_items:
    enum_item ("," enum_item)* ","?
enum_item:
    outer_attribute* visibility?
        identifier (enum_item_tuple | enum_item_struct)?
        enum_item_discriminant?

enum_item_tuple:
    "(" tuple_fields? ")"
enum_item_struct:
    "{" struct_fields? "}"
enum_item_discriminant:
    "=" expression
```

Enumerations represent a sum of enumeration types distinguished by
constructors.

Definition and use of enumerations:
```rust
enum Animal {
    // Enum variant:
    Dog(String, f64),
    // Struct-like enum variant:
    Cat { name: String, weight: f64 },
    // Unit variant:
    Mouse,
}

let mut a: Animal = Animal::Dog("Sunny".to_string(), 13.5);
a = Animal::Cat { name: "Ginger".to_string(), weight: 4.7 };
a = Animal::Mouse;

// Values are extracted using pattern matching:
if let Animal::Cat { name, _ } == a {
    println!("Cat's name is {name}");
}
```

Like structs, also enumerations support defining methods on them:
```rust
enum FileError {
    NotFound,
    Read,
    Write,
}

impl FileError {
    fn detail(&self) -> String {
        match self {
            FileError::NotFound => String::from("File not found"),
            FileError::Read => String::from("Error while reading"),
            FileError::Write => String::from("Error while writing"),
        }
    }
}
```

Syntactically, enumerations allow to use a visibility annotation for their
variants but this is rejected during the validation:
```rust
// Syntactical macros can use the enum definition to generate a code and throw
// out the old enum definition so it will not be analyzed by semantic analysis
// (if so, it will be rejected)
#[some_macro("foo")]
enum Enum {
    pub A,
    pub(crate) B(),
}
```

A *field-less enum* is an enum where no constructors contain field:
```rust
enum FieldLessEnum {
    CtorA(),
    CtorB{},
    CtorC,
}
```

A *unit-only enum* only contains unit variants:
```rust
enum UnitOnly {
    UnitA,
    UnitB,
    UnitC,
}
```

A *zero-variant enum* is an enum with no variants and thus it cannot be
instantiated:
```rust
enum ZeroVariants {}
```
* zero-variant enums are equivalent to never type:
  ```rust
  let x: ZeroVariants = panic!();
  ```
* coercion into other types is not allowed:
  ```rust
  let y: u32 = x;  // type mismatch
  ```

#### Discriminants

A discriminant is a number associated with a constructor used to distinguish
between variants of one enum instance.
* its type is `isize` under the [default representation](https://doc.rust-lang.org/reference/type-layout.html#the-default-representation)
* however, compiler is allowed to use a smaller type in its actual memory
  layout

##### Discriminant Values

A discriminant value can be set in two ways:
1. **Implicitly**, if the value of the discriminant is not specified
   explicitly:
   * the value of the discriminant is the value of the discriminant of the
     previous variant plus one
   * if the value of the discriminant of the first variant is not specified
     explicitly it is set to zero

   Example:
   ```rust
   // Unit only enumeration => setting discriminants explicitly is allowed
   enum Example {
       VarA,        // Implicitly set to 0
       VarB = 123,  // Explicitly set to 123
       VarC,        // Implicitly set to 124
   }
   ```
1. **Explicitly**, using `=` followed by a [constant expression](https://doc.rust-lang.org/reference/const_eval.html#constant-expressions),
   under these circumstances:
   * the enumeration is unit-only
   * a [primitive representation](https://doc.rust-lang.org/reference/type-layout.html#primitive-representations) is used

   Examples:
   ```rust
   #[repr(u8)]      // A primitive (u8) representation (discriminant values ranges from 0 to 255)
   enum Enum {
       Unit = 3,    // Unit = 3 (set explicitly)
       Tuple(u16),  // Tuple = 4 (set implicitly)
       Struct {     // Struct = 1 (set explicitly)
           a: u8,
           b: u16,
       } = 1,
   }

   enum Bad1 {
       A = 1,
       B = 1,   // ERROR: 1 is already used
   }

   enum Bad2 {
       A,      // Implicitly set to 0
       B,      // Implicitly set to 1
       C = 1,  // ERROR: 1 is already used
   }

   #[repr(u8)]
   enum Bad3 {
       A = 255,  // Explicitly set to 255
       B,        // ERROR: Implicitly set to 256 which cannot fit to u8 (overflow)
   }
   ```

##### How to Get the Discriminant Value

* using [`std::mem::discriminant`](https://doc.rust-lang.org/std/mem/fn.discriminant.html)
  (can be used only for `==` and `!=` comparison)
  ```rust
  enum Enum {
    VarA(&'static str),
    VarB(i32),
    VarC(i32),
  }

  assert_eq!(mem::discriminant(&Enum::VarA("abc")), mem::discriminant(&Enum::VarA("def")));
  assert_ne!(mem::discriminant(&Enum::VarC(2)), mem::discriminant(&Enum::VarC(3)));
  ```
* via typecasting (can be used only for enums having only unit variants or for
  field-less enums where only unit variants are explicit)
  ```rust
  enum Enum {
      A,  // 0
      B,  // 1
      C,  // 2
  }

  assert_eq!(Enum::B as isize, 1);

  #[repr(u8)]
  enum FieldLess {
      Tuple(),            // 0
      Struct{},           // 1
      Unit,               // 2
      ExplicitUnit = 42,  // 42
  }

  assert_eq!(FieldLess::Tuple() as u8, 0);
  assert_eq!(FieldLess::Struct{} as u8, 1);
  assert_eq!(FieldLess::ExplicitUnit as u8, 42);

  #[repr(u8)]
  enum FieldLess2 {
      Tuple() = 2,
      Unit,
  }

  // ERROR: Typecast cannot be used as non-unit variant's discriminant has been
  //        set explicitly
  // assert_eq!(FieldLess2::Unit as u8, 3);
  ```
* via (unsafe) pointer casting (can be used only for enums using a primitive
  representation)
  ```rust
  #[repr(u8)]
  enum Foo {
      A,                         // 0
      B { a: i16, b: i16 } = 3,  // 3
      C(i32) = 5,                // 5
  }

  impl Foo {
      fn discriminant(&self) -> u8 {
          unsafe { *(self as *const Self as *const u8) }
      }
  }

  let a = Foo::A;
  let b = Foo::B{a: -1, b: 4};
  let c = Foo::C(3);

  assert_eq!(a.discriminant(), 0);
  assert_eq!(b.discriminant(), 3);
  assert_eq!(c.discriminant(), 5);
  ```

See [Enumerations](https://doc.rust-lang.org/reference/items/enumerations.html),
[Enumerated types](https://doc.rust-lang.org/reference/types/enum.html),
[Struct expressions](https://doc.rust-lang.org/reference/expressions/struct-expr.html),
[The `Rust` Representation](https://doc.rust-lang.org/reference/type-layout.html#the-default-representation),
and [Primitive representations](https://doc.rust-lang.org/reference/type-layout.html#primitive-representations)
for greater detail.

### Functions

Grammar:
```
function:
    function_qualifiers "fn" identifier generic_params?
        "(" function_parameters? ")"
        function_return_type? where_clause?
        (block_expression | ";")

function_qualifiers:
    "const"? "async"? "unsafe"? ("extern" abi?)?
abi:
    string_literal
    raw_string_literal

function_parameters:
    self_param ","?
    (self_param ",")? function_param ("," function_param)* ","?
self_param:
    outer_attribute* (shorthand_self | typed_self)
shorthand_self:
    ("&" lifetime?)? "mut"? "self"
typed_self:
    "mut"? "self" ":" type
function_param:
    outer_attribute* (function_param_pattern | "..." | type)
function_param_pattern:
    pattern_no_top_alt ":" (type | "...")

function_return_type:
    "->" type
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

### External Blocks

Grammar:
```
extern_block:
    "unsafe"? "extern" abi? "{" inner_attribute* external_item* "}"

external_item:
    outer_attribute* (
        macro_invocation_semi |
        (visibility? (static_item | function))
    )
```

See [External blocks](https://doc.rust-lang.org/reference/items/external-blocks.html)
for greater detail.

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

> [!NOTE]
> If a type or any of its part implements the `Drop` trait it cannot be
> annotated with (it cannot implement) the `Copy` trait.

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
> [!NOTE]
> 1. A range `a..b` used to make a slice must have `a <= b`, where both `a` and
>    `b` are of the `usize` type.
> 1. String (`str`) slice range indices must occur at valid UTF-8 character
>    boundaries. Otherwise the program panics.
> 1. String literals have a type `&str` since they are slices of a binary data
>    stored in the data section of a program.
> 1. `String` implements the `Deref` trait that converts `&String` to `&str` by
>    calling a `deref` method (the code is generated by Rust during compile
>    time). As a consequence, if a function `fun` has a signature
>    `fn fun(s: &str)` and `x` has a type `&String` we can `fun(x)` without any
>    worries.

## Expressions

Grammar:
```
expression:
    expression_without_block
    expression_with_block

expression_without_block:
    outer_attribute* (
        break_expression |
        continue_expression |
        return_expression |
        closure_expression |
        operator_expression |
        range_expression |
        index_expression |
        call_expression |
        field_expression |
        tuple_indexing_expression |
        method_call_expression |
        await_expression |
        path_expression |
        async_block_expression |
        atomic_expression |
        grouped_expression |
        macro_invocation
    )
expression_with_block:
    outer_attribute* (
        loop_expression |
        if_expression |
        if_let_expression |
        match_expression |
        block_expression |
        unsafe_block_expression
    )

operator_expression:
    assignment_expression
    compound_assignment_expression
    lazy_boolean_expression
    comparison_expression
    arithmetic_or_logical_expression
    type_cast_expression
    negation_expression
    dereference_expression
    borrow_expression
    error_propagation_expression
```

References:
* [Expressions reference](https://doc.rust-lang.org/reference/expressions.html)
* [Operators and symbols list](https://doc.rust-lang.org/book/appendix-02-operators.html) (see also [here](https://doc.rust-lang.org/reference/tokens.html#punctuation))
* [Operator precedence](https://doc.rust-lang.org/reference/expressions.html#expression-precedence)

### Loop Expressions

Grammar:
```
loop_expression:
    loop_label? (
        "loop" block_expression
        "while" (expression - struct_expression) block_expression
        "while" "let" pattern "=" (scrutinee - lazy_boolean_expression)
            block_expression
        "for" pattern "in" (expression - struct_expression) block_expression
        block_expression
    )
loop_label:
    lifetime_or_label ":"

break_expression:
    "break" lifetime_or_label? expression?
continue_expression:
    "continue" lifetime_or_label?
```

`loop { body }`:
* execute `body` infinitely
* if `body` does not contain `break`, the type of the expression is `!`;
  otherwise, the type of the expression is the type of the `break` expression
* the type of the expression must be compatible with the type of every `break`
  expression inside `body`
* the value of the expression is the value returned by a `break` expression
  from `body`

`while condition { body }`:
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
    "if" (expression - struct_expression) block_expression
        ("else" (block_expression | if_expression | if_let_expression))?

if_let_expression:
    "if" "let" pattern "=" (scrutinee - lazy_boolean_expression) block_expression
        ("else" (block_expression | if_expression | if_let_expression))?
```
* the conditional `expression` must not be `struct_expression` and must be of
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
    } else {
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

Example:
```rust
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

    println!("{}", decide(200, &a, 42));  // prints "200"
    println!("{}", decide(20, &a, 42));   // prints "8"
    println!("{}", decide(20, &b, 42));   // prints "42"
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

### Closure Expressions

Grammar:
```
closure_expression:
    "move"? ("||" | "|" closure_parameters? "|") (
        expression | "->" type_no_bounds block_expression
    )

closure_parameters:
    closure_param ("," closure_param)* ","?

closure_param:
    outer_attribute* pattern_no_top_alt (":" type)?
```

See [Closure expressions](https://doc.rust-lang.org/reference/expressions/closure-expr.html)
for greater detail.

### Assignment Expressions

Grammar:
```
assignment_expression:
    expression "=" expression

compound_assignment_expression:
    expression "+=" expression
    expression "-=" expression
    expression "*=" expression
    expression "/=" expression
    expression "%=" expression
    expression "&=" expression
    expression "|=" expression
    expression "^=" expression
    expression "<<=" expression
    expression ">>=" expression
```

See [Assignment expressions](https://doc.rust-lang.org/reference/expressions/operator-expr.html#assignment-expressions)
and [Compound assignment expressions](https://doc.rust-lang.org/reference/expressions/operator-expr.html#compound-assignment-expressions)
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
    expression "||" expression
    expression "&&" expression
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

### Comparison Expressions

Grammar:
```
comparison_expression:
    expression "==" expression
    expression "!=" expression
    expression ">" expression
    expression "<" expression
    expression ">=" expression
    expression "<=" expression
```

See [Comparison Operators](https://doc.rust-lang.org/reference/expressions/operator-expr.html#comparison-operators)
for greater detail.

### Arithmetic and Logical Expressions

Grammar:
```
arithmetic_or_logical_expression:
    expression "|" expression
    expression "^" expression
    expression "&" expression
    expression "<<" expression
    expression ">>" expression
    expression "+" expression
    expression "-" expression
    expression "*" expression
    expression "/" expression
    expression "%" expression
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

### Type Cast Expressions

Grammar:
```
type_cast_expression:
    expression "as" type_no_bounds
```

See [Type cast expressions](https://doc.rust-lang.org/reference/expressions/operator-expr.html#type-cast-expressions)
for greater detail.

### Negation Expressions

Grammar:
```
negation_expression:
    "-" expression
    "!" expression
```

See [Negation operators](https://doc.rust-lang.org/reference/expressions/operator-expr.html#negation-operators)
for greater detail.

### Dereference Expressions

Grammar:
```
dereference_expression:
    "*" expression
```

See [The dereference operator](https://doc.rust-lang.org/reference/expressions/operator-expr.html#the-dereference-operator)
for greater detail.

### Borrow Expressions

Grammar:
```
borrow_expression:
    ("&" | "&&") "mut"? expression
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

### Error Propagation Expressions

Grammar:
```
error_propagation_expression:
    expression "?"
```

See [The question mark operator](https://doc.rust-lang.org/reference/expressions/operator-expr.html#the-question-mark-operator)
for greater detail.

### Array Index Expressions

Grammar:
```
index_expression:
    expression "[" expression "]"
```

See [Array and slice indexing expressions](https://doc.rust-lang.org/reference/expressions/array-expr.html#array-and-slice-indexing-expressions)
for greater detail.

### Call Expressions

Grammar:
```
call_expression:
    expression "(" call_params? ")"

call_params:
    expression ("," expression)* ","?
```

See [Call expressions](https://doc.rust-lang.org/reference/expressions/call-expr.html)
for greater detail.

### Field Access Expressions

Grammar:
```
field_expression:
    expression "." identifier

tuple_indexing_expression:
    expression "." integer_literal
```
* a tuple index is compared to the `integer_literal` directly
  * that is, `t.0` is valid but `t.01` or `t.0o0` are invalid
* see [Field access expressions](https://doc.rust-lang.org/reference/expressions/field-expr.html)
  and [Tuple indexing expressions](https://doc.rust-lang.org/reference/expressions/tuple-expr.html#tuple-indexing-expressions)
  for greater detail

### Method-Call Expressions

Grammar:
```
method_call_expression:
    expression "." path_expr_segment "(" call_params? ")"
```

`expression` in the grammar above is called *receiver*.

Here is how receiver and method are resolved:
1. Build a list, *L*, of candidate receiver types.
   1. Repeatedly [dereference](https://doc.rust-lang.org/reference/expressions/operator-expr.html#the-dereference-operator)
      receiver's expression type, add each encountered type to *L*.
   1. Let *T* be the last type in *L*. Apply [unsized coercion](https://doc.rust-lang.org/reference/type-coercions.html#unsized-coercions)
      to *T* and add the result, if any, to *L*.
1. For each `T` in *L*, add `&T` and `&mut T` to *L* immediately after `T`.
1. For every *T* in *L*, search for a visible method with a receiver of type
   *T* in these places:
   1. Methods implemented directly on *T*.
   1. Any of the methods provided by a visible trait implemented by *T*.
      * If *T* is a type parameter, methods provided by trait bounds on *T* are
        looked up first.
      * Then all remaining methods in scope are looked up.
1. If the look up failed or there are ambiguities an error is issued.

See [Method-call expressions](https://doc.rust-lang.org/reference/expressions/method-call-expr.html)
for greater detail.

### Await Expressions

Grammar:
```
await_expression:
    expression "." "await"
```

See [Await expressions](https://doc.rust-lang.org/reference/expressions/await-expr.html)
for greater detail.

### Path Expressions

Grammar:
```
path_expression:
    path_in_expression
    qualified_path_in_expression
```

See [Path expressions](https://doc.rust-lang.org/reference/expressions/path-expr.html)
for greater detail.

### `match` Expressions

Grammar:
```
match_expression:
    "match" scrutinee "{" inner_attribute* match_arms? "}"

scrutinee:
    expression - struct_expression

match_arms:
    (match_arm "=>" (expression_without_block "," | expression_with_block ","?))*
        match_arm "=>" expression ","?
match_arm:
    outer_attribute* pattern match_arm_guard?
match_arm_guard:
    "if" expression
```

A `match` expression branches on a pattern.
* a `scrutinee` expression and patterns must have the same type
* all match arms must have also the same type and this type is the type of the
  whole `match` expression

If a `scrutinee` expression is a value expression:
1. it is first evaluated into a temporary location
1. the resulting value is sequentially (left-to-right, down-to-bottom) compared
   to the patterns until a match is found
   * if the pattern has a match guard associated with it and the pattern
     matches, the match guard is evaluated
     * if it is true, we have a match
     * otherwise, we have no match and the match-finding process continues
       (note that in `p1 | p2 if g`, match guard `if g` is applied to both `p1`
       and `p2`)
     * if the match guard refers to the variables bound within the pattern
       1. a shared reference is taken to the part of the `scrutinee` the
          variable matches on (this prevents mutation inside guards)
       1. this shared reference is then used when accessing the variable during
          the match guard evaluation
       1. if guard evaluates to true, the value is moved or copied from the
          `scrutinee` into the variable
   * every binding in each `|` separated pattern must appear in all of the
     patterns in the arm
1. any variables bound by the first matching pattern are assigned to local
   variables in the arm's block
   * variables are scoped to the match guard and the arm's expression
   * the binding mode (copy, move or reference) depends on the pattern
   * every binding of the same name must have the same type and have the same
     binding mode
1. control enters the block
1. the value returned by the block is the value of the `match` expression

If a `scrutinee` expression is a place expression the same logic as before is
applied with these differences:
* a temporary location is not allocated
* a by-value binding may copy or move from the memory location
* lifetime of a match inherits the lifetime of the place expression

Example:
```rust
#[derive(Clone, Debug)]
enum Foo {
    A,
    B(u8),
    C(u8, u8),
}

use crate::Foo::{A, B, C};

fn test_match(obj: &Object) -> (u8, u8) {
    match *obj {
        A => (0, 0),
        B(x @ 1) | B(x @ 2) => (1, x),
        B(x) | C(_, x) if x >= 3 => (2, x),
        B(x) => (3, x),
        C(x, _) => (4, x),
    }
}

fn main() {
    let objs = vec![
        A,        // prints "(0, 0)"
        B(0),     // prints "(3, 0)"
        B(1),     // prints "(1, 1)"
        B(2),     // prints "(1, 2)"
        B(3),     // prints "(2, 3)"
        B(4),     // prints "(2, 4)"
        C(0, 0),  // prints "(4, 0)"
        C(1, 3),  // prints "(2, 3)"
        C(3, 1),  // prints "(4, 3)"
    ];

    for x in objs {
        println!("{:#?}", test_match(&x));
    }
}
```

See [`match` expressions](https://doc.rust-lang.org/reference/expressions/match-expr.html),
[Patterns](https://doc.rust-lang.org/reference/patterns.html),
[Place Expressions and Value Expressions](https://doc.rust-lang.org/reference/expressions.html#place-expressions-and-value-expressions),
and [Binding modes](https://doc.rust-lang.org/reference/patterns.html#binding-modes)
for greater detail.

### Block Expressions

Grammar:
```
block_expression:
    "{" inner_attribute* statements? "}"
statements:
    statement+ expression_without_block?
    expression_without_block

async_block_expression:
    "async" "move"? block_expression

unsafe_block_expression:
    "unsafe" block_expression
```

The value and type of `block_expression` is the value and type of `expression`
if it is present. Otherwise the value and type of `block_expression` is `()`.

See [Block expressions](https://doc.rust-lang.org/reference/expressions/block-expr.html)
for greater detail.

### Atomic Expressions

Grammar:
```
atomic_expression:
    underscore_expression
    literal_expression
    tuple_expression
    array_expression
    struct_expression

underscore_expression:
    "_"

literal_expression:
    char_literal
    string_literal
    raw_string_literal
    byte_literal
    byte_string_literal
    raw_byte_string_literal
    integer_literal
    float_literal
    "true" | "false"

tuple_expression:
    "(" tuple_elements? ")"
tuple_elements:
    (expression ",")+ expression?

array_expression:
    "[" array_elements? "]"
array_elements:
    expression ("," expression)* ","?
    expression ";" expression

struct_expression:
    path_in_expression "{" (struct_expr_fields | struct_base)? "}"
    path_in_expression "(" (expression ("," expression)* "'"?)? ")"
    path_in_expression

struct_expr_fields:
    struct_expr_field ("," struct_expr_field)* ("," struct_base | ","?)
struct_expr_field:
    outer_attribute* (
        identifier |
        (identifier | integer_literal) ":" expression
    )
struct_base:
    ".." expression
```
* for `integer_literal` in `struct_expr_field` hold same restriction as for
  `integer_literal` in `tuple_indexing_expression`

See [_ expressions](https://doc.rust-lang.org/reference/expressions/underscore-expr.html),
[Literal expressions](https://doc.rust-lang.org/reference/expressions/literal-expr.html),
[Tuple expressions](https://doc.rust-lang.org/reference/expressions/tuple-expr.html#tuple-expressions),
[Array expressions](https://doc.rust-lang.org/reference/expressions/array-expr.html#array-expressions),
and [Struct expressions](https://doc.rust-lang.org/reference/expressions/struct-expr.html)
for greater detail.

### Grouped Expressions

Grammar:
```
grouped_expression:
    "(" expression ")"
```

See [Grouped expressions](https://doc.rust-lang.org/reference/expressions/grouped-expr.html)
for greater detail.

## Statements

Grammar:
```
statement:
    ";"
    item
    let_statement
    expression_statement
    macro_invocation_semi

item:
    outer_attribute* vis_item
    macro_item

vis_item:
    visibility? (
        declaration_item
        trait
        implementation
        module
        extern_crate
    )
macro_item:
    macro_invocation_semi
    macro_rules_definition

expression_statement:
    expression_without_block ";"
    expression_with_block ";"?
```

See [Statements](https://doc.rust-lang.org/reference/statements.html),
[Item declarations](https://doc.rust-lang.org/reference/statements.html#item-declarations)
and [Items](https://doc.rust-lang.org/reference/items.html) for greater detail.

## Patterns

Grammar:
```
pattern:
    "|"? pattern_no_top_alt ("|" pattern_no_top_alt)*

pattern_no_top_alt:
    pattern_without_range
    range_pattern

pattern_without_range:
    literal_pattern
    identifier_pattern
    wildcard_pattern
    rest_pattern
    reference_pattern
    struct_pattern
    tuple_struct_pattern
    tuple_pattern
    grouped_pattern
    slice_pattern
    path_pattern
    macro_invocation

literal_pattern:
    "true" | "false"
    char_literal
    byte_literal
    string_literal
    raw_string_literal
    byte_string_literal
    raw_byte_string_literal
    "-"? integer_literal
    "-"? float_literal

identifier_pattern:
    "ref"? "mut"? identifier ("@" pattern_no_top_alt)?

wildcard_pattern:
    "_"

rest_pattern:
    ".."

range_pattern:
    range_inclusive_pattern
    range_from_pattern
    range_to_inclusive_pattern
    obsolete_range_pattern

range_inclusive_pattern:
    range_pattern_bound "..=" range_pattern_bound
range_from_pattern:
    range_pattern_bound ".."
range_to_inclusive_pattern:
    "..=" range_pattern_bound
obsolete_range_pattern:
    range_pattern_bound "..." range_pattern_bound

range_pattern_bound:
    char_literal
    byte_literal
    "-"? integer_literal
    "-"? float_literal
    path_expression

reference_pattern:
    ("&" | "&&") "mut"? pattern_without_range

struct_pattern:
    path_in_expression "{" struct_pattern_elements? "}"

struct_pattern_elements:
    struct_pattern_fields ("," struct_pattern_et_cetera?)?
    struct_pattern_et_cetera

struct_pattern_fields:
    struct_pattern_field ("," struct_pattern_field)*
struct_pattern_field:
    outer_attribute* (
        integer_literal ":" pattern |
        identifier ":" pattern |
        "ref"? "mut"? identifier
    )

struct_pattern_et_cetera:
    outer_attribute* ".."

tuple_struct_pattern:
    path_in_expression "(" tuple_struct_items? ")"
tuple_struct_items:
    pattern ("," pattern)* ","?

tuple_pattern:
    "(" tuple_pattern_items? ")"
tuple_pattern_items:
    pattern ","
    rest_pattern
    pattern ("," pattern)+ ","?

grouped_pattern:
    "(" pattern ")"

slice_pattern:
    "[" slice_pattern_items? "]"
slice_pattern_items:
    pattern ("," pattern)* ","?

path_pattern:
    path_expression
```

See [Patterns](https://doc.rust-lang.org/reference/patterns.html) for greater
detail.

## Traits

Grammar:
```
trait:
    "unsafe"? "trait" identifier generic_params? (":" type_param_bounds?)?
        where_clause? "{" inner_attribute* associated_item* "}"

type_param_bounds:
    type_param_bound ("+" type_param_bound)* "+"?
type_param_bound:
    lifetime
    trait_bound
trait_bound:
    "?"? ("for" generic_params)? type_path
    "(" "?"? ("for" generic_params)? type_path ")"

lifetime_bounds:
    (lifetime "+")* lifetime?
lifetime:
    lifetime_or_label
    "'static"
    "'_"
```

See [Traits](https://doc.rust-lang.org/reference/items/traits.html),
[Trait and lifetime bounds](https://doc.rust-lang.org/reference/trait-bounds.html),
and [Associated Items](https://doc.rust-lang.org/reference/items/associated-items.html)
for greater detail.

## Implementations

Grammar:
```
implementation:
    inherent_impl
    trait_impl

inherent_impl:
    "impl" generic_params? type where_clause?
        "{" inner_attribute* associated_item* "}"
trait_impl:
    "unsafe"? "impl" generic_params? "!"? type_path "for" type where_clause?
        "{" inner_attribute* associated_item* "}"

associated_item:
    outer_attribute* (
        macro_invocation_semi |
        (visibility? (type_alias | constant_item | function))
    )
```

An implementation associates an item definition with a concrete type.
* this happens inside of `impl` block
* multiple `impl` blocks per one implementing type are possible

Inherent implementations:
* can contain associated functions, including methods, and associated constants
* a type can also have multiple inherent implementations
* an implementing type must be defined within the same crate as the original
  type definition

See [Implementations](https://doc.rust-lang.org/reference/items/implementations.html)
for greater detail.

### Associated Functions and Methods

Associated functions are functions associated with a type.

Methods are associated functions with `self` as the first parameter. The type
of `self`, `S`, can be specified, but it undergoes the following restrictions:
* Let `T` be an implementing type and `'a` by an arbitrary lifetime.
* Then `S` is one of `Self` or `P`, where
  * `Self` refers to a type resolving to `T`, such as alias of `T`, `Self`, or
    associated type projections resolving to `T`;
  * `P` is one of `& 'a S`, `& 'a mut S`, `Box<S>`, `Rc<S>`, `Arc<S>`, or
    `Pin<S>`.

When `self` has no type specified, then
* `self` is equivalent to `self: Self`
* `& 'a self` is equivalent to `self: & 'a Self`
* `& 'a mut self` is equivalent to `self: & 'a mut Self`

Explanation on example:
```rust
#[derive(Debug)]
struct FsItem {
    name: String,
    size: usize,
}

impl FsItem {
    fn new() -> Self {
        Self {
            name: String::from(""),
            size: 0usize,
        }
    }

    fn create(name: String, size: usize) -> FsItem {
        FsItem { name, size }
    }

    fn name(&self) -> String {
        String::from(self.name.as_str())
    }

    fn size(&self) -> usize {
        self.size
    }

    fn rename(&mut self, name: String) {
        self.name = name;
    }
}

fn main() {
    let mut fsitem1 = FsItem::new();
    let fsitem2 = FsItem::create(String::from("/etc/fsitem2"), 16);

    println!("{fsitem1:#?}");
    println!("{fsitem2:#?}");

    println!("fsitem1 = {{ {}, {} }}", fsitem1.name(), fsitem1.size());

    fsitem1.rename(String::from("/etc/fsitem1"));

    println!("fsitem1 = {{ {}, {} }}", fsitem1.name(), fsitem1.size());
}
```
* `impl FsItem` block encloses functions and methods definitions associated
  with `struct FsItem`
* `new()` and `create()` are associated functions of `struct FsItem` and as
  such their path must be specified to call them: `FsItem::new()`,
  `FsItem::create()`
* `name()`, `size()`, and `rename()` are methods
  * they are called using [method-call expression](https://doc.rust-lang.org/reference/expressions/method-call-expr.html),
    e.g. `fsitem1.name()`
* `Self` refers to the implementing type, here `struct FsItem`
* `self` refers to the object of implementing type and it is an implicit
  parameter to the method
  * e.g. `x.f(a, b)` translates to `T::f(x, a, b)` where `T` is the type of the
    receiver or trait and `x` matches with `self`
* `mut` before `self` denotes that a method modifies the object referred by
  `self`
* `&` is necessary if the object referred by `self` is borrowed more than once
  * e.g. `fsitem1` is borrowed by `name()` and by `size()`

See [Associated Items](https://doc.rust-lang.org/reference/items/associated-items.html)
for greater detail.

## Generics

Grammar:
```
generic_params:
    "<" ">"
    "<" (generic_param ",")* generic_param ","? ">"
generic_param:
    outer_attribute* (lifetime_param | type_param | const_param)
lifetime_param:
    lifetime_or_label (":" lifetime_bounds)?
type_param:
    identifier (":" type_param_bounds?)? ("=" type)?
const_param:
    "const" identifier ":" type
        ("=" block_expression | identifier | "-"? literal_expression)?

where_clause:
    "where" (where_clause_item ",")* where_clause_item?
where_clause_item:
    lifetime ":" lifetime_bounds
    ("for" generic_params)? type ":" type_param_bounds?
```

See [Generic parameters](https://doc.rust-lang.org/reference/items/generics.html)
for greater detail.

## Macros

Grammar:
```
macro_invocation:
    simple_path "!" delim_token_tree
macro_invocation_semi:
    simple_path "!" "(" token_tree* ")" ";"
    simple_path "!" "[" token_tree* "]" ";"
    simple_path "!" "{" token_tree* "}"

delim_token_tree:
    "(" token_tree* ")"
    "[" token_tree* "]"
    "{" token_tree* "}"
token_tree:
    token - delimiters
    delim_token_tree
```

See [Macros](https://doc.rust-lang.org/reference/macros.html) for greater
detail.

### Selected Macros from the Standard Library

#### `format`

Creates a [`String`](https://doc.rust-lang.org/std/string/struct.String.html)
by the interpolation of format string literal. Example:
```rust
let (x, y) = (1, 2);

format!("x = {x}, y = {y}");     // "x = 1, y = 2"
format!("z = {z}", z = 3);       // "z = 3"
format!("Hello, {}!", "World");  // "Hello, World!"
```

See [`std::format`](https://doc.rust-lang.org/std/macro.format.html) for
greater detail.

##### Format String

A format string is a string containing markers with format specification. A
marker is a string starting with `{` and ending with `}`. During interpolation,
a marker is replaced by the string representation of the corresponding value.
Additional characters between `{` and `}` specify a way of interpolating a
value.

Selected markers:
| **Marker** | **Meaning** |
| ---------- | ----------- |
| `{}` | the value's type must implement [`std::fmt::Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html) trait |
| `{:?}` | the value's type must implement [`std::fmt::Debug`](https://doc.rust-lang.org/std/fmt/trait.Debug.html) trait |
| `{:#?}` | `{:?}` with pretty print flag set |

See [`std::fmt`](https://doc.rust-lang.org/std/fmt/index.html) for greater
detail.

Additional references:
* [`std::fmt::Formatter`](https://doc.rust-lang.org/std/fmt/struct.Formatter.html)
* [`std::fmt::Arguments`](https://doc.rust-lang.org/std/fmt/struct.Arguments.html)
* [`std::fmt::DebugStruct`](https://doc.rust-lang.org/std/fmt/struct.DebugStruct.html)
* [`std::fmt::DebugTuple`](https://doc.rust-lang.org/std/fmt/struct.DebugTuple.html)
* [`std::fmt::DebugList`](https://doc.rust-lang.org/std/fmt/struct.DebugList.html)
* [`std::fmt::DebugSet`](https://doc.rust-lang.org/std/fmt/struct.DebugSet.html)
* [`std::fmt::DebugMap`](https://doc.rust-lang.org/std/fmt/struct.DebugMap.html)

#### `format_args`

Creates [`std::fmt::Arguments`](https://doc.rust-lang.org/std/fmt/struct.Arguments.html)
object containing precompiled format string and its arguments. Example:
```rust
let args = format_args!("{} + {} = {}", 1, 2, 3);

println!("{:?}", args);
```

See [`std::format_args`](https://doc.rust-lang.org/std/macro.format_args.html)
and [`std::fmt::Arguments`](https://doc.rust-lang.org/std/fmt/struct.Arguments.html)
for greater detail.

#### `println`

Prints interpolated format string and the new line character to the standard
output. Examples:
```rust
println!("Hello, World!");
println!("Hello, {}!", "World");
```

See [`std::println`](https://doc.rust-lang.org/std/macro.println.html) for
greater detail.

#### `eprintln`

Like `println` but prints the output to standard error output.

See [`std::eprintln`](https://doc.rust-lang.org/std/macro.eprintln.html) for
greater detail.

#### `dbg`

Prints to the standard error output and returns the value of given expression.
The value is moved. The type of the value must implement
[`std::fmt::Debug`](https://doc.rust-lang.org/std/fmt/trait.Debug.html) trait.
Examples:
```rust
let a = dbg!(2 + 5);  // Prints: [src/main.rs:2] 2 + 5 = 7

#[derive(Debug)]
struct NoCopy(u32);

let a = NoCopy(8);
let _ = dbg!(a);
let _ = dbg!(a);  // Error! (`a` was moved)
```

See [`std::dbg`](https://doc.rust-lang.org/std/macro.dbg.html) for greater
detail.

## Attributes

Grammar:
```
inner_attribute:
    "#" "!" "[" attr "]"

outer_attribute:
    "#" "[" attr "]"

attr:
    simple_path attr_input?
attr_input:
    delim_token_tree
    "=" expression
```

See [Attributes](https://doc.rust-lang.org/reference/attributes.html) for
greater detail.

### Selected Attributes Supported by Rust

#### `derive`

Allows new items to be automatically generated for data structures. Example:
```rust
// Implement `std::fmt::Debug` trait for `Point`
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

let p = Point {x: 1, y: 2};

println!("{p:#?}");  // Pretty print `p`
```

See [Derive](https://doc.rust-lang.org/reference/attributes/derive.html) for
greater detail.

#### `repr`

Specifies the layout for user-defined composite types (structs, enums, unions).
Possible representations are:
* `Rust` (default)
* `C`
* primitive
* `transparent`

Example:
```rust
// Uses default (Rust) representation
struct Foo {
    bar: isize,
    baz: u8,
}

// C representation
#[repr(C)]
struct Pixel {
    x: u32,
    y: u32,
}
```

See [Representations](https://doc.rust-lang.org/reference/type-layout.html#representations)
for greater detail.

## Rust's Module System

Grammar:
```
crate:
    utf8bom? shebang? inner_attribute* item*

module:
    "unsafe"? "mod" identifier (";" | "{" inner_attribute* item* "}")

extern_crate:
    "extern" "crate" crate_ref as_clause? ";"

crate_ref:
    identifier
    "self"
as_clause:
    "as" (identifier | "_")
```

A module system of Rust consists of these features: packages, crates, modules
and `use`, and paths.

### Packages

A package is a feature of Cargo that allows to build, test, and share crates.
It is a bundle of one or more crates, specifically:
* a package must contain at least one crate
* a package can contain as many binary crates as needed, but at most only one
  library crate

New package can be created using `cargo new` command. The layout of a package
directory:
```
foopkg/
    Cargo.toml
    /src
        lib.rs
        main.rs
        /bin
            prog1.rs
            prog2.rs
            /prog3
                main.rs
                foomod.rs
    /benches
        bench1.rs
        /bench2
            main.rs
            benchmod.rs
    /examples
        example1.rs
        /example2
            main.rs
            barmod.rs
    /tests
        test1.rs
        /test2
            main.rs
            testmod.rs
```
* `Cargo.toml` is a manifest that tells `cargo` how to build crates in a
  package
  * defaults can be override here
* the `src` directory contains source code of package crates
  * the default library crate is `lib.rs` (`cargo` produces a library with the
    same name as the package)
  * the default executable is `main.rs` (`cargo` produces an executable with
    the same name as the package)
  * the `bin` directory is a place for other executables
* the `benches` directory contains benchmarks
* the `examples` directory contains examples
* the `tests` directory contains integration tests
* if a binary executable, bench, example, or test consists of multiple source
  files (in our case `src/bin/prog3`, `benches/bench2`, `examples/example2` and
  `tests/test2`), put them into a directory:
  * `main.rs` fine determine the crate root
  * other files are considered as modules
  * the name of a directory becomes the name of the executable

### Crates

A *crate* is a smallest compilation unit, a tree of modules. There are two
types of crates:
1. A *binary crate*, which when compiles produces an executable. A binary crate
   must contain the `main` function serving as its entry point. The `main`
   function must fulfill these requirements:
   * must take no arguments
   * must not declare any trait or lifetime bounds
   * must not have any where clauses
   * its return type must implement the `Termination` trait (types from the
     standard library implementing the `Termination` trait are `()`, `!`,
     `Infallible`, `ExitCode`, `Result<T, E> where T: Termination, E: Debug`)
1. A *library crate* is a crate without `main` function. When compiled, it
   produces a library (by default statically linked) that exposes publicly
   visible items as a part of its API.

Attributes that can be applied at the crate level:
* `#![no_main]` &ndash; emit no `main` for an executable binary
* `#![crate_name = "my_crate"]` &ndash; set the name of a crate to `my_crate`
  * hyphens are disallowed in crate names
    * if the crate name is resolved from the package name, Cargo will replace
      all hyphens with underscores
* `#![no_std]`
  * prevents `std` from being added to the extern prelude
    * note that this does not prevent `std` from being linked in using
      `extern crate std;`
  * affects which module is used to make up the standard library prelude (see
    [Preludes](#preludes))
  * injects the `core` crate into the crate root instead of `std`
  * pulls in all macros exported from `core` in the `macro_use` prelude

A source file from which the Rust compiler starts and makes up the root module
of the crate is called the *crate root*.
* the contents of the source file forms a module named `crate` which sits at
  the root of the crate's module structure, known as the module tree

The root module of a crate is the top-level of the crate's module tree.
* the root module is anonymous (from the point of view of paths within the
  module) and it is accessible via `crate` path

Any item within a crate has a canonical module path denoting its location
within the crate's module tree.

Handling the standard library:
* automatically included in the crate root module
* the `std` crate is added to the root
* implicit `macro_use` attribute pulls in all macros exported in `std` into the
  `macro_use` prelude
* `core` and `std` are added to the extern prelude

#### Extern Crates

An extern crate declaration specifies a dependency on an external crate:
* the external crate is bound into the declaring scope as the identifier
  provided in the declaration
* if the declaration appears in the crate root, the crate name is also added to
  the extern prelude so it will be automatically available in scope in all
  modules
* the `as` clause can be used to bind the imported crate to a different name
* the `self` crate may be imported
  * a binding to the current crate is created
  * `as` clause is mandatory in this case
* `extern crate foo as _;` specifies a dependency on an external crate without
  binding its name in scope
  * useful for crates that only need to be linked
* `no_link` attribute prevents a crate to be linked
  * useful to load a crate to only access its macros

During compile time and runtime linkage:
1. If the `extern crate` declaration has a `crate_name` attribute, let NAME be
   the value of this attribute. Otherwise, let NAME be from
   `extern crate NAME`.
1. From the compiler's library path and NAME resolve SONAME.
1. At runtime, pass a runtime linkage requirement SONAME have to the linker for
   loading.

Example:
```rust
extern crate pcre;
extern crate std;             // Same as `extern crate std as std;`
extern crate std as ruststd;  // Linking to `std` under `ruststd`
```

### Modules

A module is a container of zero or more items.
* `mod foo { ... }` introduces a new named module, `foo`, into the tree of
  modules making up a crate.
* Modules can nest arbitrarily.
* Modules and types share the same name space.
* In `unsafe mod foo { ... }`, `unsafe` is rejected at a semantic level (that
  is, it has use only when feed into a macro).

Every source file is a module, but not every module needs its own source file.

A module without body, i.e. `mod foo;` statement, is loaded from an external
file.
* The path to the file mirrors the logical module path, unless the module does
  have a `path` attribute.
* Ancestor module path components are directories.
* The module's contents are in a file with the name of the module plus the
  `.rs` extension.
  * If the file name containing the module's contents is `mod.rs`, the module
    file name is the name of the directory containing this `mod.rs` file.
  * For `crate::foo`, it is not allowed to have both `foo.rs` and `foo/mod.rs`.
  * Since `rustc` 1.30, using `mod.rs` files is considered obsolete.
* If a `path` attribute is given, it specifies the location of an external file
  with the module's content to be loaded. There are three ways of handling
  `path` attributes:
  1. A `path` attribute is not inside inline module block, e.g.
     ```rust
     // File `src/a/b.rs` (1) or `src/a/mod.rs` (2):
     #[path = "foo.rs"]
     mod c;
     // The path to the `c` module is:
     //   - `crate::a::b::c` in case of (1)
     //   - `crate::a::c` in case of (2)
     ```
     * the file path is relative to the directory the source file is located,
       e.g. the external file with the `c` module's content is located at
       `src/a/foo.rs`
  1. A `path` attribute is inside inline module block and the source file where
     the `path` attribute is located is the root module (like `lib.rs` or
     `main.rs`) or `mod.rs`, e.g.
     ```rust
     // File `src/lib.rs` (1) or `src/a/mod.rs` (2):
     mod inline {
         #[path = "other.rs"]
         mod inner;
     }
     // The `other.rs` file will be searched at:
     //   - `src/inline/other.rs` in case of (1)
     //       * the path to the `inner` module is `crate::inline::inner`
     //   - `src/a/inline/other.rs` in case of (2)
     //       * the path to the `inner` module is `crate::a::inline::inner`
     ```
     * the file path is relative to the directory of root module file or
       `mod.rs` file including the inline module components as directories
  1. A `path` attribute is inside inline module block and the source file where
     the `path` attribute is located is neither the root module nor `mod.rs`,
     e.g.
     ```rust
     // File `src/a/b.rs`:
     mod inline {
         #[path = "other.rs"]
         mod inner;
     }
     // The `other.rs` file will be searched at `src/a/b/inline/other.rs`.
     // The path to the `inner` module is `crate::a::b::inline::inner`.
     ```
     * the file path is relative to the directory of the source file with the
       `path` attribute, including the directory with the same name as the
       module that corresponds to the source file and the inline module
       components as directories
     * rules can be combined:
       ```rust
       // File `src/main.rs`:
       mod a;

       fn main() {
           crate::a::b::thread::thread_local::join();
       }

       // File `src/a.rs`:
       pub mod b;

       // File `src/a/b.rs`:
       #[path = "threads"]
       pub mod thread {
           #[path = "tls.rs"]
           pub mod thread_local;
       }
       // The `tls.rs` file will be searched at `src/a/threads/tls.rs`.
       // The path to the `thread_local` module is `crate::a::b::thread::thread_local`.

       // File `src/a/threads/tls.rs`:
       pub fn join() {}
       ```

A module can have attributes.
* attributes in a source file applies to the containing module
* attributes applied to the anonymous root module apply also to the crate as a
  whole
* the built-in attributes that have meaning on modules: `cfg`, `deprecated`,
  `doc`, `allow`, `warn`, `deny`, `forbid`, `path`, and `no_implicit_prelude`

Example:
```rust
// File `src/lib.rs`:
//   - this is the crate root
//   - the path accessing this module is `crate` (this module is the root
//     module)
// This module is loaded from the external file, `src/util.rs`. The effect is
// the same as `mod util { /* contents of src/util.rs */ }`:
//   - the path to the `util` module is `crate::util`
mod util;
// Like `util` module. The path to the `lexer` module is `crate::lexer`:
mod lexer;

// File `src/util.rs`:
//   - the path accessing this module is `crate::util`
//   - the path to the just loaded `config` module is `crate::util::config`
mod config;

// File `src/util/config.rs`:
//   - the path accessing this module is `crate::util::config`
struct Config {
    pub (crate) vmem_limit: usize,
    pub (crate) stack_limit: usize,
    pub (crate) nfiles_limit: usize,
}

// File `src/lexer/mod.rs`:
//   - the path accessing this module is `crate::lexer`
//   - the path to the `token` module is `crate::lexer::token`
mod token;

//   - the path to the `scan` function is `crate::lexer::scan`
pub fun scan() -> token::Token {
    token::Token(42)
}

// File `src/lexer/token.rs`:
//   - the path accessing this module is `crate::lexer::token`
//   - the path to the `Token` struct is `crate::lexer::token::Token`
struct Token(u8);
```

See [Package Layout](https://doc.rust-lang.org/cargo/guide/project-layout.html),
[The Manifest Format](https://doc.rust-lang.org/cargo/reference/manifest.html),
[Cargo Targets](https://doc.rust-lang.org/cargo/reference/cargo-targets.html),
[Crates and source files](https://doc.rust-lang.org/reference/crates-and-source-files.html),
[Trait and lifetime bounds](https://doc.rust-lang.org/reference/trait-bounds.html),
[Where clauses](https://doc.rust-lang.org/reference/items/generics.html#where-clauses),
[Trait std::process::Termination](https://doc.rust-lang.org/std/process/trait.Termination.html),
[Never type](https://doc.rust-lang.org/reference/types/never.html),
[Enum std::convert::Infallible](https://doc.rust-lang.org/std/convert/enum.Infallible.html),
[Struct std::process::ExitCode](https://doc.rust-lang.org/std/process/struct.ExitCode.html),
[Modules](https://doc.rust-lang.org/reference/items/modules.html),
[Conditional compilation](https://doc.rust-lang.org/reference/conditional-compilation.html),
[Diagnostic attributes](https://doc.rust-lang.org/reference/attributes/diagnostics.html),
[The `#[doc]` attribute](https://doc.rust-lang.org/rustdoc/write-documentation/the-doc-attribute.html),
[Preludes](https://doc.rust-lang.org/reference/names/preludes.html),
[Paths](https://doc.rust-lang.org/reference/paths.html),
[Extern crate declarations](https://doc.rust-lang.org/reference/items/extern-crates.html),
[Macros By Example](https://doc.rust-lang.org/reference/macros-by-example.html),
and [Attributes](https://doc.rust-lang.org/reference/attributes.html) for
greater detail.

### Preludes

A *prelude* is a collection of names that are automatically brought into scope
of every module in a crate.
* these prelude names are not part of the module itself
  * they are implicitly queried during name resolution
  * e.g. `Box` is valid but `self::Box` is not because `Box` is not a member of
    the current module
* the `no_implicit_prelude` attribute
  * applicable at the crate level (root module) or on a module
  * indicates that it should not automatically bring the standard library
    prelude, extern prelude, `macro_use` prelude, or tool prelude into scope
    for that module or any of its descendants

There are several kinds of preludes:
* **Standard library prelude**
  * each crate has it
  * consists of the names from a single standard library module
    * if `no_std` attribute is applied, this module is
      `core::prelude::rust_EDITION`
    * otherwise, this module is `std::prelude::rust_EDITION`
* **Extern prelude**
  * consists of these crates:
    * external crates imported with `extern crate` in the root module
      * if `extern crate foo as bar` is used, the external crate `foo` is added
        to the extern prelude under the name `bar`
    * external crates provided to the compiler (`rustc --extern`)
    * the `core` crate
    * the `std` crate as long as the `no_std` attribute is not specified in the
      crate root
  * Cargo does bring in `proc_macro` to the extern prelude for proc-macro
    crates only
* **Language prelude**
  * is always in scope
  * includes names of types and attributes that are built-in to the language
* **`macro_use` prelude**
  * macros from external crates, imported by the `macro_use` attribute applied
    to an `extern crate`, are included into the `macro_use` prelude
* **Tool prelude**
  * includes tool names for external tools in the type name space
  * in the tool prelude, each tool resides in its own name space
  * when a tool attribute is recognized by the compiler (i.e. when a tool is
    found in the tool prelude), the compiler accepts it without any warning
    * `rustc` currently recognizes the tools `rustfmt` and `clippy`
  * when a tool attribute is recognized by a tool, the tool is responsible for
    further processing and interpretation of the attribute

See [Preludes](https://doc.rust-lang.org/reference/names/preludes.html),
[Extern crate declarations](https://doc.rust-lang.org/reference/items/extern-crates.html),
[Use declarations](https://doc.rust-lang.org/reference/items/use-declarations.html),
[Macros By Example](https://doc.rust-lang.org/reference/macros-by-example.html),
[Namespaces](https://doc.rust-lang.org/reference/names/namespaces.html),
and [Attributes](https://doc.rust-lang.org/reference/attributes.html) for
greater detail.

### Visibility

Grammar:
```
visibility:
    "pub"
    "pub" "(" "crate" ")"
    "pub" "(" "self" ")"
    "pub" "(" "super" ")"
    "pub" "(" "in" simple_path ")"
```
* everything in Rust is private by default except
  * associated item in a `pub` trait
  * enum variants in a `pub` enum
  * items marked with `pub`
* an item is accessible in two cases:
  1. a public (`pub`) item can be accessed externally from some module `m` only
     if all the item's ancestors from `m` can also be accessed externally
  1. a private item can be accessed by the current module and its descendants
* availability of public items can be further restricted:
  * `pub(crate)` makes an item visible within the current crate
  * `pub(self)` is equivalent to `pub(in self)` or not using `pub` at all
  * `pub(super)` is equivalent to `pub(in super)`
  * `pub(in path)` makes an item visible within the provided `path`
    * `path` must be an ancestor module of the item whose visibility is being
      declared
    * `pub(in self)` makes an item visible to the current module
    * `pub(in super)` makes an item visible to the parent module
* `pub` together with `use` can be used for re-exporting items
  * `use` brings items to the current scope as usual
  * `pub` makes them public

Examples:
* accessibility of items demonstration:
  ```rust
  // `foo` is private, so no external crate can access it (rule #1).
  // Any module in this crate can access `foo`'s public interface since `foo`
  // lays in the root module of this crate (rule #2).
  mod foo {
      // Can be used by anything in the current crate (rule #1).
      pub fn f() {}

      // Can be accessed only by this module and its descendants (rule #2).
      fn g() {}
  }

  // Public to the root module, available to external crates (rule #1).
  pub fn h() {}

  // Public to the root module, available to external crates (rule #1).
  pub mod bar {
      // This is legal since `bar` is a descendant of the root module (rule #2).
      use crate::foo;

      // Available to external crates (rule #1).
      pub fn x() {
          // This is legal (rule #2).
          foo::f();
      }

      // Available only to this module and its descendants (rule #2).
      fn y() {}

      #[cfg(test)]
      mod test {
          #[test]
          fn test_y() {
              // This is legal since this module is a descendant of the module
              // `bar` (rule #2).
              super::y();
          }
      }
  }
  ```
* accessibility with restrictions:
  ```rust
  pub mod a {
      pub mod b {
          // Visible within `a`.
          pub (in crate::a) fn fa() {}

          // Visible to entire crate.
          pub (crate) fn fb() {}

          // Visible within `a`.
          pub (super) fn fc() {
              // Visible since we are in the same module.
              fd();
          }

          // Visible only within `b` (same as leaving `fd` private).
          pub (self) fn fd() {}
      }

      pub fn ga() {
          b::fa();
          b::fb();
          b::fc();
          //b::fd();  // Error
      }
  }

  fn ha() {
      // Still visible (we are in the same crate).
      a::b::fb();
      //a::b::fc();  // Error (we are outside of `a`)
      //a::b::fa();  // Error (we are outside of `a`)
      a::ga();
  }

  fn main() { ha() }
  ```
* re-importing/re-exporting names into the current scope:
  ```rust
  // From the standard library: brings `Option`, `None`, and `Some` to the
  // current scope and makes them publicly visible
  pub use crate::option::Option;
  pub use crate::option::Option::None;
  pub use crate::option::Option::Some;
  ```
* another example of re-exporting:
  ```rust
  pub use self::a::b;

  mod a {
      pub mod b {
          pub fn f() {}
      }
  }

  b::f();       // Valid (`b` is public in this scope, `f` is public in `b`)
  //a::b::f();  // Invalid (`a` is private)
  ```

See [Visibility and Privacy](https://doc.rust-lang.org/reference/visibility-and-privacy.html)
and [Use declarations](https://doc.rust-lang.org/reference/items/use-declarations.html)
for greater detail.

### Paths

A path refers to:
* item or variable if it has only one component
* item if it has more than one component

Path qualifiers:
* `::`
  * paths starting with `::` are considered to be *global paths*
  * segments of the path start being resolved from a place which differs based
    on edition
    * in the 2015 Edition, identifiers resolve from the crate root, i.e.
      `::foo` in the 2015 Edition is the same as `crate::foo` in the 2018 and
      newer Edition
    * in the 2018 and newer Edition, identifiers resolve from crates in the
      extern prelude, i.e. they must be followed by the name of a crate, e.g.
      `::core` resolves to the `core` crate which is always added to the extern
      prelude
      * `::` is necessary if a name from to root module collide with the name
        from the extern prelude, e.g.
        ```rust
        //use std::fs;                // Error, ambiguity
        use ::std::fs;                // Imports `fs` from the standard `std` crate
        use self::std::fs as std_fs;  // Imports `fs` from the `std` module below

        mod std {
            pub mod fs {}
        }
        ```
* `self`
  * resolves to the current module
  * a single `self` in a method body resolves to the method's `self` parameter
* `Self`
  * refers to the implementing type within traits and implementations
* `super`
  * resolves to the parent module
* `crate`
  * refers to the root module of the current crate
* `$crate`
  * allowed only inside macro transcribers
  * refers to the root module of the crate where the macro is defined

Canonical paths:
* only items defined in a module or implementation have a canonical path
* a canonical path reflects the location of the item relatively to the root
  module of its crate, e.g.
  ```rust
  // File `src/lib.rs`:
  mod foo {
      // Canonical path to this struct is `crate::foo::Foo`
      pub struct Foo;
  }
  ```
  * it is meaningful only within a given crate (there is no global namespace
    across crates)
* a canonical path consists of a *path prefix* and an item appended to it
  * for modules, the path prefix is the canonical path to that module, e.g.
    ```rust
    // File `src/lib.rs`:
    mod foo {        // `crate::foo` (the canonical path to the module `foo`)
        struct Bar;  // `crate::foo::Bar` (the canonical path to the `struct Bar`)
    }
    ```
  * for bare implementations, the path prefix has the form `<P>`, where `P` is
    the canonical path of the item being implemented, e.g.
    ```rust
    // File `src/lib.rs`:
    mod foo {
        struct Bar;

        impl Bar {
            // The canonical path to the method `f` implemented here is
            // `<crate::foo::Bar>::f`.
            fn f(&self) {}
        }
    }
    ```
  * for trait implementations, the path prefix has the form `<P as T>`, where
    `P` is the canonical path of the item being implemented and `T` is the
    canonical path to the trait, e.g.
    ```rust
    // File `src/lib.rs`:
    mod a {
        struct S;

        trait T {
            fn f(&self);
        }

        impl T for S {
            // The canonical path to the method `f` implemented here is
            // `<crate::a::S as crate::a::T>::f`.
            fn f(&self) {}
        }

        impl S {
            fn g(&self) {}
        }
    }
    ```
* items which do not have canonical paths:
  * implementations
  * use declarations
  * items defined in block expressions
  * items defined in a module that does not have a canonical path
  * associated items defined in an implementation that refers to an item
    without a canonical path

#### Simple Paths

Grammar:
```
simple_path:
    "::"? simple_path_segment ("::" simple_path_segment)*
simple_path_segment:
    identifier | "super" | "self" | "crate" | "$crate"
```
* used in visibility markers, attributes, macros and, `use` items

#### Paths in Expressions

Grammar:
```
path_in_expression:
    "::"? path_expr_segment ("::" path_expr_segment)*
path_expr_segment:
    path_ident_segment ("::" generic_args)?
path_ident_segment:
    identifier | "super" | "self" | "Self" | "crate" | "$crate"
generic_args:
    "<" ((generic_arg ",")* generic_arg ","?)? ">"
generic_arg:
    lifetime
    type
    block_expression
    "-"? literal_expression
    simple_path_segment
    identifier "=" type
```
* allow for paths with generic arguments to be specified
* used in expressions and patterns
* *turbofish* (`::<`) syntax:
  * `Vec<u8>::with_capacity(1024)` &ndash; wrong (ambiguity with `<`)
  * `Vec::<u8>::with_capacity(1024)` &ndash; correct
* the order of generic arguments is restricted to:
  1. lifetime arguments, then
  1. type arguments, then
  1. const arguments, then
  1. equality constraints

#### Qualified Paths

Grammar:
```
qualified_path_in_expression:
    qualified_path_type ("::" path_expr_segment)+

qualified_path_type:
    "<" type ("as" type_path)? ">"

qualified_path_in_type:
    qualified_path_type ("::" type_path_segment)+
```
* allow paths for trait implementations to be unambiguous
* allow canonical paths to be specified
* example:
  ```rust
  struct S;

  // Implementation of `struct S`:
  impl S {
      fn f() {}
  }

  trait T1 {
      fn f() {}
  }

  // Implementation of `trait T1` for `struct S`:
  impl T1 for S {}

  trait T2 {
      fn f() {}
  }

  // Implementation of `trait T2` for `struct S`:
  impl T2 for S {}

  // Which `f` is called? `S` implements both `T1` and `T2` so we use type cast
  // to resolve ambiguity:
  S::f();          // `S::f` is called
  <S as T1>::f();  // `T1::f` is called
  <S as T2>::f();  // `T2::f` is called
  ```

#### Type Paths

Grammar:
```
type_path:
    "::"? type_path_segment ("::" type_path_segment)*
type_path_segment:
    path_ident_segment ("::"? (generic_args | type_path_fn))?
type_path_fn:
    "(" (type ("," type)* ","?)? ")" ("->" type)?
```
* used within type definitions, trait bounds, type parameter bounds, and
  qualified paths
* turbofish notation (`::<`) is not required here as there is no danger of
  ambiguity like in expressions

See [Paths](https://doc.rust-lang.org/reference/paths.html),
[Visibility and Privacy](https://doc.rust-lang.org/reference/visibility-and-privacy.html),
[Attributes](https://doc.rust-lang.org/reference/attributes.html),
[Macros By Example](https://doc.rust-lang.org/reference/macros-by-example.html),
[Use declarations](https://doc.rust-lang.org/reference/items/use-declarations.html),
[Expressions](https://doc.rust-lang.org/reference/expressions.html),
[Patterns](https://doc.rust-lang.org/reference/patterns.html),
[Traits](https://doc.rust-lang.org/reference/items/traits.html),
[Implementations](https://doc.rust-lang.org/reference/items/implementations.html),
and [Preludes](https://doc.rust-lang.org/reference/names/preludes.html) for
greater detail.

## Libraries (Crates) and Tools

* [`aho-corasick` - fast multiple substring searching](https://crates.io/crates/aho-corasick) [[doc](https://docs.rs/aho-corasick/latest/aho_corasick/)] [[repo](https://github.com/BurntSushi/aho-corasick)]
* [`anyhow` - flexible concrete error type built on `std::error::Error`](https://crates.io/crates/anyhow) [[doc](https://docs.rs/anyhow/latest/anyhow/)] [[repo](https://github.com/dtolnay/anyhow)]
* [`ariadne` - a fancy diagnostics and reporting](https://crates.io/crates/ariadne) [[doc](https://docs.rs/ariadne/latest/ariadne/)] [[repo](https://github.com/zesterer/ariadne)]
* [`atty` - is it a tty?](https://crates.io/crates/atty) [[doc](https://docs.rs/atty/latest/atty/)] [[repo](https://github.com/softprops/atty)]
* [`bat` - a `cat(1)` clone with wings](https://crates.io/crates/bat) [[doc](https://docs.rs/bat/latest/bat/)] [[repo](https://github.com/sharkdp/bat)]
* [`cargo` - package manager for Rust](https://crates.io/crates/cargo) [[doc](https://docs.rs/cargo/latest/cargo/)] [[repo](https://github.com/rust-lang/cargo)]
* [`cargo-binutils` - proxy for LLVM tools](https://crates.io/crates/cargo-binutils) [[repo](https://github.com/rust-embedded/cargo-binutils)]
* [`chumsky` - a parser library for humans with powerful error recovery](https://crates.io/crates/chumsky) [[doc](https://docs.rs/chumsky/latest/chumsky/)] [[repo](https://github.com/zesterer/chumsky)]
* [`clap` - command line argument parser for Rust](https://crates.io/crates/clap) [[doc](https://docs.rs/clap/latest/clap/)] [[repo](https://github.com/clap-rs/clap)]
* [`colored` - coloring terminal](https://crates.io/crates/colored) [[doc](https://docs.rs/colored/latest/colored/)] [[repo](https://github.com/colored-rs/colored)]
* [`crossbeam` - tools for concurrent programming](https://crates.io/crates/crossbeam) [[doc](https://docs.rs/crossbeam/latest/crossbeam/)] [[repo](https://github.com/crossbeam-rs/crossbeam)]
* [`ctor` - `__attribute__((constructor))` for Rust](https://crates.io/crates/ctor) [[doc](https://docs.rs/ctor/latest/ctor/)] [[repo](https://github.com/mmastrac/rust-ctor)]
* [`cve-rs` - blazingly fast memory vulnerabilities](https://crates.io/crates/cve-rs) [[doc](https://docs.rs/cve-rs/latest/cve_rs/)] [[repo](https://github.com/Speykious/cve-rs)]
* [`duct` - a library for running child processes](https://crates.io/crates/duct) [[doc](https://docs.rs/duct/latest/duct/)] [[repo](https://github.com/oconnor663/duct.rs)]
* [`enumset` - a library for creating compact sets of enums](https://crates.io/crates/enumset) [[doc](https://docs.rs/enumset/latest/enumset/)] [[repo](https://github.com/Lymia/enumset)]
* [`env_logger` - a simple logger that can be configured via environment variables](https://crates.io/crates/env_logger) [[doc](https://docs.rs/env_logger/latest/env_logger/)] [[repo](https://github.com/rust-cli/env_logger)]
  * `RUST_LOG=[target][=][level][,...]` controls logging, examples (considering
    a Rust application `log_demo`):
    * `RUST_LOG=log_demo` turns on all logging for `log_demo`
    * `RUST_LOG=none` turns off all logging (`none` is unknown target)
    * `RUST_LOG=OFF` turns off all logging (`off` is pseudo level made for this
      purpose)
    * `RUST_LOG=info` turns on all info logging
    * `RUST_LOG=log_demo=debug` turns on all debug logging for `log_demo`
    * `RUST_LOG=off,log_demo::foo=info` turn off all logging but keep all info
      logging for `log_demo::foo`
    * `RUST_LOG=info,log_demo::foo=off` turn on all info logging but turn off
      all logging for `log_demo::foo`
* [`glob` - matching file paths against Unix shell style patterns](https://crates.io/crates/glob) [[doc](https://docs.rs/glob/latest/glob/)] [[repo](https://github.com/rust-lang/glob)]
* [`globset` - cross platform single glob and glob set matching](https://crates.io/crates/globset) [[doc](https://docs.rs/globset/latest/globset/)] [[repo](https://github.com/BurntSushi/ripgrep)]
* [`grcov` - Rust tool to collect and aggregate code coverage data for multiple source files](https://crates.io/crates/grcov) [[doc](https://docs.rs/grcov/latest/grcov/)] [[repo](https://github.com/mozilla/grcov)]
* [`hyperfine` - a command-line benchmarking tool](https://crates.io/crates/hyperfine) [[doc](https://docs.rs/crate/hyperfine/latest)] [[repo](https://github.com/sharkdp/hyperfine)]
* [`image` - an image processing library](https://crates.io/crates/image) [[doc](https://docs.rs/image/latest/image/)] [[repo](https://github.com/image-rs/image)]
* [`lazy_static` - a macro for declaring lazily evaluated statics](https://crates.io/crates/lazy_static) [[doc](https://docs.rs/lazy_static/latest/lazy_static/)] [[repo](https://github.com/rust-lang-nursery/lazy-static.rs)]
* [`libc` - raw FFI bindings to platforms' system libraries](https://crates.io/crates/libc) [[doc](https://docs.rs/libc/latest/libc/)] [[repo](https://github.com/rust-lang/libc)]
* [`log` - a lightweight logging facade for Rust](https://crates.io/crates/log) [[doc](https://docs.rs/log/latest/log/)] [[repo](https://github.com/rust-lang/log)]
* [`memchr` - heavily optimized routines for string search primitives](https://crates.io/crates/memchr) [[doc](https://docs.rs/memchr/latest/memchr/)] [[repo](https://github.com/BurntSushi/memchr)]
* [`num` - numeric types and traits (bigint, complex, rational and more)](https://crates.io/crates/num) [[doc](https://docs.rs/num/latest/num/)] [[repo](https://github.com/rust-num/num)]
* [`num_cpus` - get the number of CPUs on a machine](https://crates.io/crates/num_cpus) [[doc](https://docs.rs/num_cpus/latest/num_cpus/)] [[repo](https://github.com/seanmonstar/num_cpus)]
* [`proc-macro2` - a substitute implementation of `proc_macro` API](https://crates.io/crates/proc-macro2) [[doc]](https://docs.rs/proc-macro2/latest/proc_macro2/) [[repo](https://github.com/dtolnay/proc-macro2)]
* [`quote` - quasi-quoting](https://crates.io/crates/quote) [[doc](https://docs.rs/quote/latest/quote/)] [[repo](https://github.com/dtolnay/quote)]
* [`rand` - random number generators](https://crates.io/crates/rand) [[doc](https://docs.rs/rand/latest/rand/)] [[repo](https://github.com/rust-random/rand)]
* [`regex` - regular expressions](https://crates.io/crates/regex) [[doc](https://docs.rs/regex/latest/regex/)] [[repo](https://github.com/rust-lang/regex)]
* [`rhai` - embedded scripting for Rust](https://crates.io/crates/rhai) [[home](https://rhai.rs/)] [[book](https://rhai.rs/book/)] [[doc](https://docs.rs/rhai/latest/rhai/)] [[repo](https://github.com/rhaiscript/rhai)]
* [`rustc_version` - a library for querying the version of a `rustc` compiler](https://crates.io/crates/rustc_version) [[doc](https://docs.rs/rustc_version/latest/rustc_version/)] [[repo](https://github.com/djc/rustc-version-rs)]
* [`serde` - a generic serialization/deserialization framework](https://crates.io/crates/serde) [[home](https://serde.rs/)] [[doc](https://docs.rs/serde/latest/serde/)] [[repo](https://github.com/serde-rs/serde)]
* [`std` - the Rust standard library](https://doc.rust-lang.org/std/index.html)
  * [`std::boxed` - the `Box<T>` type for heap allocation](https://doc.rust-lang.org/std/boxed/index.html)
    * [`std::boxed::Box` - a pointer type that uniquely owns a heap allocation of type `T`](https://doc.rust-lang.org/std/boxed/struct.Box.html)
  * [`std::collections` - collection types](https://doc.rust-lang.org/std/collections/index.html)
    * [`std::collections::HashSet` - a hash set](https://doc.rust-lang.org/std/collections/struct.HashSet.html)
  * [`std::convert` - traits for conversions between types](https://doc.rust-lang.org/std/convert/index.html)
    * [`std::convert::AsRef` - used to do a cheap reference-to-reference conversion](https://doc.rust-lang.org/std/convert/trait.AsRef.html)
    * [`std::convert::From` - used to do value-to-value conversions while consuming the input value](https://doc.rust-lang.org/std/convert/trait.From.html)
    * [`std::convert::Into` - a value-to-value conversion that consumes the input value](https://doc.rust-lang.org/std/convert/trait.Into.html)
    * [`std::convert::TryFrom` - simple and safe type conversions that may fail in a controlled way](https://doc.rust-lang.org/std/convert/trait.TryFrom.html)
  * [`std::env` - inspection and manipulation of the process’ environment](https://doc.rust-lang.org/std/env/index.html)
    * [`std::env::join_paths` - joins a collection of `Path`s appropriately for the `PATH` environment variable](https://doc.rust-lang.org/std/env/fn.join_paths.html)
  * [`std::error` - interfaces for working with errors](https://doc.rust-lang.org/std/error/index.html)
    * [`std::error::Error` - a trait representing the basic expectations for error values](https://doc.rust-lang.org/std/error/trait.Error.html)
  * [`std::ffi` - utilities related to FFI bindings](https://doc.rust-lang.org/std/ffi/index.html)
    * [`std::ffi::OsStr` - borrowed reference to an OS string](https://doc.rust-lang.org/std/ffi/struct.OsStr.html)
    * [`std::ffi::OsString` - owned mutable platform-native string](https://doc.rust-lang.org/std/ffi/struct.OsString.html)
  * [`std::fmt` - utilities for formatting and printing `String`s](https://doc.rust-lang.org/std/fmt/)
  * [`std::fs` - file system manipulation operations](https://doc.rust-lang.org/std/fs/index.html)
    * [`std::fs::FileType` - a type of file with accessors for each file type](https://doc.rust-lang.org/nightly/std/fs/struct.FileType.html)
  * [`std::io` - the I/O module](https://doc.rust-lang.org/std/io/index.html)
    * [`std::io::Result` - a specialized `Result` type for I/O operations](https://doc.rust-lang.org/std/io/type.Result.html)
    * [`std::io::Stdin` - a handle to the standard input stream of a process](https://doc.rust-lang.org/std/io/struct.Stdin.html)
  * [`std::iter` - composable external iteration](https://doc.rust-lang.org/std/iter/index.html)
    * [`std::iter::Extend` - extend a collection with the contents of an iterator](https://doc.rust-lang.org/std/iter/trait.Extend.html)
    * [`std::iter::IntoIterator` - conversion into an `Iterator`](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html)
    * [`std::iter::Iterator` - a trait for dealing with iterators](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
  * [`std::mem` - basic functions for dealing with memory](https://doc.rust-lang.org/std/mem/index.html)
  * [`std::net` - networking primitives for TCP/UDP communication](https://doc.rust-lang.org/std/net/index.html)
  * [`std::ops` - overloadable operators](https://doc.rust-lang.org/std/ops/index.html)
    * [`std::ops::Drop` - custom code within a destructor](https://doc.rust-lang.org/std/ops/trait.Drop.html)
  * [`std::option` - optional values](https://doc.rust-lang.org/std/option/index.html)
    * [`std::option::Option` - the `Option` type](https://doc.rust-lang.org/std/option/enum.Option.html)
  * [`std::path` - cross-platform path manipulation](https://doc.rust-lang.org/std/path/index.html)
    * [`std::path::Path` - a slice of a path](https://doc.rust-lang.org/std/path/struct.Path.html)
    * [`std::path::PathBuf` - an owned, mutable path](https://doc.rust-lang.org/std/path/struct.PathBuf.html)
  * [`std::prelude` - the list of symbols which is preloaded](https://doc.rust-lang.org/std/prelude/index.html)
  * [`std::process` - a module for working with processes](https://doc.rust-lang.org/std/process/index.html)
    * [`std::process::Command` - a process builder](https://doc.rust-lang.org/std/process/struct.Command.html)
    * [`std::process::Output` - the output of a finished process](https://doc.rust-lang.org/std/process/struct.Output.html)
  * [`std::result` - error handling with the `Result` type](https://doc.rust-lang.org/std/result/index.html)
    * [`std::result::Result` - a type that represents either success (`Ok`) or failure (`Err`)](https://doc.rust-lang.org/std/result/enum.Result.html)
  * [`std::str` - utilities for the `str` primitive type](https://doc.rust-lang.org/std/str/index.html) [[`str` primitive type](https://doc.rust-lang.org/std/primitive.str.html)]
  * [`std::string` - a growable UTF-8 string module](https://doc.rust-lang.org/std/string/index.html)
    * [`std::string::String` - a growable UTF-8 string](https://doc.rust-lang.org/std/string/struct.String.html)
  * [`std::vec` - a contiguous growable array type with heap-allocated contents](https://doc.rust-lang.org/std/vec/index.html)
    * [`std::vec::Vec` - a contiguous growable array type](https://doc.rust-lang.org/std/vec/struct.Vec.html)
* [`syn` - parser for Rust source code](https://crates.io/crates/syn) [[doc](https://docs.rs/syn/latest/syn/)] [[repo](https://github.com/dtolnay/syn)]
* [`tao` - cross-platform window manager library](https://crates.io/crates/tao) [[doc](https://docs.rs/tao/latest/tao/)] [[repo](https://github.com/tauri-apps/tao)]
* [`tauri` - a framework for building tiny, blazing fast binaries for all major desktop platforms](https://crates.io/crates/tauri) [[home](https://tauri.app/)] [[doc](https://docs.rs/tauri/latest/tauri/)] [[repo](https://github.com/tauri-apps/tauri)]
* [`thread_local` - per-object thread-local storage](https://crates.io/crates/thread_local) [[doc](https://docs.rs/thread_local/latest/thread_local/)] [[repo](https://github.com/Amanieu/thread_local-rs)]
* [`ungrammar` - a DSL for specifying concrete syntax trees](https://crates.io/crates/ungrammar) [[doc](https://docs.rs/ungrammar/latest/ungrammar/)] [[repo](https://github.com/rust-analyzer/ungrammar)]
  * [Introducing Ungrammar](https://rust-analyzer.github.io/blog/2020/10/24/introducing-ungrammar.html)
* [`url` - URL library for Rust](https://crates.io/crates/url) [[doc](https://docs.rs/url/latest/url/)] [[repo](https://github.com/servo/rust-url)]
* [`walkdir` - recursively walk a directory](https://crates.io/crates/walkdir) [[doc](https://docs.rs/walkdir/latest/walkdir/)] [[repo](https://github.com/BurntSushi/walkdir)]
* [`which` - a Rust equivalent of Unix command `which`](https://crates.io/crates/which) [[doc](https://docs.rs/which/latest/which/)] [[repo](https://github.com/harryfei/which-rs)]
* [`xshell` - quick shell scripting in Rust](https://crates.io/crates/xshell) [[doc](https://docs.rs/xshell/latest/xshell/)] [[repo](https://github.com/matklad/xshell)]
