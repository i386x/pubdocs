# TeX

TeX is a tool that converts plain text files into high-quality typography
documents. A plain text file on its input consists of characters to be typeset
and control sequences that drive the way how these characters are typeset. On
its output TeX produces DVI, PostScript, or PDF document containing the result.

TeX consists of four parts (processors):
1. **Input Processor** converts lines from input files to lines that have
   unified form across all operating systems.
1. **Token Processor** converts lines from input processor to a sequence of
   tokens. Token is either a pair containing a value and a category or a
   control sequence.
1. **Expand Processor** expands all expandable tokens until there is nothing to
   expand.
1. **Main Processor** read expanded tokens from the expand processor and
   interprets them as commands. The typesetting process and output assembling
   is happening here.

Processors are interconnected and they are working *on demand*.

## Input Processor

Input processor works as follows:
1. reads line from the input as defined by operating system
1. converts characters from the line to ASCII
1. removes end-line character, which may vary depending on operating system,
   from the line end
1. strips all spaces (ASCII 32) from the line end
1. appends to the line end the character defined in `\endlinechar` register

IniTeX sets `\endlinechar` to ASCII 13.

In opposite direction (when using `\write` or `\message` to send data to
terminal or log file):
1. ASCII characters are converted back to operating system character set
   * if an ASCII character is equal to the value stored in `\newlinechar` it is
     replaced by end-line character native for the host environment

### Input Processor's Registers and Primitives

* `\endlinechar` holds the character to be appended to the end of line
* `\escapechar` holds the value that represents escape character, used whenever
  a control sequence is converted to string
* `\newlinechar` holds the character that triggers the line break

## Token Processor

*Token* is either a pair containing value and category or a control sequence.
*Token category* is a number from the range 0 to 15, including 15. Token
categories are summarized in the following table:

| Category | Meaning | Defaults |
| -------- | ------- | -------- |
| 0 | escape | `\` |
| 1 | begin group | `{` (plain) |
| 2 | end group | `}` (plain) |
| 3 | math shift | `$` (plain) |
| 4 | tab align | `&` (plain) |
| 5 | end of line | ASCII 13 |
| 6 | macro parameter | `#` (plain) |
| 7 | superscript | `^` (plain) |
| 8 | subscript | `_` (plain) |
| 9 | ignored | ASCII 0 (plain) |
| 10 | space | ASCII 32, ASCII 9 (plain) |
| 11 | letter | `A` to `Z`, `a` to `z` |
| 12 | other | remaining characters |
| 13 | active | `~`, ASCII 12 (plain) |
| 14 | comment | `%` |
| 15 | invalid | ASCII 127 |

To change a character category, use the `\catcode` command.

### Scanning for Tokens

Lines prepared by the input processor are converted to sequences of tokens by
the token processor using the finite state machine (FSM) having three states:
* the new-line state *N* (the FSM enters this state whenever it receives a line
  from the input processor)
* the inside-the-line state *M*
* the skipping-spaces state *S*

When the FSM is in the state *N*:
1. skip all characters with 9, 10, and 15 category
1. if the current input character has a category 5
   * emit the control sequence `par`
   * skip the rest of the line
   * move to the next line
1. otherwise move to the state *M*

When the FSM is in the state *M*:
1. if the current input character has a category 7 and the following input
   character has the same ASCII value as the current input character
   * let refer these characters as `^^`
   * if the next at most two characters matches the regex `[0-9a-f]{1,2}`
     * let refer these characters as `<HEXVAL>`
     * then replace `^^<HEXVAL>` with the character which ASCII value expressed
       as a hexadecimal number is `<HEXVAL>`
   * otherwise if the next character has ASCII value lesser than 128 and is
     different from `0` to `9` and `a` to `f`
     * let refer this character as `<C>`
     * then replace `^^<C>` with the character which ASCII value *V* is equal
       either to the ASCII value of `<C>` minus 64 or to the ASCII value of
       `<C>` plus 64 so the ASCII value of *V* stays within the range 0 to 127,
       including 127
1. if the current input character has a category 0
   * if the next input character has category 11
     * read this and all the following consecutive characters with the category
       11 until the character with the different category or the end of line is
       met
     * let these read characters with the category 11 form the identifier *ID*
     * emit the control sequence *ID*
     * move to the state *S*
   * otherwise
     * read the next input character
     * let this read character is *C*
     * emit the control sequence *C*
     * if *C* has an ASCII value 32
       * move to the state *S*
     * otherwise move to the state *M*
1. if the current input character has a one of 1, 2, 3, 4, 6, 7, 8, 11, 12, or
   13 category
   * let this character be *C* and its category *T*
   * emit the token (*C*, *T*)
1. if the current input character has a category 9 or 15
   * ignore this character
1. if the current input character has a category 14
   * ignore the rest of the line
   * move to the next line
1. if the current input character has a category 5
   * emit the token (ASCII 32, 10)
   * skip the rest of the line
   * move to the next line
1. if the current input character has a category 10
   * emit the token (ASCII 32, 10)
   * move to the state *S*

When the FSM is in the state *S*:
1. skip all characters with 9, 10, and 15 category
1. if the current input character has a category 5
   * skip the rest of the line
   * move to the next line
1. otherwise move to the state *M*

When the FSM reaches the real end of line (not the character with the 5
category):
1. move to the next line
1. if the last character on the line has category 0
   * emit the empty control sequence

### Token Processor's Registers and Primitives

* `\catcode` sets/gets the category code of the given character

## Expand Processor

Expand processor is invoked every time whenever the main processor needs a
token in its fully-expanded form. Expand processor processes its input in the
following way:
1. request a token from the token processor's list of tokens
   * if the list is empty the token processor is invoked to handle the request
2. is it mark as non-expandable?
   * go to step 5
3. is it a macro?
   * replace the macro and additional tokens requested as the macro's
     parameters with the macro's body where all occurrences of macro parameter
     placeholders were replaced by the corresponding tokens (for the detailed
     description see [How Macros are Expanded](#how-macros-are-expanded))
   * the new sequence of tokens appears at the beginning of the list of tokens
   * go to step 1
4. is it a primitive or its alias that can be processed by the expand
   processor?
   * process it, request additional tokens if needed
   * remove the processed tokens from the list of tokens
   * add the result of processing to the beginning of the list of tokens
   * go to step 1
5. otherwise the token is non-expandable
   * remove the token from the list of tokens
   * send it to the main processor

### How Macros are Defined

Whenever TeX sees the following sequence of tokens
```
<prefix> <def-command> <cs-or-active> <parameters-specification> <balanced-text>
```
where
* `<prefix>` can be empty or one of `\global`, `\long`, and `\outer`
* `<def-command>` is one of `\def`, `\gdef`, `\edef`, or `\xdef`
* `<cs-or-active>` is either a control sequence or a token with category 13
* `<parameters-specification>` is a specification of parameters of the macro
  (can be empty), and
* `<balanced-text>` is a balanced text
it assigns to `<cs-or-active>` the meaning of macro with parameters
`<parameters-specification>` and `<balanced-text>` as its body.

The `<parameters-specification>` undergoes the following grammar:
```
<parameters-specification> ::= <separator>? <parameter-1>? <#>?
<parameter-1> ::= <#> <1> <separator>? <parameter-2>?
<parameter-2> ::= <#> <2> <separator>? <parameter-3>?
<parameter-3> ::= <#> <3> <separator>? <parameter-4>?
<parameter-4> ::= <#> <4> <separator>? <parameter-5>?
<parameter-5> ::= <#> <5> <separator>? <parameter-6>?
<parameter-6> ::= <#> <6> <separator>? <parameter-7>?
<parameter-7> ::= <#> <7> <separator>? <parameter-8>?
<parameter-8> ::= <#> <8> <separator>? <parameter-9>?
<parameter-9> ::= <#> <9> <separator>?

<separator> ::= a sequence of tokens except tokens with category 1, 2, and 6
<#> ::= a token with the category 6
<1> ::= a token ("1", 12)
<2> ::= a token ("2", 12)
<3> ::= a token ("3", 12)
<4> ::= a token ("4", 12)
<5> ::= a token ("5", 12)
<6> ::= a token ("6", 12)
<7> ::= a token ("7", 12)
<8> ::= a token ("8", 12)
<9> ::= a token ("9", 12)
```
The sequence `<#>` `<n>`, where *n* ranges from 1 to 9, including 9, is said to
be the *nth parameter* of the macro. The `<#>` token at the end of the
parameter specification results to appending to the parameter specification the
token (`{`, 1) as a separator. This is the only way how to use (`{`, 1) as a
separator.

If the `<def-command>` is one of `\edef` or `\xdef` the `<balanced-text>`
undergoes the full expansion.

Before the (potentially expanded) `<balanced-text>` becomes a macro body, it is
checked if the sequence of consecutive `<#>` tokens inside the balanced text is
* either of an even length;
* or of an odd length and the next token that immediately follows is `<n>`,
  where *n* is less or equal to the number of parameters in the parameters
  specification.

### How Macros are Expanded

When the expand processor sees a control sequence or an active character both
of which have the meaning of macro, it performs the following steps:
1. remove the control sequence or active character from the token list
2. if the macro was defined with parameters, then for every part of macro
   parameters specification:
   1. if it is a separator, request tokens from the token processor, check if
      they are matching (both character and category) the tokens in the
      separator, and remove them from the token list
      * if the token (`{`, 1) has been matched, return it back to the token
        list (note that this token can be only matched as the last during the
        entire parameters-loading process)
   2. if it is a `<#>` `<n>` sequence, where *n* ranges from 1 to 9, including
      9, then
      * if there is no separator after the `<n>`, remove the all category 10
        tokens from the beginning of the token list and load to the *n*th
        parameter
        * either a balanced text if the current token in the token list has
          category 1
        * or the current token
        and remove these tokens from the token list
      * otherwise load to the *n*th parameter a sequence of tokens up to the
        separator keeping 1 and 2 category tokens balanced and remove the
        loaded tokens from the token list
        * if the loaded parameter is of the form *category 1 token*, *balanced
          text*, *category 2 token*, remove the outermost category 1 and 2
          tokens
      * in the both cases, the token loaded to the *n*th parameter must not be
        `\par` (this can be changed by using the `\long` prefix)
3. write the tokens from the macro's body one by one at the beginning of the
   token list in the order they are appearing in the macro body following these
   rules:
   1. if the current token is `<#>` immediately followed by `<n>`, where *n*
      ranges from 1 to 9, including 9, then replace `<#>` `<n>` with the *n*th
      parameter and send the result to be written to the token list
   2. if the current token is `<#>` immediately followed by `<#>`, then replace
      `<#>` `<#>` with `<#>` and send the result to be written to the token
      list
   3. otherwise, send the current token to be written to the token list

### Expand Processor's Registers and Primitives

* `\string` converts its parameter to its string representation composed from
  category 12 and 10 tokens (`\escapechar` drives printing the escape char in
  case of control sequences)

## Main Processor

### Main Processor's Registers and Primitives

#### Debugging

* `\showlists` shows lists with page material
* `\tracingcommands` enables tracing what primitives do
* `\tracingmacros` enables tracing macro expansions

#### Macros Definition

* `\def` defines a macro
* `\edef` defines a macro, expand tokens inside its body
* `\gdef` defines a macro globally
* `\let` gives a control sequence meaning of some other token
* `\long` means that macro parameter can contain `\par`
* `\outer` means that macro cannot be inside other macro
* `\xdef` defines a macro globally, expand tokens inside its body

#### Miscellaneous

* `\global` states that the following action (assignment, definition) will be
  done at global level
* `\uccode` holds translation mapping for `\uppercase`
* `\uppercase` translates characters according to `\uccode`, category remains
  unchanged

## Terminology and Syntax Rules

* *balanced text*
  * tokens between a token with category 1 and a token with category 2,
    excluding these tokens, where every token with category 1 must have a
    matching token with category 2

## References

* Petr Olšák: *TeXbook naruby* (TeXbook inside out), 2nd edition, 2001.
