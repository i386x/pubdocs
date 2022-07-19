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

#### Keywords

```C++
asm               do                  if                      return             typedef
auto              double              inline                  short              typeid
bool              dynamic_cast        int                     signed             typename
break             else                long                    sizeof             union
case              enum                mutable                 static             unsigned
catch             explicit            namespace               static_cast        using
char              export              new                     struct             virtual
class             extern              operator                switch             void
const             false               private                 template           volatile
const_cast        float               protected               this               wchar_t
continue          for                 public                  throw              while
default           friend              register                true
delete            goto                reinterpret_cast        try
```

### Integer Constants

### Floating-Point Constants

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

* `double`
  * floating-point data type

## Declarations

* every entity must be declared before it is used
* a declaration associates an identifier with the type
  * if the declaration also reserves the storage and the location of the
    storage and associates them with the identifier then it is called the
    *definition*

### Variable (Object) Declarations

* *object* is an entity with given storage and location
* a declaration of variable/object
  * either works also as its definition
  * or tells the compiler it is defined elsewhere (such a declaration is called
    *external declaration*)

* *type* *identifier* `;`
  * states that *identifier* represents the object of type *type*
  * reserve the storage and location for the object and associate it with
    *identifier*
* *type* *identifier* `=` *expression* `;`
  * like *type* *identifier* `;`, but also initializes the object associated
    with *identifier* with *expression*
  * the value of *expression*, after possible promotions and conversions, must
    have the same type as *type* or must be convertible to *type*

### Function Declarations

* a function signature followed by `;` is called a *function prototype*
* *return-type* *identifier* `(` *specification-of-parameters* `)` `;`
  * declares *identifier* as a function accepting arguments given by
    *specification-of-parameters* and returning a value of *return-type*

## Expressions

* following subsections describing operators are ordered by priority in
  descending order (highest priority first)
* some operators may be overloaded
  * compiler choose the proper variant based on types of the operands

### Primary Expressions

* literal, identifier, and qualified identifier is a primary expression
* `(` *expression* `)` is a primary expression
* qualified identifier is an identifier determined by the scope resolution
  operator (`::`)

### Scope Resolution Operator (`::`)

* `::` operator groups left to right
* it helps to qualify identifiers
  * `A :: B` tells the compiler to look for `B` in the name space `A`
  * `::foo` tells the compiler to look for `foo` in the global name space

### Postfix Operators

* postfix operators `()` group left to right

* *E* `(` *list-of-arguments* `)`
  * *E* must evaluate to the callable entity
  * *list-of-arguments* is a comma-separated list of expressions
    * the number and type of expressions, after they are evaluated and the
      values are converted, must match the *E*'s signature
  * expressions in the *list-of-arguments* are evaluated in any order, but
    after *E*
  * after all expressions in the *list-of-arguments* are evaluated, *E* is
    called with their values as its arguments
    * argument value is assigned to the corresponding function parameter
      declared in the *E*'s signature
    * in callee, parameters are seen as local variables defined and initialized
      every time the function is called
  * the type of *E* `(` *list-of-arguments* `)` is the return type of *E*
  * the value of *E* `(` *list-of-arguments* `)` is the value returned by
    callee

### Multiplicative Operators

* multiplicative operators `*` group left to right
* *E1* `*` *E2*
  * both *E1* and *E2* have arithmetic types
    * the values of *E1* and *E2* are converted to the same types according to
      the conversion rules and this type is the type of the result
    * the value of the expression is the value of *E1* multiplied by the value
      of *E2*

### Additive Operators

* additive operators `+`, `-` group left to right
* *E1* `+` *E2*, *E1* `-` *E2*
  * both *E1* and *E2* have arithmetic types
    * the values of *E1* and *E2* are converted to the same types according to
      the conversion rules and this type is the type of the result
    * the value of the *E1* `+` *E2* is the value of *E1* plus the value of
      *E2*
    * the value of the *E1* `-` *E2* is the value of *E1* minus the value of
      *E2*

### Shift Operators

* shift operators `<<` and `>>` group left to right

### Assignment Operators

* assignment operators `=` group right to left
* *E1* `=` *E2*
  * assigns the value of *E2* to the object referred by *E1*
  * *E1* must refer to a modifiable object
  * the value of *E1* `=` *E2* is the value of *E1* after assignment
  * the value of *E2*, after possible promotions and conversions, must have the
    same type as *E1* or must be convertible to the type of *E1*

## Statements

### Expression Statement

* construct *expression* `;` is a statement

### `return` Statement

* has two forms
  1. `return` `;` (for functions declared with `void` return-type)
  1. `return` *expression* `;`
* can only be used within functions
* returns the control-flow back to the caller
* *expression*'s type must coincide with the function's return type
  * the value of *expression* is returned to the caller (it is the value of
    `E(...)` expression)

## Functions

* a definition of a function consists of a *function signature* immediately
  followed by a *function body*
* a function signature has the form: *return-type* *identifier* `(`
  *specification-of-parameters* `)`
* a function body has the form: `{` *declarations-and-statements* `}`
  * note that function definition is neither declaration nor statement and thus
    nested functions are forbidden
* a *return-type* specifies a type of the information returned to the caller
  * such information is called *return value*
  * if *return-type* is `void`, the function has no return value
* an *identifier* specifies the name of the function
* a *specification-of-parameters* specifies a number and types of arguments
  that caller must pass to the function right before the function code is
  executed
* a *specification-of-parameters* has following forms
  * it is empty or `void`; this means that the function has no parameters
  * it is a comma-separated list of *parameter declarations*
* a *parameter declaration* can have one of the following forms
  1. *type* *identifier*
     * specifies the type and name of the function parameter
     * within the function, *identifier* is seen as a variable of type *type*,
       defined every time the function is called, and initialized by the caller
     * *identifier* can be omitted in prototypes

### Function `main`

* prototypes
  1. `int main();`
* the function `main` is the first function called when a program starts, that
  is the function `main` is the *entry point* of the program
* if there is no `return` statement at the end of the `main`'s function body,
  it is as if the last statement of the `main`'s function body was `return 0;`
* the `main`'s return value is the program's exit code

## Name Spaces

* name spaces provide facility for organizing identifiers
  * the same identifier can be member of different name spaces
* name spaces can be nested

### Directive `using`

* `using namespace foo;` exports all identifiers from the name space `foo` to
  the current scope
* `using foo::bar;` exports `bar` from the name space `foo` to the current
  scope

## Classes

* *class* is user defined data type
* the definition of class consist of
  * the definition of data
  * the definition of operations over the data
    * member functions
    * overloaded operators

## References

* Stephen Prata: *C++ Primer Plus*, 5th edition, 2005.
* *ISO/IEC 14882:1998*, 1998.
* *ISO/IEC 14882:2003*, 2003.
