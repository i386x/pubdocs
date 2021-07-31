# Expressions

* *l-value* is an expression referring to an object
* subexpressions in expression can be evaluated in any order
  * exceptions to this rule are `()`, `&&`, `||`, `?:`, and `,`
* the parse of expression is respected (i.e. `a + b` is not evaluated as
  `b + a`)
* dealing with exceptions during expression evaluation are not defined
  * sometimes the behavior can be adjusted by non-standard library functions
* overflows during integral operations are mostly ignored, but this is not in
  general guaranteed

## Conversions

The following text uses these auxiliary functions and macros:
* *trunc(F)* truncates the fractional part of floating point value *F*
* *fround(N)* returns the floating point value nearest to *N*
* `abs(x)` returns the absolute value of `x`
* `ipow(x, y)` returns the `y`th power of `x`
* `NBITS(x)` returns the number of bits occupied by `x` (i.e. `8 * sizeof x`)
* `UNSIGNED(x)` converts `x` to the corresponding unsigned type
* `PROMOTE(x)` performs integral promotion of `x`
* `PROMOTED_TYPE(x)` gets the type of promoted `x`
* `PROMOTED_TYPE_MAX(x)` returns the largest value of the type of promoted `x`

### Integral Promotion

* `char`, `short`, bit field, all signed and unsigned, and an object of `enum`
  type, can be used everywhere where integer is expected
  * if the value fits to `int`, it is converted to `int`
  * otherwise, it is converted to `unsigned int`

### Integral Conversions

* integer *I* to unsigned type *U*
  1. *Iu = I mod (Umax + 1)*
  1. if *Iu < 0*, then *Iu = Iu + (Umax + 1)*
  where *Iu* is the *I*'s value after conversion and *Umax* is the largest
  value that can be represented in *U*; in two's complement arithmetic
  * if *U* is narrower, extra left bits are truncated
  * if *U* is wider and *I* is non-negative, extra left bits are filled with
    zeros
  * otherwise, extra left bits are filled with ones (*sign extension*)
* integer *I* to signed type *S*
  * if *I* can be represented in *S*, the value is unchanged
  * otherwise the result is implementation-defined

### Integral and Floating Point Types

* floating type value *F* to integral type value *I*
  * *I = trunc(F)*
  * if *trunc(F)* cannot be represented in *I*'s type, the behavior is
    undefined
  * conversion of negative floating values to unsigned integral types is not
    defined
* integral value *I* to floating value *F*
  * if *I* is in the *F*'s type range, then *F = fround(I)*
  * if it is out of range, the behavior is undefined

### Floating Point Types

* less precise to more precise
  * value is unchanged
* more precise to less precise
  * within the range: *fround(value)*
  * out of range: undefined

### Arithmetic Conversions

In binary arithmetic expressions, these conversion rules before the arithmetic
operation are performed:
1. if one operand is `long double`, the other is converted to `long double`
1. if one operand is `double`, the other is converted to `double`
1. if one operand is `float`, the other is converted to `float`
1. apply integral promotion to both operands
1. if one operand is `unsigned long int`, the other is converted to
   `unsigned long int`
1. if one operand is `long int` and the other is `unsigned int`
   * if `long int` can represent all `unsigned int` values, the other is
     converted to `long int`
   * otherwise, both are converted to `unsigned long int`
1. if one operand is `long int`, the other is converted to `long int`
1. if one operand is `unsigned int`, the other is converted to `unsigned int`
1. both operands have type `int`

### Pointers and Integers

* integral value can be added to or subtracted from a pointer
* two pointers of the same type and in the same array can be subtracted
* `0` or `(void *)0` can be converted (by cast, assignment, comparison) to a
  pointer of any type; this produces a *null pointer*
* null pointer is equal to another null pointer of the same type, but unequal
  to any other pointer

Other pointer conversions are possible, but they are implementation-dependent
and explicit cast is needed:
* pointer to integral type conversion
  * integral type must be large enough, the size depends on implementation
  * the mapping function also depends on implementation
* integral type to pointer
  * if pointer to integer conversion was successful, then the reverse integer
    to pointer conversion assures the same pointer
  * other cases depend on implementation
* pointer to pointer conversion
  * may cause addressing exceptions as a result of different alignment
    requirements
  * the notion of *alignment* is implementation dependent
  * conversion to pointer to type with less or equally strict alignment and
    back is guaranteed to succeed
  * `char` type has a least strict alignment requirements
  * conversion to `void *` and back are also guaranteed to succeed
* pointer to pointer conversion (same type, different qualifiers)
  * qualifiers are added: new restrictions are implied
  * qualifiers are removed: old restrictions still apply in operations
* pointer to pointer conversion (functions case)
  * allowed, but the calling of the result is implementation dependent
  * conversion to the original type guarantees no change in behavior when the
    function is called

### Void

* `void` to other type conversion is not allowed
* `void` expression can be used only in cases where the value is not required
  (e.g. as left operand of comma expression or as an expression statement)
* conversion to `void`
  * by cast
  * discards the value

### Pointers to Void

* `void *` is a generic pointer
* pointer to `void *` conversion guarantees no information loss
* conversion back to the original type recovers the pointer
* assignments to/from `void *` pointers and comparisons of `void *` pointer
  with another pointer do not require explicit casts

### Pointer Generation

* if the expression *E* has type *array of T*, then
  * its type is altered to `T *`
  * its value is `&(E[0])`
  * the exceptions from this rule are expressions `&E`, `++E`, `--E`,
    `sizeof(E)`, left operand of an assignment operator or the `.` operator
* if the expression *E* has type *function returning T*, then
  * its type is converted to pointer to function returning *T*
  * the exception from this rule is the expression `&E`

## Expressions

### Primary Expressions

Grammar for primary expressions:
```abnf
primary-expression = identifier
primary-expression =/ constant
primary-expression =/ string-literal
primary-expression =/ "(" expression ")"
```

* identifier is an l-value if it refers to an object of arithmetic, structure,
  union, or pointer type
* string literal has type array of `char` (`wchar_t`), but the pointer
  generation rules are usually (except certain initializers) applied

### Postfix Expressions

Grammar for postfix expressions:
```abnf
postfix-expression = primary-expression
postfix-expression =/ postfix-expression "[" expression "]"
postfix-expression =/ postfix-expression "(" [ argument-expression-list ] ")"
postfix-expression =/ postfix-expression "." identifier
postfix-expression =/ postfix-expression "->" identifier
postfix-expression =/ postfix-expression "++"
postfix-expression =/ postfix-expression "--"

argument-expression-list = assignment-expression *( "," assignment-expression )
```

#### Array References

* in `E1[E2]`, one expression must have type *pointer to T* and the other one
  must have integral type
  * the type of `E1[E2]` is *T*
* `E1[E2]` is identical to `*((E1) + (E2))`

#### Function Calls

* postfix expression in function call is called the *function designator*
* if the function designator is an undeclared identifier, it is implicitly
  declared at the current scope level as `extern int identifier();`
* the function designator must be of type *pointer to function returning T*
  * the value of the function call has type *T*
  * the older specification restricts the type to *function*; hence, pointer to
    function must be explicitly dereferenced by `*`
* under the term *argument* (or *actual argument/parameter*) is understood an
  expression passed by a function call
* under the term *parameter* (or *formal argument/parameter*) is understood an
  input object
  * received by a function definition
  * described by a function declaration
* before the function call, each argument is copied
  * arguments are passed *by value*
* if the function declaration in the function call scope is old style
  * integral promotion is applied to integral type arguments
  * `float` arguments are converted to `double`
  * if the number of arguments and parameters disagrees, the effect of call is
    undefined
  * if types of arguments after promotion and parameters disagrees, the effect
    of call is undefined
    * if the function's definition is old style, the promoted types (both
      arguments and parameters) are compared
    * if it is new style, the promoted type of the argument must match that of
      the parameter
* if the function declaration in the function call scope is new style
  * arguments are converted to the types of the corresponding parameters
    * the conversion is as in the assignment
  * the number of arguments must match the number of explicitly defined
    parameters, unless the last parameter is ellipsis (`...`)
  * in the case of `...`, the number of arguments must be greater or equal to
    the number of parameters
    * arguments covered with `...` are promoted as described in the case of
      old style declared functions
  * if the function's definition is old style
    * the type of the parameter from the prototype visible at the call must
      match the type of the corresponding function's definition parameter after
      that parameter's promotion
* the order of arguments evaluation is not specified
* before the function is entered, designator and all arguments are fully
  evaluated
* recursive calls are permitted

#### Structure References

* in `E.I`, `E` must be a structure or a union, and `I` must be the name of a
  member of `E`
* the value of `E.I` is the value of the named member of `E`, the type of `E.I`
  is the type of the member
* `E.I` is an l-value if `E` is an l-value and the type of `E.I` is not an
  array type
* in `P->I`, `P` must be a pointer to a structure or a union, and `I` must be
  the name of a member of `E`
* `P->I` is the same as `(*P).I`
  * the result of `P->I` refers to the named member of `*P` and the type is the
    type of the member
  * the result of `P->I` is an l-value if the type is not an array type

#### Postfix Incrementation

* in `E++` and `E--`, the value of `E++` and `E--` is the value of `E`
  * after the value is noted, `E` is incremented (`++`) or decremented (`--`)
    by 1
* `E` must be l-value, the result of `E++` and `E--` is not an l-value

### Unary Expressions

Grammar for unary expressions:
```abnf
unary-expression = postfix-expression
unary-expression /= "++" unary-expression
unary-expression /= "--" unary-expression
unary-expression /= unary-operator cast-expression
unary-expression /= %x73.69.7A.65.6F.66 unary-expression
unary-expression /= %x73.69.7A.65.6F.66 "(" type-name ")"

unary-operator = "&" / "*" / "+" / "-" / "~" / "!"
```

#### Prefix Incrementation Operators

* in `++E` and `--E`, `E` is incremented (`++`) or decremented (`--`) by 1
* the value of `++E` (`--E`) is the value of `E` after the incrementation
  (decrementation)
* `E` must be an l-value, the result of `++E` and `--E` is not an l-value

#### Address Operator

* `&E` takes the adrress of `E`
* `E` must be an l-value referring
  * neither to a bit field
  * not to a `register` object
  or must be of function type
* `&E` evaluates to the pointer to object `E` or, if `E` is of function type,
  to the function referred to by the l-value `E`
* if `E` has the type *T*, `&E` has the type *pointer to T*

#### Indirection Operator

* `*E` returns the object or function to which `E` points
* if `E` is a pointer to an object of arithmetic, structure, union, or pointer
  type, then `*E` is an l-value
* if `E` has the type *pointer to T*, then `*E` has the type *T*

#### Unary Plus Operator

* in `+E`, `E` must have arithmetic type (if it is integral, `E` undergoes
  integral promotion)
* the value of `+E` is the value of `E`
* the type of `+E` is the type of promoted `E`

#### Unary Minus Operator

* in `-E`, `E` must have arithmetic type (if it is integral, `E` undergoes
  integral promotion)
* the value of `-E` is the negative of the value of `E`
  * if `E` is unsigned, then `-E == PROMOTED_TYPE_MAX(E) - PROMOTE(E) + 1`;
    `-0 == 0`
* the type of `-E` is the type of promoted `E`

#### One's Complement Operator

* in `~E`, `E` must have integral type (integral promotions are performed)
* the value of `~E` is the one's complement of the value of `E`
  * if `E` is unsigned, then `~E == PROMOTED_TYPE_MAX(E) - PROMOTE(E)`
  * if `E` is signed, then `~E == (PROMOTED_TYPE(E))~UNSIGNED(PROMOTE(E))`
* the type of `~E` is the type of promoted `E`

#### Logical Negation Operator

* in `!E`, `E` must have arithmetic type or be a pointer
* the value of `!E` is 1 if `E == 0` and 0 otherwise
* the type of `!E` is `int`

#### `sizeof` Operator

* in `sizeof E`, `E` is either expression (not evaluated) or `(T)`, where `T`
  is a type name
* `E` must not be a function type, incomplete type, or bit field
* the value of `sizeof E` is the number of bytes needed to store an object of
  the type of `E`
  * if `E` has type `char`, it is 1
  * if `E` is an array, it is the total number of bytes in the array
    * `sizeof T[n] == n * sizeof(T)`
  * if `E` is a structure or union, it is the number of bytes in the object,
    including any padding required to make the object tile an array
* the type of `sizeof E` is unsigned integral constant
  * the particular type is implementation-defined (`<stddef.h>` defines this
    type as `size_t`

### Cast Expressions

Grammar for cast expressions:
```abnf
cast-expression = unary-expression
cast-expression /= "(" type-name ")" cast-expression
```

* the construction `(T)E` is called a *cast*
* the value of `E` is converted to the type `T`
* `(T)E` is not an l-value

### Multiplicative Expressions

Grammar for multiplicative expressions:
```abnf
multiplicative-expression = cast-expression
multiplicative-expression /= multiplicative-expression "*" cast-expression
multiplicative-expression /= multiplicative-expression "/" cast-expression
multiplicative-expression /= multiplicative-expression "%" cast-expression
```

* in `E1 * E2` and `E1 / E2`, `E1` and `E2` must have arithmetic type
* in `E1 % E2`, `E1` and `E2` must have integral type
* first, `E1` and `E2` undergo the usual arithmetic conversions, then the type
  of the result is predicted
* `E1 * E2` denotes the multiplication of `E1` by `E2`
* `E1 / E2` yields the quotient of the division of `E1` by `E2`
  * if `E2` is 0, the result is undefined
* `E1 % E2` yields the remainder of the division of `E1` by `E2`
  * if `E2` is 0, the result is undefined, otherwise `(a/b)*b + a%b == a`
  * if both `E1 >= 0` and `E2 >= 0`, then `E1 % E2 >= 0` and `E1 % E2 < E2`;
    otherwise it is only guaranteed then `abs(E1 % E2) < abs(E2)`

### Additive Expressions

Grammar for additive expressions:
```abnf
additive-expression = multiplicative-expression
additive-expression /= additive-expression "+" multiplicative-expression
additive-expression /= additive-expression "-" multiplicative-expression
```

* in `E1 + E2` or `E1 - E2`, if `E1` or `E2` have arithmetic type, the usual
  arithmetic conversions are performed
* `E1 + E2` evaluates to the sum of `E1` and `E2`, `E1 - E2` evaluates to the
  difference of `E1` and `E2`
* a pointer `P` to an object in an array and a value `I` of any integral
  type may be added
  * the type of `P + I` is the type of `P`
  * `I` is converted to the address offset by multiplying `I` by `sizeof *P`
  * `P + I` points to the object in the same array appropriately offset from
    `P` (`P + 1` points to the object next to `P`)
  * if `B` points to the first object in the array and `I < 0` or `I > N`,
    where `N` is the size of the array, then `B + I` is undefined
* in `P - I`, if `P` is a pointer and `I` a value of any integral type, then
  the same conversions and conditions as for `P + I` apply
* if `P1` and `P2` are pointers to objects of the same type, `P1 - P2` is the
  displacement between `*P1` and `*P2`
  * signed integral value
  * the type depends on the implementation; in `<stddef.h>`, it is defined as
    `ptrdiff_t`
  * for two successive objects, the difference is 1
  * if `P1` and `P2` not point to objects within the same array, the value is
    undefined
* if `P` points to the last element of an array, then `(P + 1) - P` is 1

### Shift Expressions

Grammar for shift expressions:
```abnf
shift-expression = additive-expression
shift-expression /= shift-expression "<<" additive-expression
shift-expression /= shift-expression ">>" additive-expression
```

* in `E1 << E2` and `E1 >> E2`, `E1` and `E2` must be integral and they undergo
  integral promotions
* the type of the result is the type of promoted `E1`
* if `E2 < 0` or `E2 >= NBITS(E1)`, then the result is undefined
* `E1 << E2` evaluates to `E1` left-shifted `E2` bits; this is equivalent to
  `E1 * ipow(2, E2)` without overflow
* `E1 >> E2` evaluates to `E1` right-shifted `E2` bits
  * if `E1` is unsigned or `E1 >= 0`, this is equivalent to `E1 / ipow(2, E2)`
  * otherwise the result is implementation-defined

### Relational Expressions

Grammar for relational expressions:
```abnf
relational-expression = shift-expression
relational-expression /= relational-expression "<" shift-expression
relational-expression /= relational-expression ">" shift-expression
relational-expression /= relational-expression "<=" shift-expression
relational-expression /= relational-expression ">=" shift-expression
```

* `a R b`, where `R` is one of `<` (less), `>` (greater), `<=` (less or equal)
  and `>=` (greater or equal), evaluates to
  * 1 if the relation is true
  * 0 otherwise
* the type of the result of `a R b` is `int`
* arithmetic operands undergo the usual arithmetic conversions
* two pointers, `P1` and `P2`, may be compared
  * `*P1` and `*P2` must have same type
  * qualifiers are ignored
  * the result of `P1 R P2` depends on relative locations of `*P1` and `*P2` in
    the address space
    * if `P1` and `P2` point to the same simple object, `P1` and `P2` compare
      equal
    * if `P1` and `P2` point to members of the same structure, then `P1`
      compare higher than `P2` if and only if `*P1` is declared later in the
      structure than `*P2`
    * if `P1` and `P2` point to members of the same union, `P1` and `P2`
      compare equal
    * if `P1` and `P2` point to members of an array, then the corresponding
      subscripts are compared
    * if `P` points to the last member of an array, then `P + 1` compares
      higher than `P`
    * otherwise, `P1 R P2` is undefined

### Equality Expressions

Grammar for equality expressions:
```abnf
equality-expression = relational-expression
equality-expression /= equality-expression "==" relational-expression
equality-expression /= equality-expression "!=" relational-expression
```

* `==` (equal to) and `!=` (not equal to) follow the same rules as the
  relational operators
* a pointer may be compared to
  * a constant integral expression with value 0
  * a pointer to `void`

### Bitwise AND Expression

Grammar for a bitwise AND expression:
```abnf
AND-expression = equality-expression
AND-expression /= AND-expression "&" equality-expression
```

* in `E1 & E2`, `E1` and `E2` must be integral and undergo the usual arithmetic
  conversions
* `E1 & E2` evaluates to bitwise AND of `E1` and `E2`

### Bitwise Exclusive OR Expression

Grammar for a bitwise exclusive OR expression:
```abnf
exclusive-OR-expression = AND-expression
exclusive-OR-expression /= exclusive-OR-expression "^" AND-expression
```

* in `E1 ^ E2`, `E1` and `E2` must be integral and undergo the usual arithmetic
  conversions
* `E1 ^ E2` evaluates to bitwise exclusive OR of `E1` and `E2`

### Bitwise Inclusive OR Expression

Grammar for a bitwise inclusive OR expression:
```abnf
inclusive-OR-expression = exclusive-OR-expression
inclusive-OR-expression /= inclusive-OR-expression "|" exclusive-OR-expression
```

* in `E1 | E2`, `E1` and `E2` must be integral and undergo the usual arithmetic
  conversions
* `E1 | E2` evaluates to bitwise inclusive OR of `E1` and `E2`

### Logical AND Expression

Grammar for a logical AND expression:
```abnf
logical-AND-expression = inclusive-OR-expression
logical-AND-expression /= logical-AND-expression "&&" inclusive-OR-expression
```

* in `E1 && E2`, each of `E1` and `E2` must have arithmetic type or be a
  pointer
* `E1 && E2` evaluates to 1 if both `E1` and `E2` compare unequal to 0;
  otherwise `E1 && E2` evaluates to 0
* the type of the result of `E1 && E2` is `int`
* `E1 && E2` is evaluated as follows:
  1. `E1` is evaluated, including all side effects
  1. if `E1 == 0`, then `E1 && E2 == 0`
  1. `E2` is evaluated, including all side effects
  1. if `E2 == 0`, then `E1 && E2 == 0`
  1. `E1 && E2 == 1`

### Logical OR Expression

Grammar for a logical OR expression:
```abnf
logical-OR-expression = logical-AND-expression
logical-OR-expression /= logical-OR-expression "||" logical-AND-expression
```

* in `E1 || E2`, each of `E1` and `E2` must have arithmetic type or be a
  pointer
* `E1 || E2` evaluates to 1 if `E1` or `E2` compare unequal to 0; otherwise
  `E1 || E2` evaluates to 0
* the type of the result of `E1 && E2` is `int`
* `E1 || E2` is evaluated as follows:
  1. `E1` is evaluated, including all side effects
  1. if `E1 != 0`, then `E1 || E2 == 1`
  1. `E2` is evaluated, including all side effects
  1. if `E2 != 0`, then `E1 || E2 == 1`
  1. `E1 || E2 == 0`

### Conditional Expression

Grammar for a conditional expression:
```abnf
conditional-expression = logical-OR-expression
conditional-expression /= logical-OR-expression "?" expression ":" conditional-expression
```

* `E1 ? E2 : E3` is evaluated as follows:
  1. `E1` is evaluated, including all side effects
  1. if `E1 != 0`, then `E2` is evaluated and returned as the result of the
     expression
  1. `E3` is evaluated and returned as the result of the expression
* the type of the result is determined as follows:
  * if `E2` and `E3` are both arithmetic, then the usual arithmetic conversions
    are performed to bring them to a common type and that is the type of the
    result
  * if both `E2` and `E3` are `void`, or structures or unions of the same type,
    or pointers to objects of the same type, the result has the common type
  * if one of `E2` and `E3` is a pointer and the other the constant 0, then the
    0 is converted to the pointer type and that is the type of the result
  * if one of `E2` and `E3` has type `void *` and the other is a pointer, the
    other is converted to `void *` and that is the type of the result
  * in the type comparison of pointers, any type qualifiers of `*E2` and `*E3`
    are insignificant
  * the result inherits qualifiers from both `E2` and `E3`

### Assignment Expressions

Grammar for assignment expressions:
```abnf
assignment-expression = conditional-expression
assignment-expression /= unary-expression assignment-operator assignment-expression

assignment-operator = "=" / "*=" / "/=" / "%=" / "+=" / "-=" / "<<=" / ">>=" / "&=" / "^=" / "|="
```

* in `E1 op= E2`
  * `E1` must be an l-value
  * `E1` must be modifiable
    * `E1` must not be an array or a function
    * `E1` must not have an incomplete type
  * `E1`'s type must not be qualified with `const`
    * if it is a structure or union, it must not have any member or submember
      qualified with `const`
* the value of `E1 op= E2` is the value stored in `E1` after the assignment has
  taken place
* the type of `E1 op= E2` is the type of `E1`
* in `E1 = E2`
  * the value of the object referred to by `E1` is replaced by the value of
    `E2`
  * one of the following must be true:
    * `E1` and `E2` have both arithmetic type; then `E2` is converted to the
      type of `E1` by the assignment
    * `E1` and `E2` are both structures or unions of the same type
    * `E1` (`E2`) is a pointer and `E2` (`E1`) has type `void *`
    * `E1` is a pointer, `E2` is a constant expression with value 0
    * `E1` and `E2` are both pointers to functions or objects whose types are
      the same except for the possible absence of `const` or `volatile` in `E2`
* `E1 op= E2` is equivalent to `E1 = E1 op (E2)` except that `E1` is evaluated
  only once

### Comma Expression

Grammar for a comma expression:
```abnf
expression = assignment-expression
expression /= expression "," assignment-expression
```

* `E1, E2` is evaluated as follows:
  1. `E1` is evaluated with all side effects
  1. the value of `E1` is discarded
  1. `E2` is evaluated
* the type and value of `E1, E2` is the type and value of `E2`

### Constant Expressions

Grammar for constant expressions:
```abnf
constant-expression = conditional-expression
```

* constant expressions may not contain
  * assignments
  * increment and decrement operators
  * function calls
  * comma operators
  except in an operand of `sizeof`
* if the constant expression is required to be integral
  * its operands must consist of integer, enumeration, character, and floating
    constants
  * casts must specify an integral type
  * any floating constants must be cast to an integer
  * any operand is permitted to `sizeof`
* in the constant expressions of initializers
  * the operands may be any type of constant
  * the unary `&` operator may be applied
    * to external or static objects
    * to external or static arrays subscripted with a constant expression
    * implicitly by appearance of unsubscripted arrays and functions
  * constant expressions must evaluate either to a constant or to the address
    of a previously declared external or static object plus or minus a constant
* in the integral constant expressions after `#if` are not permitted
  * `sizeof` expressions
  * enumeration constants
  * casts
