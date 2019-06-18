# C Cheat Sheet

The purpose of this document is to provide a handy notes about C programming
language. Its aim is to be as short as possible and at the same time cover the
most important parts of the language.

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

* **TODO**

### 2.1 C Program

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

* **TODO**

## 4 Variables

* **TODO**

## 5 Expressions

* **TODO**

### 5.1 Atomic Expressions

#### String Constants (Literals)

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

## 6 Statements

* **TODO**

## 7 Functions

* **TODO**
  * values are passed to function throught *arguments*

## 8 Preprocessor

* **TODO**
  * `#include <header.h>` - include the content of `header.h` to the currently
    processed source file

## 9 References

* Kernighan, Brian and Ritchie, Denis: *The C Programming Language*, 1978.
* *ISO/IEC 9899-1990* (C90), 1990.
* *ISO/IEC 9899-1999* (C99), 1999.
