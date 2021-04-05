# Declarations

Grammar:
```abnf
declaration = declaration-specifiers [ init-declarator-list ] ";"

declaration-specifiers = 1*( storage-class-specifier / type-specifier / type-qualifier )

init-declarator-list = init-declarator *( "," init-declarator )

init-declarator = declarator [ "=" initializer ]
```

* declaration gives an interpretation to identifier
* if a declaration also allocates a memory, it is called *definition*
* declaration must have at least one declarator or type specifier must declare
  a name of structure, union, or enumeration

## Objects (Variables)

* *object* (variable) is a memory place characterized by its type and storage
  class
* type gives the meaning to the object's value

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
* enumeration types are unique `int` types representing a set of named integer
  constants

### Derived Types

From `void`, arithmetic types, and derived types can be derived arrays,
functions, pointers, structures, and unions.

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
