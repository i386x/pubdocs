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

The following text uses these auxiliary functions:
* *trunc(F)* truncates the fractional part of floating point value *F*
* *fround(N)* return the floating point value nearest to *N*

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
