# Declarations

Grammar:
```abnf
declaration = declaration-specifiers [ init-declarator-list ] ";"

; before C99
declaration-specifiers = 1*( storage-class-specifier / type-specifier / type-qualifier )
; C99
declaration-specifiers = 1*( storage-class-specifier / type-specifier / type-qualifier / function-specifier )

init-declarator-list = init-declarator *( "," init-declarator )

init-declarator = declarator [ "=" initializer ]

declarator = [ pointer ] direct-declarator

direct-declarator = identifier
direct-declarator =/ "(" declarator ")"
; before C99
direct-declarator =/ direct-declarator "[" [ constant-expression ] "]"
; C99
direct-declarator =/ direct-declarator "[" *type-qualifier [ assignment-expression ] "]"
; C99
direct-declarator =/ direct-declarator "[" %x73.74.61.74.69.63 *type-qualifier assignment-expression "]"   ; static
; C99
direct-declarator =/ direct-declarator "[" 1*type-qualifier %x73.74.61.74.69.63 assignment-expression "]"  ; static
; C99
direct-declarator =/ direct-declarator "[" *type-qualifier "*" "]"
direct-declarator =/ direct-declarator "(" parameter-type-list ")"
direct-declarator =/ direct-declarator "(" [ identifier-list ] ")"

pointer = "*" *type-qualifier [ pointer ]

parameter-type-list = parameter-list [ "," "..." ]

parameter-list = parameter-declaration *( "," parameter-declaration )

parameter-declaration = declaration-specifiers declarator
parameter-declaration =/ declaration-specifiers [ abstract-declarator ]

identifier-list = identifier *( "," identifier )

; C99
function-specifier = %x69.6E.6C.69.6E.65  ; inline
```

* declaration gives an interpretation to identifier
* if a declaration also reserves a storage, it is called *definition*
* declaration must have at least one declarator or type specifier must declare
  a structure tag, a union tag, or an enumeration member
* declarator syntax resembles the syntax of the expression that returns the
  object of the specified type, formally
  * if *T D* is a declaration, where *T* is the type and *D* is the declarator,
    *h* is the isomorphism from declarators abstract syntax trees to
    expressions abstract syntax trees, and *E = h(D)* is the expression, then
    `E` returns the object of the type *T*
* let *T D* is a declaration, where *T* is the type and *D* is the declarator;
  the type of identifier in *D* can be then defined inductively
  * if *D* is `identifier`, then *D* is of the type *T*
  * if *D* has form `"(" D1 ")"`, then identifier in *D1* is of the type *T*

## Types

Grammar for type specifier is:
```abnf
type-specifier = %x76.6F.69.64                     ; void
type-specifier =/ %x63.68.61.72                    ; char
type-specifier =/ %x73.68.6F.72.74                 ; short
type-specifier =/ %x69.6E.74                       ; int
type-specifier =/ %x6C.6F.6E.67                    ; long
type-specifier =/ %x66.6C.6F.61.74                 ; float
type-specifier =/ %x64.6F.75.62.6C.65              ; double
type-specifier =/ %x73.69.67.6E.65.64              ; signed
type-specifier =/ %x75.6E.73.69.67.6E.65.64        ; unsigned
type-specifier =/ %x5F.42.6F.6F.6C                 ; _Bool (C99)
type-specifier =/ %x5F.43.6F.6D.70.6C.65.78        ; _Complex (C99)
type-specifier =/ %x5F.49.6D.61.67.69.6E.61.72.79  ; _Imaginary (C99)
type-specifier =/ struct-or-union-specifier
type-specifier =/ enum-specifier
type-specifier =/ typedef-name

typedef-name = identifier
```

* `void` denotes the empty set of values; it is used as a return type for
  functions returning no values
* only one type specifier per declaration is allowed, except the following
  cases:
  * with `int`, at most one of the following can be used: `short`, `long`
  * `long` can be used with `double`
  * with `int`, `short`, `long`, and `char`, at most one of the following can
    be used: `signed`, `unsigned`
* `signed` and `unsigned` are abbreviations for `int` and `unsigned int`,
  respectively
* `signed` is meaningful only with `char`; with other types it is redundant
* *old rule, not in C99*: the implicit type specifier is `int`
* (*C99*) if there is a `long` but not `double`, the second `long` can be used
* (*C99*) with `float` and `double`, either `_Complex` or `_Imaginary` can be
  used
* (*C99*) in each declaration or function definition, at least one type
  specifier shall be given in the declaration specifiers
* (*C99*) in each `struct` declaration and type name, at least one type
  specifier shall be given in the specifier-qualifier list

### Arithmetic Types

* `_Bool` type (*C99*)
  * unsigned integral type
  * can hold only 0 or 1
* `char` type
  * at least 8 bits
  * `char` is large enough to hold any character from the execution character
    set
  * a value of stored character is equal to its code from the character set and
    its non-negative
    * it holds for machine's standard printing characters
  * other values can be also stored in `char`, but their range and whether they
    are signed or unsigned depends on implementation
  * `unsigned char` and `signed char` are of the same size as `char`
* `short int` type
  * at least 16 bits
  * signed
* `int` type
  * signed
  * reflects the integer type native for the host architecture
  * its size is greater or equal to the size of `short int`
* `long int` type
  * at least 32 bits
  * signed
  * its size is greater or equal to the size of `int`
* `long long int` type (*C99*)
  * at least 64 bits
  * signed
  * its size is greater or equal to the size of `long int`
* `unsigned` types arithmetic is modulo `2**n` (`**` denotes power), where `n`
  is the number of bits of the used type representation
* non-negative values stored in signed type is a subset of values stored in
  unsigned type and they are represented in the same way
* `float`, `double`, and `long double` can refer to the same type
  * `float` (single precision type) is usually 32 bit with 6 significant digits
    and a range from `10**-37` to `10**37`
  * `double` (double precision type) must be precise at least as `float`
  * `long double` must be precise at least as `double`
* (*C99*) besides the standard integer types, the implementation may provide
  also *extended integer types*
* (*C99*) `float _Complex`, `double _Complex`, and `long double _Complex` are
  three types representing complex numbers
  * arithmetic types which are not complex types are called real types
  * each complex type has the same representation and alignment requirements as
    an array containing exactly two elements of the corresponding real type
    * the first element is equal to the real part
    * the second element is equal to the imaginary part
* (*C99*) optionally, `float _Imaginary`, `double _Imaginary`, and
  `long double _Imaginary` may also be provided
  * they are representing the imaginary part of the complex number
  * the difference between imaginary and ordinary floating-point types is in
    arithmetic conversions

#### Enumerations

Enumeration types are unique `int` types representing a set of named integer
constants, called enumeration constants:
```abnf
enum-specifier = %x65.6E.75.6D [ identifier ] "{" enumerator-list [ "," ] "}"  ; enum
enum-specifier =/ %x65.6E.75.6D identifier                                     ; enum

enumerator-list = enumerator *( "," enumerator )

enumerator = enumeration-constant [ "=" constant-expression ]
```

* the value of enumeration constant is inductively defined as follows
  1. if the enumerator is of the form `A = k`, the enumeration constant `A` has
     value *k*
  1. if the enumerator is of the form `A`, the enumeration constant `A` has
     value of its predecessor increased by one
  1. if the enumerator is of the form `A` and it has no predecessor, the
     enumeration constant `A` has value 0
* enumeration constants share the same scope as variables
* enumeration tags have the same meaning as the structure and union tags except
  that incomplete enumeration types are not allowed
* compilers need not check whether the value stored in an object declared to
  have enumeration type is valid for that type

### Derived Types

From `void`, arithmetic types, and derived types can be derived arrays,
functions, pointers, structures, and unions.

#### Pointers

* in a declaration *T D*
  * if *D* has the form `"*" Q D1`, where *Q* is the list of type qualifiers
    and
  * identifier in *T D1* is of the type *m T*, where *m* is the type modifier
    (e.g. array, pointer, function)
  * then the type of the identifier of *D* is *m Q pointer to T*
* *Q* applies to the pointer, not to the object to which the pointer points
* arithmetic and pointer types are collectively called *scalar types*

#### Arrays

* in a declaration *T D*
  * if *D* has the form `D1 "[" [ N ] "]"`, where *N*, specifying the number of
    elements (the size) of the array, is a constant expression that must
    evaluate to positive integer, and
  * identifier in *T D1* is of the type *m T*, where *m* is the type modifier
  * then the type of identifier of *D* is *m array of T*
  * (*C99*) if *D* has the one of the forms
    * `D1 "[" *type-qualifier [ assignment-expression ] "]"`
    * `D1 "[" "static" *type-qualifier assignment-expression "]"`
    * `D1 "[" 1*type-qualifier "static" assignment-expression "]"`
    * `D1 "[" *type-qualifier "*" "]"`
    where *assignment-expression* shall have an integer type and after
    evaluation be greater than zero, and
  * (*C99*) identifier in *T D1* is of the type *d T*, where *d* is the derived
    declarator type list
  * (*C99*) then the type of identifier of *D* is *d array of T*
* if *N* is missing, the array type is incomplete
* array elements can be objects of arithmetic types, pointers, structures,
  unions, and arrays
* array elements must have complete types
* incomplete array type can be completed by another, complete, declaration or
  by initialization
* expression `arr[i]` is equivalent to `*(arr + i)`
  * elements of arrays are indexed starting with 0
  * the address of array is the address of its first element
* arrays are stored by rows
  * given a declaration of array `int arr[N][M][K]`, the expression
    `arr[i][j][k]` is equivalent to `*(arr + i*M*K + j*K + k)`
* (*C99*) the optional type qualifiers and `static` shall appear only in
  * a declaration of a function parameter with an array type
  * the outermost array type derivation
* (*C99*) the array type is not a variable length array type if
  * the size is an integer constant expression and
  * the element type has a known constant size
* (*C99*) only identifier with no linkage and both block or function prototype
  scope declaring an object with no static storage duration shall have a
  variable length array type
* (*C99*) the size of each instance of a variable length array type remains the
  same during its lifetime
* (*C99*) if the size is `*`, the array type is a variable length array type of
  unspecified size and its type is considered complete
  * can only be used in declarations with function prototype scope
* (*C99*) in a declaration at function prototype scope
  * *assignment-expression* that is not a constant integer expression is
    treated as if it were replaced by `*`

#### Functions

##### New Style Declarations

* a new style declaration is also called *function prototype*
* in a declaration *T D*
  * if *D* has the form `D1 "(" P ")"`, where *P* is the type parameters list
    and
  * identifier in *T D1* is of the type *m T*, where *m* is the type modifier
  * then the type of identifier of *D* is *m function with arguments P and a
    return type T*
* *P* determines the types of parameters
* for functions without parameters, *P* is `void`
* if *P* ends with `, ...`, more arguments than those explicitly specified in
  *P* can be passed to the function
* parameters that are declared as arrays and functions are converted to
  pointers (see [Functions](func.md))
* the only storage class specifier that is allowed in parameters declarations
  is `register`
* if the function declarator is not directly followed by the function
  definition
  * `register` is ignored in parameters declarations
  * identifiers in parameters declarations become out of scope after the end of
    the declarator

##### Old Style Declarations

* in a declaration *T D*
  * if *D* has the form `D1 "(" [ I ] ")"`, where *I* is the list of
    identifiers and
  * identifier in *T D1* has type *m T*, where *m* is the type modifier
  * then the type of identifier of *D* is *m function with unspecified
    arguments and a return type T*
* declaration gives no information about parameters types
* *I* can be used only if the function declarator is a part of function
  definition as a function header

##### `inline` Specifier (C99)

* shall be used only in the declaration of an identifier for a function
* an inline definition of a function with external linkage shall not contain
  * a definition of a modifiable object with static storage duration
  * a reference to an identifier with internal linkage
* should be placed before return type and after storage class specifier
* suggests that calls to the function be as fast as possible (it is
  implementation-defined whether or not such suggestions are effective)
* an inline function can be any function with internal linkage
* for a function with external linkage
  * if it is declared with an `inline` specifier
    * it shall be also defined in the same translation unit
  * if all of the file scope declarations for a function in a translation unit
    have `inline` without `extern`
    * the definition in that translation unit is an inline definition
  * an inline definition
    * does not provide an external definition for the function
    * does not forbid an external definition in another translation unit
    * provides an alternative to an external definition
      * may be used by a translator to implement any call to the function in
        the same translation unit

#### Structures and Unions

* structure is an object made from the sequence of named elements of various
  types, called members
* union is defined like structure, but all its members are placed on the same
  storage location
* array and structure types are collectively called *aggregate types*

Declarations of structures and unions are summed up by the following grammar:
```abnf
struct-or-union-specifier = struct-or-union [ identifier ] "{" struct-declaration-list "}"
struct-or-union-specifier =/ struct-or-union identifier

struct-or-union = %x73.74.72.75.63.74  ; struct
struct-or-union =/ %x75.6E.69.6F.6E    ; union

struct-declaration-list = 1*struct-declaration

struct-declaration = specifier-qualifier-list struct-declarator-list ";"

specifier-qualifier-list = 1*( type-specifier / type-qualifier )

struct-declarator-list = struct-declarator *( "," struct-declarator )

struct-declarator = declarator
struct-declarator =/ [ declarator ] ":" constant-expression
```

* a member of structure declared as `[ declarator ] ":" constant-expression` is
  called *bit field*
* a type specifier of the form
  `struct-or-union identifier "{" struct-declaration-list "}"` declares
  `identifier` as the tag of a structure or union determined by
  `struct-declaration-list`
* the following declaration of the form `struct-or-union identifier` in the
  same scope refers to the same type
* a type specifier of the form `struct-or-union identifier`, where `identifier`
  was not yet declared, is considered as an incomplete type specification
* incomplete structure or union types can be used in situations where their
  size is not important to know, e.g. in declarations, pointer specifications,
  or in `typedef`s
* an incomplete type of structure or union becomes complete when
  `struct-or-union identifier "{" struct-declaration-list "}"` appears
* a type of structure or union is considered incomplete, in the case it was not
  declared before, also between `{` and `}`
* structure and union cannot contain a member of incomplete type
  * a recursive structure or union can be made via pointers, since the pointer
    to incomplete type is considered valid, complete, type
* declaration `struct-or-union identifier ";"` make `identifier` a new
  incomplete type of structure or union in the given scope, even if
  `identifier` has been declared before
* a type specifier of the form
  `struct-or-union "{" struct-declaration-list "}"` makes a unique type that
  can be referred only within the same declaration
* a bit field member
  * has `int` or `unsigned int` type (depends on implementation)
    * (*C99*) has also `_Bool` type
  * it is interpreted as integer object which size is the number of specified
    bits
  * the layout of adjacent members depends on implementation
  * if the second of two adjacent bit fields don't fit into cell occupied by
    the first bit field, the second field can be split between the cells or the
    first bit field cell can be padded
  * unnamed bit field of the size 0 enforces the padding
* addresses of structure members are ascending in the order of their
  declarations
* non-bit-field structure members are usually aligned, depending on their types
* given a declaration `struct { int i; double d; } *p;`, the expression
  `(int *)p == &(p->i)` is 1
* given a declaration `union { int i; double d; } *p`, the expression
  `(int *)p == &(p->i) && (double *)p == &(p->d)` is 1
* member of union should be accessed only after value has been assigned to it
* if union contains only structures and all of these structures begin with the
  same sequence of members then these members can be accessed from any
  structure within the union

### Type Qualifiers

Grammar for type qualifiers:
```abnf
type-qualifier = %x63.6F.6E.73.74            ; const
type-qualifier =/ %x76.6F.6C.61.74.69.6C.65  ; volatile
type-qualifier =/ %x72.65.73.74.72.69.63.74  ; restrict (C99)
```

* type qualifiers can be used with any type specifier
* `const` means that the value of the object remains the same for the entire
  object lifetime
* `volatile` denotes that the object has important optimization properties;
  usually optimizations are not allowed with volatile objects
* (*C99*) `restrict` marks a pointer as restrict-qualified
  * introduced for optimization purposes
  * only valid for pointer types derived from object or incomplete types
  * an object that is accessed through a restrict-qualified pointer should not
    be accessed by any other pointer (all accesses to that object use, directly
    or indirectly, the value of that particular pointer)
  * restrict-qualified pointer should not change its value during its lifetime
  * the value of a restrict-qualified pointer should not be carried out of the
    block, except the case it is declared in a block when that block finishes
    execution

### Type Names

Grammar for type names:
```abnf
type-name = specifier-qualifier-list [ abstract-declarator ]

abstract-declarator = pointer
abstract-declarator =/ [ pointer ] direct-abstract-declarator

direct-abstract-declarator = "(" abstract-declarator ")"
; before C99
direct-abstract-declarator =/ [ direct-abstract-declarator ] "[" [ constant-expression ] "]"
; C99
direct-abstract-declarator =/ [ direct-abstract-declarator ] "[" [ assignment-expression ] "]"
; C99
direct-abstract-declarator =/ [ direct-abstract-declarator ] "[" "*" "]"
direct-abstract-declarator =/ [ direct-abstract-declarator ] "(" [ parameter-type-list ] ")"
```

* used in typecast expressions, in declaration of types of parameters in
  function declarations, and as `sizeof` argument

### `typedef`

* storage class specifier `typedef` defines type aliases (identifiers), called
  *typedef names*
* typedef names can be overridden in inner scope declarations; when doing this,
  type specifiers must not be omitted

## Objects (Variables)

* *object* (variable) is a storage location characterized by its type and
  storage class
* type gives the meaning to the object's value

### Object Initialization

Grammar for initializers:
```abnf
initializer = assignment-expression
initializer =/ "{" initializer-list [ "," ] "}"

; before C99
initializer-list = initializer *( "," initializer )
; C99
initializer-list = [ designation ] initializer *( "," [ designation ] initializer )

; C99
designation = 1*designator "="

; C99
designator = "[" constant-expression "]"
; C99
designator =/ "." identifier
```

* initializer gives to object its initial value
* each `"{" initializer-list [ "," ] "}"` has an associated *current object*
* static objects require initializers with constant expressions
* arrays and `register` or `auto` objects with initializers of the form
  `"{" C *( "," C ) [ "," ] "}"` require *C* to be constant expressions
* automatic objects initialized with a simple expression do not require this
  expression to be constant
* static objects are implicitly initialized to 0
* the initial value of uninitialized automatic objects is undefined
* pointers and arithmetic objects can be initialized by the simple expression,
  optionally surrounded by `{` and `}`
* arrays can be initialized
  * by the list of initializers of their elements enclosed between `{` and `}`
    * if the array size is not known, it is derived from the initializer, and
      the array type becomes complete
    * if the array size is given, the number of initializers must not exceed
      the number of array elements; if there are less initializers than
      elements, the rest of elements are initialized to 0
  * by string literals
    * a string literal is treated as a list of character constant initializers,
      including null character, enclosed between `{` and `}`
    * if the size of array is `N`, the length of string literal (terminating
      null character is not counted) must be less or equal to `N`
* structures can be initialized
  * by the simple expression of the same type
  * by the list of initializers of their members in order and enclosed between
    `{` and `}`
    * unnamed bit fields are ignored and left uninitialized
    * more initializers than members is not allowed
    * less initializers than members is allowed, the missing initializers are
      treated as 0
* unions can be initialized
  * by the simple expression of the same type
  * by the initializer of the first member of the union enclosed between `{`
    and `}`
* static unions cannot be initialized explicitly
* arrays and structures are called *aggregates*
* if a member of aggregate is an aggregate, the rules are applied recursively
* `{` and `}` can be omitted from the subaggregate initializer; the
  initialization of the subaggregate consumes only the required number of
  elements and the rest is leave for the initialization of the remaining
  aggregate members
  * e.g. `int a[2][2] = { { 1, 2 }, { 3, 4 } };` has the same effect as
    `int a[2][2] = { 1, 2, 3, 4 };`
* (*C99*) a designation causes the following initializer to begin
  initialization of the subobject described by the designator
  * initialization then continues forward in order, beginning with the next
    subobject after that described by the designator
  * a list of designators unambiguously determines a subobject to be
    initialized
    * `[N]` determines the *N*th element of an array
    * `.name` determines the *name* member of a structure or union
* (*C99*) if a designator has the form `"[" constant-expression "]"`
  * the current object shall have array type
  * the *constant-expression* shall be an integer constant expression
    * for arrays of known sizes the value shall be a non-negative integer less
      than the size of the array
    * for arrays of unknown sizes this can be any non-negative value
* (*C99*) if a designator has the form `"." identifier`
  * the current object shall have structure or union type
  * the *identifier* shall be name of a member of that type

## Storage Class

Grammar for storage class specifier:
```abnf
storage-class-specifier = %x61.75.74.6F               ; auto
storage-class-specifier =/ %x72.65.67.69.73.74.65.72  ; register
storage-class-specifier =/ %x73.74.61.74.69.63        ; static
storage-class-specifier =/ %x65.78.74.65.72.6E        ; extern
storage-class-specifier =/ %x74.79.70.65.64.65.66     ; typedef
```

* only one storage class specifier per declaration is allowed
* implicit storage class specifier is:
  * for objects declared inside function: `auto`
  * for functions declared inside function: `extern`
  * for objects and functions declared outside function: static with external
    linkage
* storage class determines the storage lifetime of the object
* two storage classes: automatic and static

### Automatic Storage Class

* declaration of automatic object is also its definition
* automatic objects are local in the given block and they are disposed during
  the exit from the block
* if no storage class is specified or `auto` specifier is used, declarations
  inside a block make automatic objects
* objects declared with `register` are automatic and should be stored in
  machine's registers
* objects with `register` specifier are not addressable

### Static Storage Class

* inside function, a declaration containing `static` specifier is also a
  definition
* static objects can be local in the given block or external to all blocks
* their values persist the entering to or exiting from the block
* inside the block they are declared with `static` keyword
* objects outside of all block are always static
  * if they are declared with `static` keyword, their linkage is internal
  * if they are declared with no or `extern` keyword, their linkage is external

## Scope

* scope determines the visibility of the object
* lexical scope determines the visibility of the identifier in the translation
  unit
* identifiers can belong to the several non-overlapping name spaces:
  * objects
  * functions
  * `typedef` names
  * `enum` constants
  * labels
  * tags of structures, unions, and enumerations
  * members of structures or unions
* in external declarations, the scope of the object or function begins with the
  end of their declarators and ends with the end of the translation unit
* the scope of function parameter in the function definition begins at the
  start of the function block and ends at the end of the function block
* the scope of function parameter in the function declaration ends with the end
  of the function declarator
* in block, the scope of identifier begins with the end of declarator and ends
  with the end of the block
* the scope of the label is the entire function
* the scope of the tag of the structure, union, or enumeration and the scope
  of the enumeration constant begins with their occurrence in the type
  specifier and ends
  * with the end of translation unit (for the external declarations)
  * with the end of block (for the declarations inside functions)
* in block, if the identifier is explicitly declared, it overrides other
  declarations of the same identifier until the end of the block

## Linkage

* linkage determines whether objects or functions of the same name, but from
  different scopes, are identical
* in the translation unit, all declarations of the same object or function
  identifier with internal linkage refer to the same thing unique in the entire
  translation unit
* all declarations of the same object or function identifier with the external
  linkage refer to the same thing shared with entire program
* the first external identifier declaration
  * if `static` is used, its linkage is internal
  * otherwise, its linkage is external
* identifier has no linkage
  * if it is declared in block without `extern` specifier
  * such an identifier is unique
* if the identifier is declared in block with `extern` specifier
  * if the external or internal linkage of the identifier is active outside the
    block
    * the linkage is inherited from the prior declaration
  * otherwise, the linkage is external

## Types Equivalence

* two list of type specifiers are considered equivalent if their sets of type
  specifiers are the same
* structures, unions, and enumerations with different or no tags are considered
  distinct
* two types are considered the same if, after `typedef` names substitutions and
  identifiers from function parameters omission, they have same abstract
  declarators; array sizes and function parameters types are significant

## Declarations Equivalence

Two declarations are considered equivalent:
* if their types are equivalent
* if one type is incomplete type of structure, union, or enumeration and the
  other type is the corresponding complete type with the same tag
* if one type is incomplete type of array and the other type is complete type
  of array and their elements have equivalent types
* if one type is a function declared in the old-style and the other type is the
  otherwise identical function declared in the new-style

## External Declarations

* includes functions and objects
* *external* means outside of function
* declarations on a translation unit level are external and are visible until
  the end of the translation unit
* storage class of external declaration can be `static` or `extern`
* same identifier can be declared multiple times per translation unit only if
  the following conditions are met:
  * their declarations are equivalent
  * they are of the same linkage
  * at most one definition of identifier exists
* if the first external declaration has `static` specifier, the identifier has
  internal linkage; otherwise, it has external linkage
* if the external definition of the object has initializer, it is definition
* external declaration of the object is called *tentative definition* if it has
  no initializer and no `extern` specifier
* if the translation unit contains the object definition, all tentative
  definitions of the same object are considered redundant
* if the translation unit contains no object definition, all tentative
  definitions of the same object become a single definition with the
  initializer 0
* every object must have exactly one definition
  * for objects with internal linkage, this applies to the translation unit
  * for objects with external linkage, this applies to the program
