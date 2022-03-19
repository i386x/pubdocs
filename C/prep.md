# Preprocessor

A preprocessor recognizes these types of tokens:
```abnf
preprocessing-token = header-name
preprocessing-token =/ identifier
preprocessing-token =/ pp-number
preprocessing-token =/ character-constant
preprocessing-token =/ string-literal
preprocessing-token =/ punctuator
preprocessing-token =/ PPOTHER

header-name = "<" 1*h-char ">"
header-name =/ %x22 1*q-char %x22  ; "..."

pp-number = [ "." ] DIGIT *( DIGIT / ILETTER / ( "E" / "P" ) ( "+" / "-" ) "." )

; Any source character except new line (\n) and greater than (>).
h-char = %x07-09 / %x0B-0D / %x20-3D / %x3F-7E
; Any source character except new line (\n) and quotes (\").
q-char = %x07-09 / %x0B-0D / %x20-21 / %x23-7E

PPOTHER = "$" / "@" / "\" / "`"
```

The complete preprocessor grammar is:
```abnf
preprocessing-file = [ group ]

group = 1*group-part

group-part = if-section
group-part =/ control-line
group-part =/ text-line
group-part =/ "#" non-directive

if-section = if-group *elif-group [ else-group ] endif-line

if-group = "#" %x69.66 constant-expression %x0A [ group ]      ; if
if-group =/ "#" %x69.66.64.65.66 identifier %x0A [ group ]     ; ifdef
if-group =/ "#" %x69.66.6E.64.65.66 identifier %x0A [ group ]  ; ifndef

elif-group = "#" %x65.6C.69.66 constant-expression %x0A [ group ]  ; elif
else-group = "#" %x65.6C.73.65 %x0A [ group ]                      ; else
endif-line = "#" %x65.6E.64.69.66 %x0A                             ; endif

control-line = "#" %x69.6E.63.6C.75.64.65 pp-tokens %x0A                                                 ; include
control-line =/ "#" %x64.65.66.69.6E.65 identifier [ pp-tokens ] %x0A                                    ; define
control-line =/ "#" %x64.65.66.69.6E.65 identifier "(" [ identifier-list ] ")" [ pp-tokens ] %x0A        ; define
; C99
control-line =/ "#" %x64.65.66.69.6E.65 identifier "(" "..." ")" [ pp-tokens ] %x0A                      ; define
; C99
control-line =/ "#" %x64.65.66.69.6E.65 identifier "(" identifier-list "," "..." ")" [ pp-tokens ] %x0A  ; define
control-line =/ "#" %x75.6E.64.65.66 identifier %x0A                                                     ; undef
control-line =/ "#" %x6C.69.6E.65 pp-tokens %x0A                                                         ; line
control-line =/ "#" %x65.72.72.6F.72 [ pp-tokens ] %x0A                                                  ; error
control-line =/ "#" %x70.72.61.67.6D.61 [ pp-tokens ] %x0A                                               ; pragma
control-line =/ "#" %x0A

text-line = [ pp-tokens ] %x0A

non-directive = pp-tokens %x0A

pp-tokens = 1*preprocessing-token
```

* lines starting with `#` (can be preceded by white space characters)
  communicate with preprocessor
  * each line is analyzed individually
  * their effect lasts until the end of the translation unit
  * white space characters other than `\t` and `\n` have undefined behavior
    within `#` lines
* preprocessing is done in following steps:
  1. [trigraph sequences](#trigraph-sequences) are replaced by their
     equivalents
  1. if it is required by the operating system environment, new line characters
     are introduced between the lines of the source file
  1. each occurrence of `\\\n` sequence is deleted, thus splicing lines
  1. the program is split into tokens separated by white space characters
  1. comments are replaced by a single space
  1. preprocessing directives are obeyed
  1. [macros are expanded](#macro-definition-and-expansion)
  1. escape sequences in character constants and string literals are replaced
     by their equivalents
  1. adjacent string literals are concatenated

## Trigraph Sequences

Trigraph sequences and their meaning:
```C
??=   #                ??(   [                ??<   {
??/   \                ??)   ]                ??>   }
??'   ^                ??!   |                ??-   ~
```

* the character set of C source programs is
  * contained within 7-bit ASCII
  * a superset of the ISO 646-1983 Invariant Code Set
* trigraph sequences allow to write C programs also in the reduced character
  set

## Macro Definition and Expansion

* `#` `define` *identifier* *[ pp-tokens ]*
  * after this line, preprocessor replaces subsequent instances of *identifier*
    with the *[ pp-tokens ]*
  * leading and trailing white space around *pp-tokens* is discarded
  * if another `#define` line with the same *identifier* occurs, its
    *[ pp-tokens ]* must be same as the *[ pp-tokens ]* of the previous one;
    otherwise, the `#define` line is erroneous
* `#` `define` *identifier* `(` *[ identifier-list ]* `)` *[ pp-tokens ]*
  * define macro *identifier* with parameters given by the
    *[ identifier-list ]*
  * there must be no white space between *identifier* and `(`
  * leading and trailing white space around *pp-tokens* is discarded
  * *identifier* may be redefined only with macro with the same
    *[ identifier-list ]* and *[ pp-tokens ]*
* (*C99*) `#` `define` *identifier* `(` `...` `)` *[ pp-tokens ]* and `#`
  `define` *identifier* `(` *identifier-list* `,` `...` `)` *[ pp-tokens ]*
  * only these forms allow an occurrence of the identifier `__VA_ARGS__` in
    *pp-tokens*
* `#` `undef` *identifier*
  * causes the *identifier*'s definition to be forgotten
  * *identifier* may be unknown (not defined)
* a macro call is constituted of
  * the macro *identifier* (defined by the second form of `#define`)
  * optional white space
  * `(`
  * a sequence of tokens separated by commas
  * `)`
* the arguments of the call are comma separated token sequences
  * parenthesised or quoted commas do not separate arguments
* the macro expansion process consists of the following steps:
  1. arguments are collected; during collection, arguments are not expanded
  1. the number of arguments in the call must match the number of parameters in
     the definition
     * (*C99*) if the identifier list in the macro definition ends with an
       ellipsis, the number of arguments in the invocation shall be greater
       than the number of parameters in the macro definition (excluding the
       `...`); the trailing arguments (those covered with `...`), including
       their comma separators, are merged to form a single item: the variable
       arguments
  1. leading and trailing white space is removed from the arguments
  1. every unquoted occurrence of the parameter's identifier in the macro body
     is replaced by the token sequence of the corresponding argument
     * (*C99*) `__VA_ARGS__` is replaced with the variable arguments
     * unless the parameter's identifier is preceded by `#`, or preceded or
       followed by `##`, the argument tokens are examined for macro calls, and
       expanded as necessary, just before insertion
     * if the parameter's identifier is preceded by `#`, then
       1. the argument tokens are quoted (`"`)
       1. every occurrence of `"` and `\` that surrounds or is inside a string
          literal or character constant in the quoted argument tokens is
          escaped (`\`)
       1. both the `#` and the parameter's identifier are replaced with the
          quoted argument tokens
  1. every occurrence of `##` tokens are removed from the macro body together
     with surrounding white spaces so the adjacent tokens are concatenated
     producing a new token (this step is also performed when expanding a macro
     defined by the first form of `#define`)
     * if invalid tokens are produced, or if the result depends on the order of
       processing of the `##` operators, or `##` appears at the beginning or
       end of the macro body, the effect is undefined
  1. the macro body is repeatedly rescanned for more defined identifiers (this
     step is also performed when expanding a macro defined by the first form of
     `#define`)
     * in a given expansion, once replaced identifiers are not replaced again
       during rescanning
  1. the macro call is replaced with the final value of the expansion
* if the final value of a macro expansion begins with `#` it is not taken to be
  a preprocessing directive

## File Inclusion

* `#` `include` *\<filename\>*
  * replaces itself by the entire contents of the file *filename*
  * if the *filename* contains any of `"`, `'`, `\`, or `/*`, the effect is
    undefined
  * the file *filename* is searched for in a sequence of implementation
    dependent places
* `#` `include` *"filename"*
  * searches first in association with the original source file (a deliberately
    implementation dependent phrase); in a case of failure, it searches as if
    in the first form
  * if the *filename* contains any of `'`, `\`, or `/*`, the effect is
    undefined
    * `>` is permitted
* `#` `include` *pp-tokens*
  * the case where *pp-tokens* is neither *<...>* nor *"..."*
  * *pp-tokens* is expanded as a normal text, and the result of the expansion
    must be either in *<...>* or *"..."* form; then, it is treated as
    previously described
* `#include` files may be nested

## Conditional Compilation

* in `#if` and `#elif` directives
  1. *constant-expression* is first searched for `defined` *identifier* or
     `defined (` *identifier* `)` forms of expressions
     * they are replaced by `1L` if the *identifier* is defined or by `0L`
       otherwise
  1. then, the *constant-expression* is expanded
  1. then, all remaining identifiers are replaced by `0L`
  1. then, each integer constant is considered to be suffixed with `L`
  1. then, the final *constant-expression* must be integral with no `sizeof`,
     cast, or enumeration constant
  1. then, the final *constant-expression* is evaluated until a non-zero value
     is found
  1. lines following the directive with a zero value are discarded, except for
     checking the nesting of conditionals, up to the next `#elif`, `#else`, or
     `#endif` directive
  1. lines following the directive with a non-zero value are treated normally;
     after they are processed, succeeding `#elif` and `#else` directives
     together with their *[ group ]* are discarded except for checking the
     nesting of conditionals
  1. if all `#if` and `#elif` expressions are zero, and there is an `#else`,
     the lines following the `#else` are treated normally
* `#ifdef` *identifier* is equivalent to `# if defined` *identifier*
* `#ifndef` *identifier* is equivalent to `# if ! defined` *identifier*

## Line Control

* `#` `line` *constant* *"filename"* tells the compiler
  * that the line number of the next source line is the decimal integer
    *constant*
  * that the current input file is named *filename*
* `#` `line` *constant* tells the compiler
  * that the line number of the next source line is the decimal integer
    *constant*
  * that the current input file name stay as previously remembered
* `#` `line` *pp-tokens*
  1. macros in *pp-tokens* are expanded
  1. the `#line` is interpreted as described above

## Error Generation

* `#` `error` *[ pp-tokens ]*
  * writes a diagnostic message that includes *[ pp-tokens ]*

## Pragmas

* `#` `pragma` *[ pp-tokens ]*
  * performs an implementation dependent action
  * an unrecognized pragma is ignored
* pragmas introduced with *C99*
  * `#pragma STDC FP_CONTRACT ON|OFF|DEFAULT`
    * enables/disables *contracted* floating-point expressions
    * depending on the place of usage, its effect lasts until the next
      `FP_CONTRACT` appearance, the end of the compound statement, or the end
      of the translation unit
  * `#pragma STDC FENV_ACCESS ON|OFF|DEFAULT`
    * inform the implementation when a program might access the floating-point
      environment (allows the compiler to do certain optimizations)
    * the scoping rules are the same as for `FP_CONTRACT`
  * `#pragma STDC CX_LIMITED_RANGE ON|OFF|DEFAULT`
    * inform the implementation that the usual mathematical formulas for
      complex multiply, divide, and absolute value are acceptable
    * the scoping rules are the same as for `FP_CONTRACT`

### (C99) `_Pragma` Operator

* a unary operator of the form `_Pragma` `(` *string-literal* `)`
* it is processed as follows:
  1. *string-literal* is converted to *pp-tokens* in the following way:
     * the `L` prefix is deleted
     * leading and trailing double-quotes are deleted
     * `\"` and `\\` are replaced with `"` and `\`, respectively
     * the resulting sequence of characters is processed to produce *pp-tokens*
  1. *pp-tokens* are executed as if they were the *pp-tokens* in a pragma
     directive
  1. the original four tokens `_Pragma` `(` *string-literal* `)` are removed

## Null Directive

A preprocessor line containing only `#` has no effect.

## Predefined Macros

These macros, and the special identifier `defined`, cannot be undefined or
redefined:
* `__LINE__`
  * a decimal constant containing the current source line number
* `__FILE__`
  * a string literal containing the name of the file being compiled
* `__DATE__`
  * a string literal containing the date of compilation (in the `"Mmm dd yyyy"`
    form)
* `__TIME__`
  * a string literal containing the time of compilation (in the `"hh:mm:ss"`
    form)
* `__STDC__`
  * the constant 1
  * defined to be 1 only in standard conforming implementations
* (*C99*) `__STDC_VERSION__`
  * the version of the standard
  * has the form `yyyymmL`, e.g. `199901L`
* (*C99*) `__STDC_IEC_559__`
  * indicates conformance to the IEC 60559 floating-point arithmetic (set to 1
    in the positive case)
* (*C99*) `__STDC_ISO_10646__`
  * the version of the ISO/IEC 10646 standard; indicates that values of type
    `wchar_t` are the coded representations of the characters defined by
    ISO/IEC 10646
  * has the form `yyyymmL`, e.g. `199712L`
