# The C++ Programming Language Notes

This document contains notes taken from the book *C++ Primer Plus* by Stephen
Prata.

## Contents

## C++ Project

* a C++ project can be a collection of C++ programs and libraries
* a C++ program consists of functions
  * every C++ program should have defined the entry point function, which is
    conventionally named `main` (depending on the type of an application there
    can be exceptions)
* a C++ library is a collection of functions, classes, templates, and other
  code, intended to be shared by other C++ programs and libraries
* a C++ project is organized into two types of plain text files
  * header files providing declarations of functions, classes, variables and
    data types, and definitions of common constants
  * source files providing their definitions (if the definition makes sense)
* both types of files have their suffixes
  * source files: `C`, `cc`, `cxx`, `cpp`
  * header files: `H`, `hh`, `hxx`, `hpp`, no suffix (system header files)

## Lexical Elements

* the first thing compiler do is splitting the source code to the sequence of
  lexical elements, called *tokens*

### White Space Characters

* white space characters (horizontal tab, vertical tab, form feed, line feed,
  carriage return, space) are ignored by a compiler unless they are part of
  another lexical element (e.g. string literal or character constant)
* they work as delimiters between other lexical elements

### Comments

* ignored by a compiler
* single line comments
  * begin with `//` and end with a new-line character (`\n`)
* multi-line comments (from C) are also supported
  * begin with `/*` and end with `*/`

### Identifiers

* identifiers are case sensitive

### Character Constants and String Literals

* in C++, a *character* is one of
  * horizontal tab (`\t`, 9)
  * vertical tab (`\v`, 11)
  * form feed (`\f`, 12)
  * new line (aka line feed, `\n`, 10)
  * space (32)
  * symbols with ASCII values 33 to 126, including 126
  * escape sequence

* escape sequences
  * new line (ASCII 10): `\n`

* a string literal is a sequence of zero or more characters, except double
  quote, back slash, and new line, enclosed between double quotes

## Preprocessor

* source files are processed by preprocessor before they go to compiler
* preprocessor
  * process directives (lines beginning with `#`)
  * expand macros and symbolic constants

### Directives

* `#include`
  * `#include <file>` tells the preprocessor to replace `#include <file>` with
    the content of `file`, where `file` is expected to be stored in the system
    location

## Data Types

### Basic Data Types

* `int`
  * integral data type
  * can hold both negative and non-negative values

## Declarations

* a declaration associates an identifier with the type
  * if the declaration also reserves the storage and the location of the
    storage and associates them with the identifier then it is called the
    *definition*

### Variable (Object) Declaration

* *object* is an entity with given storage and location
* a declaration of variable/object
  * either works also as its definition
  * or tells the compiler it is defined elsewhere (such a declaration is called
    *external declaration*)

* *type* *identifier* `;`
  * states that *identifier* represents the object of type *type*
  * reserve the storage and location for the object and associate it with
    *identifier*

## Expressions

* following subsections describing operators are ordered by priority in
  descending order (highest priority first)
* some operators may be overloaded
  * compiler choose the proper variant based on types of the operands

### `::` Operator

* `::` operator groups left to right
* it helps to qualify identifiers
  * `A :: B` tells to compiler to look for `B` in the name space `A`
  * `::foo` tells to compiler to look for `foo` in the global name space

### Shift Operators

* shift operators `<<` and `>>` group left to right

## Statements

### Expression Statement

* construct *expression* `;` is a statement

### `return` Statement

* has two forms
  1. `return` `;` (for functions declared with `void` return-type)
  1. `return` *expression* `;`
* can only be used within functions
* returns the control-flow back to the caller

## Functions

* a definition of a function consists of a *function prototype* immediately
  followed by a *function body*
* a function prototype has the form: *return-type* *identifier* `(`
  *specification-of-parameters* `)`
* a function body has the form: `{` *declarations-and-statements* `}`
* a *return-type* specifies a type of the information returned to the caller
  * such information is called *return value*
* a *specification-of-parameters* specifies a number and types of arguments
  that caller must pass to the function right before the function code is
  executed
  * unlike C, missing *specification-of-parameters* means that the function has
    no parameters; the same effect has also when *specification-of-parameters*
    is `void`

### Function `main`

* prototypes
  1. `int main()`
* the function `main` is the first function called when a program starts, that
  is the function `main` is the *entry point* of the program
* if there is no `return` statement at the end of the `main`'s function body,
  it is as if the last statement of the `main`'s function body was `return 0;`

## Name Spaces

* name spaces provide facility for organizing identifiers
  * the same identifier can be member of different name spaces
* name spaces can be nested

### Operator `::`

* operator `::` is used to access the name space member
  * `A :: B` means *use `B` from name space `A`*

### Directive `using`

* `using namespace foo;` exports all identifiers from the name space `foo` to
  the current scope
* `using foo::bar;` exports `bar` from the name space `foo` to the current
  scope

## References

* Stephen Prata: *C++ Primer Plus*, 5th edition, 2005.
* *ISO/IEC 14882:1998*, 1998.
* *ISO/IEC 14882:2003*, 2003.
