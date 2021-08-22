# Functions

Grammar for a function definition:
```abnf
function-definition = [ declaration-specifiers ] declarator [ declaration-list ] compound-statement
```

* for the *declaration-specifiers* part
  * the only storage class specifiers allowed are `extern` or `static`
  * the return type of a function may be an arithmetic type, a structure, a
    union, a pointer, or `void`, but not a function or an array
* for the *declarator declaration-list* part
  * the *declarator* must specify explicitly that the declared identifier has
    function type, i.e. it must contain one of the forms
    ```abnf
    direct-declarator =/ direct-declarator "(" parameter-type-list ")"
    direct-declarator =/ direct-declarator "(" [ identifier-list ] ")"
    ```
    where the *direct-declarator* is an identifier or a parenthesized
    identifier
    * the declared identifier must not achieve function type by means of a
      `typedef`
  * the first form is a new style function definition
    * the *parameter-type-list* declares its parameters, together with their
      types
      * each declarator in the *parameter-type-list* must contain an identifier
        * identifier names need not agree with the function prototype
        * if the function takes no parameters, the *parameter-type-list*
          consists solely of `void`
      * if the *parameter-type-list* ends with `, ...`, the function may be
        called with more arguments than parameters; such a function is called
        *variadic*
        * variadic functions must have at least one named parameter
    * the *declaration-list* part must be absent
  * the second form is the old style function definition
    * the *identifier-list* names the parameters
    * the *declaration-list* attributes types to them
      * only parameters named in the *identifier-list* must be declared
      * if a parameter has no declaration given, its type is taken to be `int`
      * initialization is not permitted
      * the only storage class specifier possible is `register`
  * in both forms of function definitions
    * the parameters are understood to be declared just after the beginning of
      the *compound-statement*
      * the same identifiers must not be redeclared there (they may be
        redeclared only in inner blocks)
    * in the declaration of parameters
      * a type *array of T* is adjusted to read *pointer to T*
      * a type *function returning T* is adjusted to read *pointer to function
        returning T*
      * in the first edition of ANSI standard, `float` is adjusted to read
        `double` (this is noticeable when a pointer to a parameter is generated
        within a function)

## The Funtion `main`

* the function `main` is the program entry point
* the value returned by the function `main` is the program exit code
