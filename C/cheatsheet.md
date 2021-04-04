# C Cheat Sheet

The purpose of this document is to provide a handy notes about C programming
language. Its aim is to be as short as possible and at the same time to cover
the most important parts of the language. The standard C library is not
included in this cheat sheet.

## 0 Contents

1. [Overview](#1-overview)
1. [C Project](#2-c-project)
   1. [C Program](#2-1-c-program)
   1. [C Library](#2-2-c-library)
1. [References](#9-references)

## 1 Overwiev

* machine and system independent programming language
* provides data types
  * basic (atomic): characters, integral and floating-point numbers
  * composed: pointers, arrays, structures and unions
  * enumerations
* supports expressions
  * composed from operators and operands
    * operators reflect operations supported by the target machine
    * operands are characters, numbers, and addresses
  * can be used as statements
* supports machine independent pointer arithmetic
* supports constructions and statements for structured programming
  * organizing statements into blocks or functions
  * branching (`if-else`)
  * selection from the set of possible cases (`switch`)
  * loops (`for`, `while`, `do-while`)
  * loop termination (`break`)
* supports functions
  * may return values of base data types, structures, unions, and pointers
  * can be called recursively
  * can be defined separately in separately compiled source files
* variables
  * block scoped
  * can be declared statically or locally
  * local variables
    * declared in function
    * automatic (stack based)
    * created every time when function is called
  * levels of visibility
    * function < source file < program
* preprocessor
  * expand macros
  * include source files
  * support conditional compilation
* rich standard library
* lot of 3rd party libraries

* do not support
  * nested functions definitions
  * operations with composed objects (except structure copying)
  * automatic memory and other resources management
  * input and output operations
  * concurrency and multithreading

## 2 C Project

### 2.1 Source Files

#### Comments

* comments in C are multiline
* a comment starts with `/*` and ends with `*/`

### 2.2 C Program

* every C program is consisting of *functions* and *variables*
* source files of C programs (files where functions and variables are defined)
  have `.c` suffix
* every C program must contain the function `main` which serves as a program
  entry point

#### Example

The simplest program in C looks like this:

```C
/* hello.c */
#include <stdio.h>

int main(void)
{
  printf("Hello, World!\n");
  return 0;
}
```

**Note:** The above for earlier versions of C compilers:

```C
/* hello.c */
#include <stdio.h>

main()
{
  printf("Hello, World!\n");
}
```

* functions have `int` as implicit return type
* implicit return value of functions is zero

### 2.2 C Library

* **TODO**

## 3 Data Types

### 3.1 Base Data Types

* size is machine dependent
* `char` - character, byte
* `short` - short integer
* `int` - integer
  * can be 16 bits or 32 bits long
* `long` - long integer, at least 32 bits long
* `float` - floating point number (single precision)
  * usually 32 bits, 6 significant digits, range from `1e-38` to `1e38`
* `double` - floating point number (double precision)
* can be arranged to arrays, structures, and unions
* can be referenced by pointers
* can be returned by functions

## 4 Variables

* variables must be declared before they are used

### 4.1 Declaration

A general form of declaration is
```
<type> <list-of-variables> ";"
```
where `<type>` is a type of a variable and `<list-of-variables>` is a comma
separated list of identifiers.

Arrays are declared

#### Example

```C
int a, b, c;
```
declares three variables `a`, `b`, and `c` of type `int`.

## 5 Expressions

* expression evaluation can be changed by enclosing subexpressions between
  parentheses, compare `a + b*c` and `(a + b)*c`
* during the evaluation, when the value of expression is known, the expression
  is not further evaluated (this is called *short evaluation*)

### 5.1 Atomic Expressions

#### Number Literals

* character literals are written as a single character or a single escape
  sequence between single quotes, e.g. `'A'`, `'\n'`, and their values are
  ASCII values of quoted character/escape sequence
* integers are written as sequence of digits, e.g. `1`, `123`, etc
* floating point numbers are written as a sequence of digits containing a dot
  that delimits integral and fractional part. e.g. `0.4`, `42.0`, etc

#### String Literals

* sequence of characters between `"`
  * exaples: `""`, `"abcd"`, `"abcd\n"`
* characters that are not allowed to be between `"` directly can be expressed
  using *escape sequences*

**Note:** escape sequences in string literals:

Escape Sequence | Meaning | Code | Note
--------------- | ------- | ---- | ----
`\b` | backspace character | 8 |
`\t` | tab character | 9 |
`\n` | new line character | 10 |
`\"` | `"` character | 34 |
`\\` | `\` character | 92 |

### 5.2 Assignment Expressions

* `a = b` - convert `b` to the `a`'s type, assign `b`'s value (after
  conversion) to `a`, and return `a`'s value
* evaluated from right to left

### 5.3 Arithmetic Expressions

* if both operands are integers, the operation is done over integers
* if one of the operands is a floating point number and the other one is an
  integer, it is converted to a floating point number before the operation is
  performed
* `a + b` addition
* `a - b` subtraction
* `a * b` multiplication
* `a / b` division
  * if `a` and `b` are both integers, the result is truncated
* `++a` increase `a` by one, evaluates to `a + 1`
* `a++` increase `a` by one, evaluates to `a`
* `--a` decrease `a` by one, evaluates to `a - 1`
* `a--` decrease `a` by one, evaluates to `a`

### 5.4 Comparisons

* same conversion rules as for arithmetic expressions are applied
* `a < b` is 1 if `a` is less than `b`
* `a <= b` is 1 if `a` is less or equal to `b`
* `a != b` is 1 if `a` is not equal to `b`
* `a == b` is 1 if `a` is equal to `b`

### 5.5 Logic Expressions

* evaluated from left to right
* `a || b` true if `a` or `b` is true
* `a && b` true if both `a` and `b` are true

## 6 Statements

### 6.1 Empty Statement

```C
;
```

No operation.

### 6.2 Expression Statement

Syntax:
```C
expression;
```

Evaluate `expression`.

### 6.3 Branching

Syntax:
```C
if (expr)
  stmt1
else
  stmt2
```

If `expr` is true, do `stmt1`. Otherwise, do `stmt2`.

### 6.4 Loops

#### `while`

Syntax:
```C
while (expression) {
  statement1;
  statement2;
  ...
  statementN;
}

while (expression)
  statement;
```

Semantics:
1. evaluate `expression`
1. if `expression` is true
   1. execute statements
   1. go to 1

#### `for`

Syntax:
```C
for (expr1; expr2; expr3) {
  stmt1;
  stmt2;
  ...
  stmtN;
}

for (expr1; expr2; expr3)
  statement;
```

Semantics:
1. evaluate `expr1`
1. if `expr2` is true
   1. execute statements
   1. evaluate `expr3`
   1. go to 2

## 7 Functions

* values are passed to function through *arguments*

## 8 Preprocessor

* `#include <header.h>` - include the content of `header.h` to the currently
  processed source file
* `#define NAME TEXT` - define macro `NAME` that expands to `TEXT`; `NAME` is
  an identifier; since now, every occurrence of identifier `NAME` is replaced
  by `TEXT`

## 9 References

* Kernighan, Brian and Ritchie, Denis: *The C Programming Language*, 1978.
* *ISO/IEC 9899-1990* (C90), 1990.
* *ISO/IEC 9899-1999* (C99), 1999.
