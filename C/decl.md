# Declarations

Grammar:
```abnf
declaration = declaration-specifiers [ init-declarator-list ] ";"

declaration-specifiers = 1*( storage-class-specifier / type-specifier / type-qualifier )

init-declarator-list = init-declarator *( "," init-declarator )

init-declarator = declarator [ "=" initializer ]

declarator = [ pointer ] direct-declarator

direct-declarator = identifier
direct-declarator =/ "(" declarator ")"
direct-declarator =/ direct-declarator "[" [ constant-expression ] "]"
direct-declarator =/ direct-declarator "(" parameter-type-list ")"
direct-declarator =/ direct-declarator "(" [ identifier-list ] ")"

pointer = "*" *type-qualifier [ pointer ]

parameter-type-list = parameter-list [ "," "..." ]

parameter-list = parameter-declaration *( "," parameter-declaration )

parameter-declaration = declaration-specifiers declarator
parameter-declaration =/ declaration-specifiers [ abstract-declarator ]

identifier-list = identifier *( "," identifier )
```

* declaration gives an interpretation to identifier
* if a declaration also allocates a memory, it is called *definition*
* declaration must have at least one declarator or type specifier must declare
  a name of structure, union, or enumeration
* declarator syntax follows the syntax of expression that returns the object of
  the type given in declaration, formally
  * if *T D* is a declaration, where *T* is the type and *D* is the declarator,
    *h* is the isomorphism from declarators abstract syntax trees to
    expressions abstract syntax trees, and *E = h(D)* is the expression, then
    `E` returns the object of the type *T*
* let *T D* is a declaration, where *T* is the type and *D* is the declarator;
  identifier in *D* can be then defined inductively
  * if *D* is `identifier`, then *D* has type *T*
  * if *D* has form `"(" D1 ")"`, then identifier in *D1* has type *T*

## Types

Grammar for type specifier is:
```abnf
type-specifier = %x76.6F.69.64               ; void
type-specifier =/ %x63.68.61.72              ; char
type-specifier =/ %x73.68.6F.72.74           ; short
type-specifier =/ %x69.6E.74                 ; int
type-specifier =/ %x6C.6F.6E.67              ; long
type-specifier =/ %x66.6C.6F.61.74           ; float
type-specifier =/ %x64.6F.75.62.6C.65        ; double
type-specifier =/ %x73.69.67.6E.65.64        ; signed
type-specifier =/ %x75.6E.73.69.67.6E.65.64  ; unsigned
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

### Arithmetic Types

* `char` type
  * at least 8 bits
  * `char` is large enough to hold any character from the runtime character set
  * a value of stored character is equal to its code from the character set and
    its unsigned
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
* `unsigned` types arithmetic is modulo `2**n` (`**` denotes power), where `n`
  is the number of bits of the used type
* non-negative values stored in signed type is a subset of values stored in
  unsigned type and they are represented in the same way
* `float`, `double`, and `long double` can refer to the same type
  * `float` (single precision type) is usually 32 bit with 6 significant digits
    and a range from `10**-37` to `10**37`
  * `double` (double precision type) must be precise at least as `float`
  * `long double` must be precise at least as `double`

#### Enumerations

Enumeration types are unique `int` types representing a set of named integer
constants, called enumeration constants:
```abnf
enum-specifier = %x65.6E.75.6D [ identifier ] "{" enumerator-list [ "," ] "}"
enum-specifier =/ %x65.6E.75.6D identifier

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
* enumeration constant shares the same name space as variables
* enumeration names have the same meaning as the names of structures and unions
  except that incomplete enumeration types are not allowed

### Derived Types

From `void`, arithmetic types, and derived types can be derived arrays,
functions, pointers, structures, and unions.

#### Pointers

* in a declaration *T D*
  * if *D* has the form `"*" Q D1`, where *Q* is the list of type qualifiers
    and
  * identifier in *T D1* has type *m T*, where *m* is the type modifier (e.g.
    array, pointer, function)
  then the type of identifier in *D* is *m Q pointer to T*
* *Q* relates to pointer, not to referred object

#### Arrays

* in a declaration *T D*
  * if *D* has the form `D1 "[" [ N ] "]"`, where *N* is a constant expression
    which value must be of integer type and greater than 0 and
  * identifier in *T D1* has type *m T*, where *m* is the type modifier
  then the type of identifier in *D* is *m array of T*
* if *N* is omitted, the array type is incomplete
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

#### Functions

##### New Style Declarations

* in a declaration *T D*
  * if *D* has the form `D1 "(" P ")"`, where *P* is the type parameters list
    and
  * identifier in *T D1* has type *m T*, where *m* is the type modifier
  then the type of identifier in *D* is *m function with parameters P and
  return type T*
* *P* determines types of parameters
* for functions without parameters, *P* is `void`
* if *P* ends with `...`, more parameters than those explicitly specified in
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
  then the type of identifier in *D* is *m function with unspecified parameters
  and return type T*
* declaration gives no hints about parameters types
* *I* can be used only if the function declarator is a part of function
  definition as a function header

#### Structures and Unions

* structure is an object made from the sequence of named elements, called
  members of the structure, that have different types
* union is defined like structure, but all its members have the same memory
  address

Declarations of structures and unions is summed up by the following grammar:
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
  `identifier` as the name of a structure or union determined by
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
  * it is interpreted as integer object which size is the number of specified
    bits
  * the layout of neighbor members depends on implementation
  * if the first of two neighboring bit fields is too large, it can be split or
    padded
  * unnamed bit field of the size 0 enforces padding
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
```

* type qualifiers can be used with any type specifier
* `const` means that the value of the object remains the same for the entire
  object lifetime
* `volatile` denotes that the object has important optimization properties;
  usually optimizations are not allowed with volatile objects

### Type Names

Grammar for type names:
```abnf
type-name = specifier-qualifier-list [ abstract-declarator ]

abstract-declarator = pointer
abstract-declarator =/ [ pointer ] direct-abstract-declarator

direct-abstract-declarator = "(" abstract-declarator ")"
direct-abstract-declarator =/ [ direct-abstract-declarator ] "[" [ constant-expression ] "]"
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

* *object* (variable) is a memory place characterized by its type and storage
  class
* type gives the meaning to the object's value

### Object Initialization

Grammar for initializers:
```abnf
initializer = assignment-expression
initializer =/ "{" initializer-list [ "," ] "}"

initializer-list = initializer *( "," initializer )
```

* initializer gives to object its initial value
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
* structures can be initialized
  * by the simple expression of the same type
  * by the list of initializers of their members following the given order and
    enclosed between `{` and `}`
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
* `{` and `}` can be omitted from the nested aggregate initializer; the
  initialization of the nested aggregate consumes only the required number of
  elements and the rest is leave for the initialization of the remaining
  aggregates
  * e.g. `int a[2][2] = { { 1, 2 }, { 3, 4 } };` has the same effect as
    `int a[2][2] = { 1, 2, 3, 4 };`

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
* storage class determines the storage duration of the object
* two storage classes: automatic and static

### Automatic Storage Class

* declaration of automatic object is also its definition
* automatic objects are local in the given block and they are disposed during
  the exit from the block
* if no storage class is specified besides `auto` specifier, declarations
  inside of block make automatic objects
* objects declared with `register` are automatic and should be stored in
  processor registers
* objects with `register` specifier are not addressable

### Static Storage Class

* inside function, a declaration containing `static` specifier is also a
  definition
* static objects can be local in the given block or external for all blocks
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
  * `typedef`ed names
  * enumeration constants
  * labels
  * names of structures, unions, and enumerations
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
* the scope of the name of the structure, union, or enumeration and the scope
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
  identifier with internal linkage refer to the same subject unique in the
  entire translation unit
* all declarations of the same object or function identifier with the external
  linkage refer to the same subject shared with entire program
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

* two list of type specifiers are considered the same if their sets of type
  specifiers are the same
* structures, unions, and enumerations with different or no names are
  considered distinct
* two types are considered the same if, after `typedef` names substitutions and
  identifiers from function parameters omission, they have same abstract
  declarators; arrays sizes and functions parameters types are significant

## Declarations Equivalence

Two declarations are considered equivalent:
* if their types are equivalent
* if one type is incomplete type of structure, union, or enumeration and the
  other type is the corresponding complete type with the same name
* if one type is incomplete type of array and the other type is complete type
  of array and their elements have equivalent types
* if one type is a function declared in the old way and the other type is the
  corresponding function declared in the new way

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
  definitions of the same object become one definition with the initializer 0
* every object must have exactly one definition
  * for objects with internal linkage, this applies to the translation unit
  * for objects with external linkage, this applies to the program