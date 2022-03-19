# Lexical Elements

After a C source file is preprocessed, it is scanned to produce a sequence of
lexical elements, called *tokens*. A token is a terminal symbol of C grammar.
There are five types of tokens:
```abnf
token = keyword
token =/ identifier
token =/ constant
token =/ string-literal
token =/ punctuator
```

## Comments and White Space Characters

A comment is everything between `/*` and `*/`.

(*C99*) Also everything between `//` and a new-line character is a comment.

As a white space character is considered:
```abnf
WS = %x07 / %x08 / %x09 / %x0A / %x0B / %x0C / %x0D / %x20  ; \a, \b, \t, \n, \v, \f, \r, " "
```

Comments and white space characters are ignored.

## Universal Character Names

Grammar:
```abnf
UCNAME = %x5C.75 4XDIGIT   ; \u (C99)
UCNAME =/ %x5C.55 8XDIGIT  ; \U (C99)
```

* `\Unnnnnnnn` designates the character from the ISO/IEC 10646 (Unicode)
  character set with the code point `nnnnnnnn`
* `\unnnn` is an abbreviation for `\U0000nnnn`
* `nnnn` should be neither in the range `0000` through `00A0` (except `0024`,
  `0040`, or `0060`) nor in the range `D800` through `DFFF` inclusive
* universal character names can be used in identifiers, character constants,
  and string literals

## Keywords

A `keyword` is one of:
```C
auto            double        int             struct
break           else          long            switch
case            enum          register        typedef
char            extern        return          union
const           float         short           unsigned
continue        for           signed          void
default         goto          sizeof          volatile
do              if            static          while
```

Additionally, these names are considered keywords in *C99*:
```C
_Bool
_Complex
_Imaginary
```

## Identifiers

Grammar:
```abnf
identifier = ILETTER *( ILETTER / DIGIT )

ILETTER = LETTER / UCNAME
DIGIT  = %x30-39                  ; 0-9
LETTER = "_" / %x41-5A / %x61-7A  ; _, A-Z, a-z
```

* their names are case sensitive
* identifiers serve as names of macros, variables (objects), type names,
  members, tags, labels, and functions
* for internal identifiers is significant the first 31 characters
* for external identifiers, it is 6 (case sensitivity may be not considered)

### `__func__` Identifier (C99)

* shall be implicitly declared by the translator as if the declaration
  ```C
  static const char __func__[] = "<function-name>";
  ```
  appeared immediately after the opening brace of each function definition,
  where `<function-name>` is the name of the lexically-enclosing function
* `<function-name>` is encoded as if it had been written in the source
  character set and then translated into the execution character set

## Constants

There are four types of constants:
```abnf
constant = integer-constant
constant =/ floating-constant
constant =/ enumeration-constant
constant =/ character-constant
```

### Integer Constants

Grammar:
```abnf
integer-constant = decimal-constant [ integer-suffix ]
integer-constant =/ octal-constant [ integer-suffix ]
integer-constant =/ hexadecimal-constant [ integer-suffix ]

decimal-constant     = NZDIGIT *DIGIT
octal-constant       = "0" *ODIGIT
hexadecimal-constant = "0X" *XDIGIT

integer-suffix = "U" [ "L" ]
integer-suffix =/ "U" ( %x4C.4C / %x6C.6C )      ; LL/ll (C99)
integer-suffix =/ "L" [ "U" ]
integer-suffix =/ ( %x4C.4C / %x6C.6C ) [ "U" ]  ; LL/ll (C99)

NZDIGIT = %x31-39  ; 1-9
ODIGIT  = %x30-37  ; 0-7
XDIGIT  = DIGIT / "A" / "B" / "C" / "D" / "E" / "F"  ; 0-9 A-F a-f
```

* `U` or `u` suffix means that the integer is unsigned (its type is
  `unsigned int`, `unsigned long int`, or `unsigned long long int`)
* `L` or `l` suffix means that the integer is `long` or `long long` (its type
   is `long int`, `long long int`, `unsigned long int`, or
   `unsigned long long int`)
* `LL` or `ll` suffix means that the integer is `long long` (its type is
  `long long int` or `unsigned long long int`)
* if the integer constant is written as decimal without suffix, then its type
  is the first matching type from `int`, `long int`, `unsigned long int`
  (*C99*: `long long int`)
* if the integer constant is written as octal or hexadecimal without suffix,
  its type is the first matching from `int`, `unsigned int`, `long int`,
  `unsigned long int`, `long long int`, `unsigned long long int`
* (*C99*) if the integer constant cannot be represented by any type in its list,
  it may have an extended integer type; if it has no extended integer type, it
  has no type

### Floating Constants

Grammar:
```abnf
floating-constant = fractional-constant [ exponent-part ] [ "F" / "L" ]
floating-constant =/ 1*DIGIT exponent-part [ "F" / "L" ]
floating-constant =/ "0X" x-fractional-constant b-exponent-part [ "F" / "L" ]  ; C99
floating-constant =/ "0X" 1*XDIGIT b-exponent-part [ "F" / "L" ]               ; C99

fractional-constant = *DIGIT "." 1*DIGIT
fractional-constant =/ 1*DIGIT "."

x-fractional-constant = *XDIGIT "." 1*XDIGIT
x-fractional-constant =/ 1*XDIGIT "."

exponent-part = "E" [ "+" / "-" ] 1*DIGIT

b-exponent-part = "P" [ "+" / "-" ] 1*DIGIT
```

* floating constant without suffix has type `double`
* with `F` or `f` suffix, it has type `float`
* with `L` or `l` suffix, it has type `long double`
* (*C99*) hexadecimal floating constants
  * the fractional part is interpreted as a hexadecimal rational number, e.g
    `AB.C == A*16 + B + C*(1./16)`
  * in the exponent part, the base is 2 and the exponent is interpreted as a
    decimal number; thus `0x.4p3` is equal to `4./16 * 8` which is `2.0`

### Enumeration Constants

Grammar:
```abnf
enumeration-constant = identifier
```

Enumeration constants have type `int`.

### Character Constants

Grammar:
```abnf
character-constant = [ %x4C ] %x27 1*c-char %x27  ; L?' c-char '

; Any source character except new line (\n), apostrophe (\'), and backslash (\\).
; For simplicity, as a source character is considered anything from exclamation
; mark (!) to tilde (~) plus white space characters. Depending on implementation,
; other characters can be also included.
c-char =  %x07-09 / %x0B-0D / %x20-26 / %x28-5B / %x5D-7E
c-char =/ escape-sequence

escape-sequence = simple-escape-sequence
escape-sequence =/ octal-escape-sequence
escape-sequence =/ hexadecimal-escape-sequence
escape-sequence =/ UCNAME

simple-escape-sequence = %x5C.27   ; \'
simple-escape-sequence =/ %x5C.22  ; \"
simple-escape-sequence =/ %x5C.3F  ; \?
simple-escape-sequence =/ %x5C.5C  ; \\
simple-escape-sequence =/ %x5C.61  ; \a
simple-escape-sequence =/ %x5C.62  ; \b
simple-escape-sequence =/ %x5C.66  ; \f
simple-escape-sequence =/ %x5C.6E  ; \n
simple-escape-sequence =/ %x5C.72  ; \r
simple-escape-sequence =/ %x5C.74  ; \t
simple-escape-sequence =/ %x5C.76  ; \v

octal-escape-sequence = "\" 1*3ODIGIT

hexadecimal-escape-sequence = %x5C.78 1*XDIGIT  ; \x XDIGIT+
```

* a value of character constant is equal to the numerical value of `c-char` in
  the used character set
* if the character constant contains more than one `c-char`s, its value depends
  on implementation
* if `c-char` is an escape sequence, its value is given by the following table:

  | Escape Sequence | Meaning | Value (ASCII) |
  | --------------- | ------- | ------------- |
  | `\'` | apostrophe | 39 |
  | `\"` | quotes | 34 |
  | `\?` | question mark | 63 |
  | `\\` | backslash | 92 |
  | `\a` | bell | 7 |
  | `\b` | backspace | 8 |
  | `\f` | form feed | 12 |
  | `\n` | line feed | 10 |
  | `\r` | carriage return | 13 |
  | `\t` | horizontal tab | 9 |
  | `\v` | vertical tab | 11 |
  | `\ooo` | octal escape sequence | `0ooo` |
  | `\xhh` | hexadecimal escape sequence | `0xhh` |
* if `char` is considered signed, `\ooo` and `\xhh` are sign-extended and
  typecast to `char`
* character constants prefixed with `L` are considered wide and their type is
  `wchar_t`

## String Literals

Grammar:
```abnf
string-literal = [ %x4C ] %x22 *s-char %x22  ; L?" s-char* "

; Any source character except new line (\n), quotes (\"), and backslash (\\).
s-char =  %x07-09 / %x0B-0D / %x20-21 / %x23-5B / %x5D-7E
s-char =/ escape-sequence
```

* a string literal `s` is declared as `static char s[]`, initialized by
  characters from `s` and terminated with `\0`
* adjacent string literals are concatenated into one string literal (their
  characters form a consecutive sequence terminated with `\0`)
* if `s` is a string literal prefixed with `L`, it is considered as wide and
  declared as `static wchar_t s[]`

## Punctuators

A `punctuator` is one of:
```C
[  ]  (  )  {  }  .  ->
++  --  &  *  +  -  ~  !
/  %  <<  >>  <  >  <=  >=  ==  !=  ^  |  &&  ||
?  :  ;  ...
=  *=  /=  %=  +=  -=  <<=  >>=  &=  ^=  |=
,  #  ##
```
