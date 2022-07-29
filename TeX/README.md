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

### How Macros Are Defined

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

### How Macros Are Expanded

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

### How Conditionals Are Processed

In the following text, under the `\if...`, `\or`, `\else`, and `\fi` are also
considered control sequences having the same meaning.

When the expand processor sees `\if...`, it performs the following steps,
simultaneously keeping the track of processed `\if...`s, `\or`s, `\else`s, and
`\fi`s:
1. remove `\if...` from the token list and request as many tokens as needed to
   evaluate the condition
   * unless `\ifx`, the condition is evaluated after the tokens are fully
     expanded
1. if the condition went true
   * remove the condition tokens from the token list
1. if the condition went false
   * remove the condition tokens from the token list
   * remove the tokens from the token list until the corresponding `\else` or
     `\fi`, included
1. if the `\if...` was `\ifcase` and the condition went a number *n*
   * remove the condition tokens from the token list
   * if *n* is non-zero, remove the tokens from the token list until the
     nearest matching `\or`, `\else`, or `\fi`, included

When the expand processor is going to expand `\or`:
1. check whether the `\or` is not extra
1. remove the `\or` and the following tokens from the token list until the
   nearest matching `\fi`, included

When the expand processor is going to expand `\else`:
1. check whether the `\else` is not extra
1. remove the `\else` and the following tokens from the token list until the
   matching `\fi`, included

When the expand processor is going to expand `\fi`:
1. check whether the `\fi` is not extra
1. remove the `\fi` from the token list

### Summary of What Is Not Expanded

1. a control sequence is not expanded if it is requested as a parameter of a
   command
1. a control sequence is not expanded if it is a token at `\let`, `\futurelet`,
   and `\show` commands
1. the `\let` command does not expand *equals*
1. a control sequence which is a part of parameters definition at `\def`,
   `\edef`, `\gdef`, and `\xdef` is not expanded
1. a control sequence which is a part of a macro body at `\def` and `\gdef` is
   not expanded
1. a control sequence that is going to be a part of a token list assembled by
   `\read` or to be assigned to a token register is not expanded
1. a control sequence is not expanded during the first pass through a token
   sequence at `\uppercase`, `\lowercase`, and `\write`
1. a control sequence is not expanded when it is read as a part of align
   template during a processing of `\halign` or `\valign`
   * token immediately following `\span` is expanded
   * tokens that form a *glue* value to be assigned to `\tabskip` are also
     expanded
1. a single character control sequence which is a part of a *number* value
   (i.e. a token behind `` ` ``) is not expanded
1. when TeX scans for the presence of the second `$` after the first `$` to
   distinguish between `$` and `$$` it not expands the token in the place where
   the second `$` is expected
1. when TeX scans parameters to be passed to a macro, it does not expand
   possible control sequences
1. a control sequence marked with `\noexpand` is not expanded
1. when TeX skips tokens between `\if...` and `\fi`, it does not expand
   possible control sequences
1. the first two tokens after `\ifx` are not expanded
1. the first token after `\string`, `\meaning`, `\noexpand`,
   `\afterassignment`, and `\aftergroup` is not expanded
1. during the first pass, `\expandafter` and `\futurelet` do not expand the
   first token
1. the tokens from the token register inserted to the token list by `\the` are
   not expanded

### Expand Processor's Registers and Primitives

* `\botmark` expands to the last `\mark` on the current page
* `\csname` `<text>` `\endcsname` fully expands `<text>` and then makes a
  control sequence from the result
* `\else` is a part of a conditional
* `\endcsname` marks the end of the control sequence produced by `\csname`
* `\endinput` tells the expand processor to stop reading tokens from the
  current file
* `\expandafter` expands the second token after `\expandafter`, then going back
  to the beginning of the token list
* `\fi` is a part of a conditional
* `\firstmark` expands to the first `\mark` on the current page
* `\fontname` expands to the font name (file name and *at clause*) of the given
  font
* `\if` compares ASCII values of two tokens (non-expandable control sequences
  has ASCII value 256)
* `\ifcase` chooses the *n*th case where *n* is the number from the condition
* `\ifcat` compares categories of two tokens
* `\ifdim` compares two dimensions
* `\ifeof` tests whether the file whose number is given in the condition has
  been entirely read
* `\iffalse` is always false
* `\ifhbox` tests whether the box whose number is given in the condition is a
  `\hbox`
* `\ifhmode` tests whether the main processor is in horizontal mode
* `\ifinner` tests whether the main processor is in inner (restricted) mode
* `\ifmmode` tests whether the main processor is in math mode
* `\ifnum` compares two numbers
* `\ifodd` tests whether the number given in the condition is odd
* `\iftrue` is always true
* `\ifvbox` tests whether the box whose number is given in the condition is a
  `\vbox`
* `\ifvmode` tests whether the main processor is in vertical mode
* `\ifvoid` tests whether the box whose number is given in the condition is
  void
* `\ifx` compares two macros for their definitions equality or two tokens for
  matching their ASCII values and categories
* `\input` tells to the expand processor to read tokens from the given file
* `\jobname` expand to the name of the main `*.tex` source file
* `\meaning` expands to the meaning of the next token
* `\noexpand` marks the following token as non-expandable
* `\or` is a part of a conditional
* `\romannumeral` followed by a number or a number register expands to the
  value of the number expressed in Roman digits
* `\string` converts its parameter to its string representation composed from
  category 12 and 10 tokens (`\escapechar` drives printing the escape char in
  case of control sequences)
* `\the` gets the value stored in the given register or font
  * if the register is a tokens register its content is not further expanded
  * otherwise the result of an expansion is a sequence of category 10 and 12
    tokens

## Main Processor

When TeX starts, the main processor is invoked, processing ongoing commands in
a simple evaluation loop:
1. request a fully-expanded token from the expand processor
1. interpret the token as a command
   1. if the command needs parameters, request other tokens
   1. do the command
1. go to step 1

### Main Processor's Modes

### Handling the Commands

A command to be processed by the main processor is either a token or a control
sequence. TeX distinguishes between find types of control sequences:
1. registers
2. font switches
3. main processor's primitives
4. expand processor's primitives
5. macros

Expand processor's primitives and macros are processed directly by expand
processor before they go to the main processor. Registers, font switches, and
main processor's primitives are processed directly by the main processor.

The way how a command is processed depends on the current mode of the main
processor. Here is the table with the summary:
| Command | Vertical mode | Horizontal mode | Math mode |
| ------- | ------------- | --------------- | --------- |
| letter | start a paragraph | typeset | typeset |
| other | start a paragraph | typeset | typeset |
| `\char` | start a paragraph | typeset | typeset |
| `\noboundary` | start a paragraph | suppress kerns and ligatures | idle |
| space | idle | insert a glue | idle |
| `\<space>` | start a paragraph | insert a glue | insert a glue |
| `\relax` | idle | idle | idle |
| `\ignorespaces` | skip blanks | skip blanks | skip blanks |
| `\end` | finish all activities | insert `\par` and try again | missing `$` |
| `\dump` | finish all activities | insert `\par` and try again | missing `$` |
| `\lastpenalty` | illegal | illegal | illegal |
| `\lastkern` | illegal | illegal | illegal |
| `\lastskip` | illegal | illegal | illegal |
| `\inputlineno` | illegal | illegal | illegal |
| `\badness` | illegal | illegal | illegal |
| `#` | illegal | illegal | illegal |
| `\leqno` | illegal | illegal | start equation numbering |
| `\eqno` | illegal | illegal | start equation numbering |
| `\raise` | illegal | raise a box | raise a box |
| `\lower` | illegal | lower a box | lower a box |
| `\moveleft` | move a box to the left | illegal | illegal |
| `\moveright` | move a box to the right | illegal | illegal |
| `\vadjust` | illegal | put vertical material under the current line | put vertical material under the current display math |
| `\/` | illegal | add italic correction | add a kern |
| `^` | missing `$` | missing `$` | typeset superscript |
| `_` | missing `$` | missing `$` | typeset subscript |
| `\mathchar` | missing `$` | missing `$` | typeset |
| `\mathord` | missing `$` | missing `$` | typeset |
| `\mathop` | missing `$` | missing `$` | typeset |
| `\mathbin` | missing `$` | missing `$` | typeset |
| `\mathrel` | missing `$` | missing `$` | typeset |
| `\mathopen` | missing `$` | missing `$` | typeset |
| `\mathclose` | missing `$` | missing `$` | typeset |
| `\mathpunct` | missing `$` | missing `$` | typeset |
| `\mathinner` | missing `$` | missing `$` | typeset |
| `\underline` | missing `$` | missing `$` | typeset |
| `\overline` | missing `$` | missing `$` | typeset |
| `\delimiter` | missing `$` | missing `$` | typeset |
| `\left` | missing `$` | missing `$` | typeset left delimiter |
| `\right` | missing `$` | missing `$` | typeset right delimiter |
| `\over` | missing `$` | missing `$` | typeset fraction |
| `\atop` | missing `$` | missing `$` | typeset fraction |
| `\abovewithdelims` | missing `$` | missing `$` | typeset fraction |
| `\overwithdelims` | missing `$` | missing `$` | typeset fraction |
| `\atopwithdelims` | missing `$` | missing `$` | typeset fraction |
| `\above` | missing `$` | missing `$` | typeset fraction |
| `\radical` | missing `$` | missing `$` | typeset |
| `\displaystyle` | missing `$` | missing `$` | set *display* style |
| `\textstyle` | missing `$` | missing `$` | set *text* style |
| `\scriptstyle` | missing `$` | missing `$` | set *script* style |
| `\scriptscriptstyle` | missing `$` | missing `$` | set *script script* style |
| `\mathchoice` | missing `$` | missing `$` | typeset |
| `\vcenter` | missing `$` | missing `$` | typeset |
| `\nonscript` | missing `$` | missing `$` | do not add glue to script styles |
| `\mkern` | missing `$` | missing `$` | add a kern |
| `\limits` | missing `$` | missing `$` | set *limits* flag |
| `\nolimits` | missing `$` | missing `$` | set *nolimitis* flag |
| `\displaylimits` | missing `$` | missing `$` | set *displaylimits* flag |
| `\mskip` | missing `$` | missing `$` | add a glue |
| `\mathaccent` | missing `$` | missing `$` | typeset |
| end of alignment template | finish alignment template | finish alignment template | missing `$` |
| `\par` | idle | finish a paragraph | missing `$` |
| `\vskip` | add a glue | insert `\par` and try again | missing `$` |
| `\vfil` | add a glue | insert `\par` and try again | missing `$` |
| `\vfill` | add a glue | insert `\par` and try again | missing `$` |
| `\vss` | add a glue | insert `\par` and try again | missing `$` |
| `\vfilneg` | add a glue | insert `\par` and try again | missing `$` |
| `\unvcopy` | unpack a vbox, keep the box register | insert `\par` and try again | missing `$` |
| `\unvbox` | unpack a vbox | insert `\par` and try again | missing `$` |
| `\valign` | start a paragraph | start align | missing `$` |
| `\hrule` | add a rule | insert `\par` and try again | missing `$` |
| `\vrule` | start a paragraph | add a rule | add a rule |
| `\hskip` | start a paragraph | add a glue | add a glue |
| `\hfil` | start a paragraph | add a glue | add a glue |
| `\hfill` | start a paragraph | add a glue | add a glue |
| `\hss` | start a paragraph | add a glue | add a glue |
| `\hfilneg` | start a paragraph | add a glue | add a glue |
| `\kern` | add a kern | add a kern | add a kern |
| `{` | start a group | start a group | start a group |
| `\begingroup` | start a group | start a group | start a group |
| `\endgroup` | start a group | start a group | start a group |
| `}` | end the group | end the group | end the group |
| `\leaders` | add a glue filled with box or rule | add a glue filled with box or rule | add a glue filled with box or rule |
| `\cleaders` | centered `\leaders` | centered `\leaders` | centered `\leaders` |
| `\xleaders` | `\cleaders` with the same spacing between boxes | `\cleaders` with the same spacing between boxes | `\cleaders` with the same spacing between boxes |
| `\shipout` | emit the box to DVI | emit the box to DVI | emit the box to DVI |
| `\box` | insert a box | insert a box | insert a box |
| `\copy` | insert a box without clearing the box register | insert a box without clearing the box register | insert a box without clearing the box register |
| `\lastbox` | pick the last box | pick the last box | pick the last box |
| `\vsplit` | split the `\vbox` | split the `\vbox` | split the `\vbox` |
| `\vtop` | `\vbox` with the reference point at the top | `\vbox` with the reference point at the top | `\vbox` with the reference point at the top |
| `\vbox` | make a vertical box | make a vertical box | make a vertical box |
| `\hbox` | make a horizontal box | make a horizontal box | make a horizontal box |
| `\noindent` | start a paragraph | idle | idle |
| `\indent` | start a paragraph | insert a box | insert a box |
| `$` | start a paragraph | start a math | end the math |
| `\unhcopy` | start a paragraph | unpack a hbox, keep the register | unpack a hbox, keep the register |
| `\unhbox` | start a paragraph | unpack a hbox | unpack a hbox |
| `\accent` | start a paragraph | make an accent | typeset |
| `\-` | start a paragraph | add a discretionary | add a discretionary |
| `\discretionary` | start a paragraph | add a discretionary | add a discretionary |
| `\halign` | start align | insert `\par` and try again | start align |
| `\insert` | make an insert | make an insert | make an insert |
| `\mark` | make a mark | make a mark | make a mark |
| `\penalty` | add a penalty | add a penalty | add a penalty |
| `\unskip` | remove the last glue | remove the last glue | remove the last glue |
| `\unkern` | remove the last kern | remove the last kern | remove the last kern |
| `\unpenalty` | remove the last penalty | remove the last penalty | remove the last penalty |
| `\cr` | align error | align error | align error |
| `\crcr` | align error | align error | align error |
| `\span` | align error | align error | align error |
| `&` | align error | align error | align error |
| `\noalign` | no align error | no align error | no align error |
| `\omit` | omit error | omit error | omit error |
| `\endcsname` | illegal | illegal | illegal |
| `\afterassignment` | insert a token after assignment | insert a token after assignment | insert a token after assignment |
| `\aftergroup` | insert a token after a group closes | insert a token after a group closes | insert a token after a group closes |
| `\closein` | close file for reading | close file for reading | close file for reading |
| `\openin` | open file for reading | open file for reading | open file for reading |
| `\message` | issue a message | issue a message | issue a message |
| `\errmessage` | issue an error message | issue an error message | issue an error message |
| `\lowercase` | convert to lower case | convert to lower case | convert to lower case |
| `\uppercase` | convert to upper case | convert to upper case | convert to upper case |
| `\showbox` | show the content of a box | show the content of a box | show the content of a box |
| `\showthe` | show the content of a register | show the content of a register | show the content of a register |
| `\showlists` | show the lists with a typeset material | show the lists with a typeset material | show the lists with a typeset material |
| `\show` | show the meaning of a token | show the meaning of a token | show the meaning of a token |
| `\openout` | open file for writing | open file for writing | open file for writing |
| `\write` | write to file | write to file | write to file |
| `\closeout` | close file for writing | close file for writing | close file for writing |
| `\special` | insert a *whatsit* node | insert a *whatsit* node | insert a *whatsit* node |
| `\immediate` | do I/O operations immediately | do I/O operations immediately | do I/O operations immediately |
| `\setlanguage` | insert a *set language* mark | insert a *set language* mark | insert a *set language* mark |
| `\long` | set *long* prefix | set *long* prefix | set *long* prefix |
| `\outer` | set *outer* prefix | set *outer* prefix | set *outer* prefix |
| `\global` | set *global* prefix | set *global* prefix | set *global* prefix |
| font switch | select a font | select a font | select a font |
| `\def` | define a macro | define a macro | define a macro |
| `\gdef` | define a macro | define a macro | define a macro |
| `\edef` | define a macro | define a macro | define a macro |
| `\xdef` | define a macro | define a macro | define a macro |
| `\futurelet` | define an alias to the 3rd token | define an alias to the 3rd token | define an alias to the 3rd token |
| `\let` | define an alias | define an alias | define an alias |
| `\chardef` | define an alias for `\charNN` | define an alias for `\charNN` | define an alias for `\charNN` |
| `\mathchardef` | define an alias for `\mathcharNN` | define an alias for `\mathcharNN` | define an alias for `\mathcharNN` |
| `\countdef` | define an alias for `\countNN` | define an alias for `\countNN` | define an alias for `\countNN` |
| `\dimendef` | define an alias for `\dimenNN` | define an alias for `\dimenNN` | define an alias for `\dimenNN` |
| `\skipdef` | define an alias for `\skipNN` | define an alias for `\skipNN` | define an alias for `\skipNN` |
| `\muskipdef` | define an alias for `\muskipNN` | define an alias for `\muskipNN` | define an alias for `\muskipNN` |
| `\toksdef` | define an alias for `\toksNN` | define an alias for `\toksNN` | define an alias for `\toksNN` |
| `\read` | define the file content as a macro | define the file content as a macro | define the file content as a macro |
| `\toks`(1) | assign to the `\toks` register | assign to the `\toks` register | assign to the `\toks` register |
| `\count`(2) | assign to the `\count` register | assign to the `\count` register | assign to the `\count` register |
| `\dimen`(3) | assign to the `\dimen` register | assign to the `\dimen` register | assign to the `\dimen` register |
| `\skip`(4) | assign to the `\skip` register | assign to the `\skip` register | assign to the `\skip` register |
| `\muskip`(5) | assign to the `\muskip` register | assign to the `\muskip` register | assign to the `\muskip` register |
| `\catcode` | set the category code | set the category code | set the category code |
| `\mathcode` | set the math code | set the math code | set the math code |
| `\lccode` | set the lower-case code | set the lower-case code | set the lower-case code |
| `\uccode` | set the upper-case code | set the upper-case code | set the upper-case code |
| `\sfcode` | set the space factor code | set the space factor code | set the space factor code |
| `\delcode` | set the delimiter code | set the delimiter code | set the delimiter code |
| `\textfont` | set the *text* font |  set the *text* font |  set the *text* font |
| `\scriptfont` | set the *script* font | set the *script* font | set the *script* font |
| `\scriptscriptfont` | set the *script script* font | set the *script script* font | set the *script script* font |
| `\advance` | do sum operation | do sum operation | do sum operation |
| `\multiply` | do multiplication operation | do multiplication operation | do multiplication operation |
| `\divide` | do division operation | do division operation | do division operation |
| `\setbox` | set a box register | set a box register | set a box register |
| `\prevdepth` | set the `\prevdepth` register | set the `\prevdepth` register | set the `\prevdepth` register |
| `\spacefactor` | set the `\spacefactor` register | set the `\spacefactor` register | set the `\spacefactor` register |
| `\prevgraf` | set the `\prevgraf` register | set the `\prevgraf` register | set the `\prevgraf` register |
| `\pagegoal` | set the `\pagegoal` register | set the `\pagegoal` register | set the `\pagegoal` register |
| `\pagetotal` | set the `\pagetotal` register | set the `\pagetotal` register | set the `\pagetotal` register |
| `\pagestretch` | set the `\pagestretch` register | set the `\pagestretch` register | set the `\pagestretch` register |
| `\pagefilstretch` | set the `\pagefilstretch` register | set the `\pagefilstretch` register | set the `\pagefilstretch` register |
| `\pagefillstretch` | set the `\pagefillstretch` register | set the `\pagefillstretch` register | set the `\pagefillstretch` register |
| `\pagefilllstretch` | set the `\pagefilllstretch` register | set the `\pagefilllstretch` register | set the `\pagefilllstretch` register |
| `\pageshrink` | set the `\pageshrink` register | set the `\pageshrink` register | set the `\pageshrink` register |
| `\deadcycles` | set the `\deadcycles` register | set the `\deadcycles` register | set the `\deadcycles` register |
| `\insertpenalties` | set the `\insertpenalties` register | set the `\insertpenalties` register | set the `\insertpenalties` register |
| `\wd` | set the box's width | set the box's width | set the box's width |
| `\ht` | set the box's height | set the box's height | set the box's height |
| `\dp` | set the box's depth | set the box's depth | set the box's depth |
| `\parshape` | set the paragraph shape | set the paragraph shape | set the paragraph shape |
| `\patterns` | set the hyphenation patterns | set the hyphenation patterns | set the hyphenation patterns |
| `\hyphenation` | set the hyphenation data | set the hyphenation data | set the hyphenation data |
| `\fontdimen` | set the font dimension | set the font dimension | set the font dimension |
| `\skewchar` | set the skew character | set the skew character | set the skew character |
| `\hyphenchar` | set the hyphenation character | set the hyphenation character | set the hyphenation character |
| `\font` | define new font | define new font | define new font |
| `\batchmode` | skip soft errors, no terminal output | skip soft errors, no terminal output | skip soft errors, no terminal output |
| `\nonstop` | skip soft errors | skip soft errors | skip soft errors |
| `\scrollmode` | errors are scrolled | errors are scrolled | errors are scrolled |
| `\errorstopmode` | stop on every error | stop on every error | stop on every error |

* (1) besides `\toks0` to `\toks255`, there are also primitive tokens registers:
  ```tex
  \output      \everypar    \everymath    \everydisplay    \everyhbox
  \everyvbox   \everyjob    \everycr     \errhelp
  ```
* (2) besides `\count0` to `\count255`, there are also primitive count
  (integer) registers:
  ```tex
  \pretolerance          \tolerance           \linepenalty             \hyphenpenalty
  \clubpenalty           \exhyphenpenalty     \widowpenalty            \displaywidowpenalty
  \brokenpenalty         \binoppenalty        \relpenalty              \predisplaypenalty
  \postdisplaypenalty    \interlinepenalty    \doublehyphendemerits    \finalhyphendemerits
  \adjdemerits           \mag                 \delimiterfactor         \looseness
  \time                  \day                 \month                   \year
  \showboxbreadth        \showboxdepth        \hbadness                \vbadness
  \pausing               \tracingonline       \tracingmacros           \tracingstats
  \tracingparagraphs     \tracingpages        \tracingoutput           \tracinglostchars
  \tracingcommands       \tracingrestores     \uchyph                  \outputpenalty
  \maxdeadcycles         \hangafter           \floatingpenalty         \globaldefs
  \fam                   \escapechar          \defaulthyphenchar       \defaultskewchar
  \endlinechar           \newlinechar         \language                \lefthyphenmin
  \righthyphenmin        \holdinginserts      \errorcontextlines
  ```
* (3) besides `\dimen0` to `\dimen255`, there are also primitive dimension
  registers:
  ```tex
  \parindent             \mathsurround          \lineskiplimit    \hsize             \vsize
  \maxdepth              \splitmaxdepth         \boxmaxdepth      \hfuzz             \vfuzz
  \delimitershortfall    \nulldelimiterspace    \scriptspace      \predisplaysize    \displaywidth
  \displayindent         \overfullrule          \hangindent       \hoffset           \voffset
  \emergencystretch
  ```
* (4) besides `\skip0` to `\skip255`, there are also primitive glue registers:
  ```tex
  \lineskip            \baselineskip             \parskip                  \abovedisplayskip
  \belowdisplayskip    \abovedisplayshortskip    \belowdisplayshortskip    \leftskip
  \rightskip           \topskip                  \splittopskip             \tabskip
  \spaceskip           \xspaceskip               \parfillskip
  ```
* (5) besides `\muskip0` to `\muskip255`, there are also primitive math glue
  registers:
  ```tex
  \thinmuskip    \medmuskip    \thickmuskip
  ```

Commands different from those listed in the table above are treated as errors.

### Registers and Data Types

Register is a place in TeX's memory where information is stored. Register can
serve either as an auxiliary storage or a parameter for tuning TeX's
algorithms. Registers can be accessed by:
1. primitive, e.g. `\baselineskip`
1. primitive and number, e.g. `\catcode64`
1. primitive and a more complex specifier, e.g. `\fontdimen3\tenrm`
1. a control sequence defined by commands like `\countdef`, e.g. `\pageno`

A value can be assigned to or retrieved from a register. Before the assignment
the value must match the type of a register or be convertible to this type.

There are six types of registers:
1. number registers, e.g. `\count0`
1. dimension registers, e.g. `\dimen0`
1. glue registers, e.g. `\skip0`
1. math glue registers, e.g. `\muskip0`
1. token registers, e.g. `\toks0`
1. box registers, e.g. `\box0`

Registers that serve as an auxiliary storage are:
* `\count10` to `\count255`
* `\dimen0` to `\dimen255`
* `\skip0` to `\skip255`
* `\muskip0` to `\muskip255`
* `\toks0` to `\toks255`
* `\box0` to `\box254`

`\count0` to `\count9` keeps a page number inserted to DVI (up to 10 page
numbers per page). The rightmost zeros are stripped, e.g. if `\count0` is 1 and
`\count2` is 3, the page in DVI will be numbered as `1.0.3`.

`\box255` holds a page ready to be shipped out (by `\shipout`).

`\countdef`, `\dimendef`, `\skipdef`, `\muskipdef`, and `\toksdef` can define a
control sequence to be a register equivalent, e.g. `\countdef\counta=0` makes
`\counta` to be an equivalent for `\count0`.

#### Numeric Constants

#### Data Types

#### Conversions

### Groups

If the main processor sees a category 1 token or the `\begingroup` primitive it
opens a group. Inside a group, all assignments are local unless this behavior
is changed with the `\globaldefs` register or the `\global` prefix.

If the main processor sees a category 2 token or the `\endgroup` primitive it
closes the group and all the local assignments are reverted back to their state
before the group opening.

The groups can be also nested. Some TeX primitive commands, like box primitives
are also opening/closing a group while being processed.

### Fonts

New fonts are loaded using the `\font` command, which also defines a font
switch control sequence which is used to change the current used font. The
syntax of `\font` is:
* `\font` *\<control sequence\> \<equals\> \<file name\> \<at clause\>*

### Main Processor's Registers and Primitives

#### Alignment

* `\cleaders` works like *centered* `\leaders`
* `\cr` ends row or column
* `\crcr` works like `\cr`, but if the last command was `\cr`, `\crcr`, or
  `\noalign` then this command has no effect
* `\halign` makes a table
* `\leaders` makes a space repeatedly filled with a rule or a box
* `\noalign` specifies what to insert between columns or rows

#### Arithmetic

* `\advance` performs `x += y`
* `\divide` performs `x /= y`
* `\multiply` performs `x *= y`

#### Boxes

* `\box` inserts the content of the box to the contributions list and clears
  the box register
* `\copy` works like `\box` but without clearing the register
* `\lastbox` removes the last box (does not work if the box is in the page
  list)

#### Contributions

* `\kern` makes a solid space
* `\lastkern` gets the last `\kern`
* `\lastpenalty` gets the last `\penalty`
* `\lastskip` gets the last `\skip`

#### Debugging

* `\message` writes an expanded text to the log file and terminal window
* `\show` shows the meaning of the following token
* `\showlists` shows lists with page material
* `\tracingcommands` enables tracing what primitives do
* `\tracingmacros` enables tracing macro expansions

#### Definitions

* `\chardef` gives a control sequence a meaning of `\charXX`
* `\countdef` gives a control sequence a meaning of `\countXX`
* `\def` defines a macro
* `\dimendef` gives a control sequence a meaning of `\dimenXX`
* `\edef` defines a macro, expand tokens inside its body
* `\font` defines font switch or refers to the current font
* `\futurelet` gives a control sequence a meaning of the token behind the next
  token (these 2 tokens are not removed)
* `\gdef` defines a macro globally
* `\let` gives a control sequence a meaning of some other token
* `\long` means that macro parameter can contain `\par`
* `\mathchardef` gives a control sequence a meaning of `\mathcharXXXX`
* `\muskipdef` gives a control sequence a meaning of `\muskipXX`
* `\outer` means that macro cannot be inside other macro
* `\read` defines a macro with a content of the entire file as its body
* `\skipdef` gives a control sequence a meaning of `\skipXX`
* `\toksdef` gives a control sequence a meaning of `\muskipXX`
* `\xdef` defines a macro globally, expand tokens inside its body

#### Error Management

* `\batchmode` tells TeX to skip all errors and to not display error messages
  on terminal
* `\errmessage` issues user-defined error
* `\errorstopmode` tells TeX to stop on error and ask a user for what to do
  next

#### File Operations

* `\closein` closes the input file
* `\closeout` closes the output file
* `\immediate` states that file operations are performed immediately
* `\write` writes a fully expanded text to a given file
  * unless `\immediate` prefix is used, the expansion is deferred until
    `\shipout`
  * if the file number is outside the range 0 to 15, including 15, the text is
    written to the log file
  * if the file number is greater than 15, the text is written also to terminal

#### Floats

* `\insert` makes a float

#### Grouping

* `\begingroup` opens a group
* `\endgroup` closes the group

#### Horizontal Contributions

* `\<space>` inserts space explicitly
* `\-` marks a place where a word can be hyphenated
* `\/` makes italic correction
* `\accent` makes an accent
* `\char` typesets a character
* `\discretionary` tells the TeX how a word should be hyphenated
* `\hbox` makes a horizontal box
* `\hfil` is a shortcut for `\hskip 0pt plus 1fil`
* `\hfill` is a shortcut for `\hskip 0pt plus 1fill`
* `\hfilneg` is a shortcut for `\hskip 0pt plus -1fil`
* `\hrule` makes a horizontal rule
* `\hskip` makes a horizontal space
* `\hss` is a shortcut for `\hskip 0pt plus 1fil minus 1fil`
* `\hyphenation` defines a user-defined hyphenation of words
* `\indent` starts the horizontal mode and inserts an empty box about
  `\parindent` width
* `\lower` lowers the box
* `\mark` puts a mark to the contributions list
* `\noboundary` suppresses implicit ligatures and `\kern`s
* `\noindent` starts the horizontal mode (paragraph) without indentation

#### Math Contributions

* `\delimiter` typesets a delimiter (parentheses)
* `\displaylimits` sets the *displaylimits* flag to Op atom
* `\eqno` allows to put a number on the right-hand side of an equation
* `\left` makes a left parentheses
* `\leqno` allows to put a number on the left-hand side of an equation
* `\limits` sets the *limits* flag to Op atom
* `\mathaccent` makes an accent in math list
* `\mathbin` makes a Bin atom
* `\mathchar` typesets a math character
* `\mathchoice` chooses what to typeset according to used math font style
* `\mathclose` makes a Close atom
* `\mathinner` makes an Inner atom
* `\mathop` makes an Op atom
* `\mathopen` makes an Open atom
* `\mathord` makes an Ord atom
* `\mathpunct` makes a Punct atom
* `\mathrel` makes a Rel atom
* `\mkern` makes a solid space in the math list
* `\mskip` makes a space in the math list

##### Fonts

* `\displaystyle` sets display (D) style

##### Fractions

* `\above` makes a fraction line, accepts thickness
* `\abovewithdelims` works like `\above` plus adds parentheses to sides

##### Matrices

* `\atop` puts a one object above the second
* `\atopwithdelims` works like `\atop` plus add parentheses to sides

#### Miscellaneous

* `\afterassignment` saves a token and puts it to the token list after
  assignment has been performed
* `\aftergroup` saves a token and puts it to the token list after group has
  been finished
* `\dump` works like `\end` but dumps TeX's memory to FMT file (works only with
  IniTeX)
* `\end` finishes all TeX activities and terminates it
  * in the horizontal mode inserts `\par` before itself and it is read again
  * in the vertical mode
    * if the current page and the contributions list are empty and
      `\deadcycles` is zero it terminates TeX
    * otherwise insert an empty box of `\hsize` width, `\vfill`, and
      `\penalty-2^30` to the vertical list, which invokes the page completion
      algorithm, and then `\end` is read again
* `\global` states that the following action (assignment, definition) will be
  done at the global level
* `\ignorespaces` tells to the main processor to ignore all spaces until
  non-space command occurs
* `\lowercase` translates characters according to `\lccode`, category remains
  unchanged
* `\uppercase` translates characters according to `\uccode`, category remains
  unchanged

#### Parameters and Registers

##### Alignment

* `\everycr` is a list of tokens inserted to the token list after `\cr` or
  `\crcr` that ends a line

##### Auxiliary Storage

* `\count` gives the access to a register for an integer storage
* `\dimen` gives the access to a register for a dimension storage
* `\muskip` gives the access to a register for a math space storage
* `\toks` gives the access to a register for a token sequence storage

##### Boxes

* `\badness` keeps a badness of the last completed box
* `\boxmaxdepth` keeps the maximal allowed depth of a box
* `\dp` sets/gets a box depth
* `\ht` sets/gets a box height

##### Codes

* `\catcode` sets/gets the category code of the given character
* `\delcode` tells to TeX how a character should be treated if it appears in
  math formula as a delimiter (parentheses)
* `\lccode` defines translation mapping for `\lowercase`
* `\mathcode` associates a math code with a character, i.e. it tells TeX how to
  translate `\char` to `\mathchar`
* `\uccode` defines translation mapping for `\uppercase`

##### Definitions

* `\globaldefs` states that all assignments are either global (if positive),
  local (if negative), or default (if zero &ndash; then the `\global` comes to
  action)

##### Error Management

* `\errhelp` holds tokens displayed when help is requested after `\errmessage`
* `\errorcontextlines` holds the number of extra context lines when error
  occurs

##### Floats

* `\floatingpenalty` is a penalty for insertions that are split
* `\holdinginserts` keeps inserts in the output box if the value is positive
* `\insertpenalties` is the sum of all penalties for split insertions on the
  page

##### Fonts

* `\defaulthyphenchar` is the default hyphenation character for all fonts
* `\defaultskewchar` is the default skew character for all fonts
* `\fontdimen` sets/gets a parameter to the font
* `\hyphenchar` sets/gets the hyphenation character of the font

##### Horizontal Contributions

* `\emergencystretch` is a stretch value added to a line to reduce badnesses
  on third (final) pass of line-breaking
* `\everyhbox` is a list of tokens inserted to the token list when a `\hbox`
  assembling process just started
* `\everypar` is a list of tokens inserted to the token list when entering the
  horizontal mode
* `\exhyphenpenalty` is a penalty for line break after explicit hyphen
* `\hangafter` is a number of lines in a paragraph affected by `\hangindent`
* `\hbadness` is a maximal horizontal badness
* `\hfuzz` is a maximum `\hbox` overrun
* `\hyphenpenalty` is a penalty for line break after discretionary hyphen
* `\language` chooses a hyphenation table
* `\lefthyphenmin` is a minimal number of characters in the left-hand side of
  the split word
* `\linepenalty` is a penalty added to badness of every line in a paragraph

##### Horizontal Spaces

* `\hangindent` is an indentation of lines given by `\hangafter`
* `\leftskip` is a space before each line of a paragraph

##### Input Processor

* `\endlinechar` holds the character to be appended to the end of line
* `\escapechar` holds the value that represents escape character, used whenever
  a control sequence is converted to string
  * applicable for `\errmessage`, `\message`, `\string`, and `\write`
  * if the escape character is outside of the range 0 to 255, including 255,
    nothing is printed before a control sequence identifier
* `\inputlineno` holds the number of the currently processed line from the
  currently processed file
* `\newlinechar` holds the character that is converted to the operating system
  defined line break

##### Math Contributions

* `\binoppenalty` is the amount of penalty inserted after every Bin atom
* `\delimiterfactor` is a ratio for variable delimiters multiplied by 1000
* `\everydisplay` is a list of tokens inserted to the token list after entering
  the display mode (`$$`)
* `\everymath` is a list of tokens inserted to the token list after entering
  the math mode (`$`)
* `\fam` sets the current family number

##### Math Spaces

* `\delimitershortfall` is a maximum space not covered by a delimiter
* `\displayindent` is an indentation for lines in math displays
* `\displaywidth` is a length of line in math displays
* `\mathsurround` is a size of a solid space around `$...$`
* `\medmuskip` is a size of the medium math space

##### Miscellaneous

* `\everyjob` is a list of tokens inserted to the token list when TeX starts

##### Page

* `\deadcycles` holds the number of output routines that ship no pages to DVI
* `\hoffset` is a horizontal offset in `\shipout`
* `\hsize` determines the width of the box with a page material
* `\mag` is a document magnification (1000 means 1)
* `\maxdeadcycles` is a maximal number of *dead cycles* (a call of the output
  routine where no page was shipped to DVI)
* `\maxdepth` is the maximal depth of the page box

##### Time

* `\day` keeps a day of a month
* `\month` keeps a month number

##### Vertical Contributions

* `\adjdemerits` is the amount of demerits of adjacent incompatible lines
* `\brokenpenalty` is the amount of penalty added to `\interlinepenalty`,
  `\widowpenalty` and `\clubpenalty` if the line contains a hyphenated word
* `\clubpenalty` is the amount of penalty added after the first line of a
  paragraph
* `\displaywidowpenalty` is the amount of penalty for creating a widow line
  before a display
* `\doublehyphendemerits` is the amount of demerits of consecutive broken lines
* `\everyvbox` is a list of tokens inserted to the token list when a `\vbox`
  assembling process just started
* `\finalhyphendemerits` is the amount of demerits of a penultimate broken line
* `\interlinepenalty` is the penalty added between two lines of a paragraph
* `\looseness` is a change to the number of lines in a paragraph

##### Vertical Spaces

* `\abovedisplayshortskip` is the space between a text and the top of the math
  display if the last line of text do not collide with the equation
* `\abovedisplayskip` is the space between a text and the top of the math
  display
* `\baselineskip` is the desired space between baselines
* `\belowdisplayshortskip` is the space between a text and the bottom of the
  math display if the last line of text do not collide with the equation
* `\belowdisplayskip` is the space between a text and the bottom of the math
  display
* `\lineskip` is the space between two lines if `\baselineskip` cannot be used
* `\lineskiplimit` is used to choose between `\lineskip` and `\baselineskip`

#### Vertical Contributions

* `\moveleft` moves a box to the left
* `\moveright` moves a box to the right

## Terminology and Syntax Rules

In the following, `at`, `scaled`, `bp`, `cc`, `cm`, `dd`, `in`, `mm`, `pc`,
`pt`, `sp`, `em`, `ex`, `by`, `depth`, `height`, `width`, `fil`, `fill`,
`filll`, `minus`, `plus`, `spread`, `to`, and `true` are considered *keywords*.
Keywords are case insensitive. In `fil`, `fill`, and `filll` are allowed spaces
between the `l`s.

* *at clause*
  * `<space>*`
  * `<space>* at<dimen>`
  * `<space>* scaled<number>`
* *balanced text*
  * tokens between a token with category 1 and a token with category 2,
    excluding these tokens, where every token with category 1 must have a
    matching token with category 2
* *control sequence*
  * a token with the 13 category is a control sequence
  * if the token processor sees a category 0 character followed by category 11
    characters, the these category 11 characters form a control sequence 
  * if the expand processor sees `\csname<something>\endcsname`,
    then `<something>` becomes a control sequence
* *dimen*
  * `<sign><float><unit>`
  * `<sign>?` `\dimen` register
  * `<sign>?` `\skip` register
* *equals*
  * `<space>*` `(=, 12)?`
* *file name*
  * a fully expanded sequence of tokens
  * initial spaces are skipped
  * non-expandable control sequence ends the file name
  * tokens of categories 1, 2, 3, 4, 6, 7, 8, 11, and 12 can be included in the
    file name
* *filler*
  * `( <space> | \relax )*`
* *font*
  * a control sequence with the meaning of a font switch
  * `\font`
  * `\textfont<number>`
  * `\scriptfont<number>`
  * `\scriptscriptfont<number>`
* *number*
  * `<sign> <digit>+ <space>?`
  * `<sign> (', 12) <odigit>+ <space>?`
  * `<sign> (", 12) <xdigit>+ <space>?`
  * ``<sign> (`, 12) <single-char-token> <space>?``
  * `<sign> \count register`
  * `<sign> \dimen register`
  * `<sign> \skip register`
  * `<sign> \chardef constant`
  * `<sign> \mathchardef constant`
* `<digit>` is defined as:
  ```
  <digit> ::= <odigit> | (8, 12) | (9, 12)
  ```
* `<float>` is defined as:
  ```
  <float> ::= <digit>+
  <float> ::= (', 12) <odigit>+
  <float> ::= (", 12) <xdigit>+
  <float> ::= (`, 12) <single-char-token>
  <float> ::= \chardef constant
  <float> ::= \mathchardef constant
  <float> ::= <digit>+ <float-point> <digit>*
  <float> ::= <float-point> <digit>+
  ```
* `<float-point>` is defined as:
  ```
  <float-point> ::= (., 12) | (,, 12)
  ```
* `<odigit>` is defined as:
  ```
  <odigit> ::= (0, 12) | (1, 12) | (2, 12) | (3, 12)
            |  (4, 12) | (5, 12) | (6, 12) | (7, 12)
  ```
* `<sign>` is defined as:
  ```
  <sign> ::= [ <space>, (+, 12), (-, 12) ]*
  ```
* `<single-char-token>` is defined as:
  ```
  <single-char-token> ::= single character control sequence
                       |  token (ASCII value, category)
  ```
* `<space>` is defined as:
  ```
  <space> ::= token of category 10 or its equivalent control sequence
  ```
* `<unit>` is defined as:
  ```
  <unit> ::= <space>* ( true <space>* )? pt <space>?
  <unit> ::= <space>* ( true <space>* )? pc <space>?
  <unit> ::= <space>* ( true <space>* )? bp <space>?
  <unit> ::= <space>* ( true <space>* )? dd <space>?
  <unit> ::= <space>* ( true <space>* )? cc <space>?
  <unit> ::= <space>* ( true <space>* )? in <space>?
  <unit> ::= <space>* ( true <space>* )? cm <space>?
  <unit> ::= <space>* ( true <space>* )? mm <space>?
  <unit> ::= <space>* ( true <space>* )? sp <space>?
  <unit> ::= <space>* em <space>?
  <unit> ::= <space>* ex <space>?
  <unit> ::= <space>* \count register
  <unit> ::= <space>* \dimen register
  <unit> ::= <space>* \skip register
  <unit> ::= <space>* \chardef constant
  <unit> ::= <space>* \mathchardef constant
  ```
* `<xdigit>` is defined as:
  ```
  <xdigit> ::= <digit> | (A, 11) | (B, 11) | (C, 11) | (D, 11) | (E, 11) | (F, 11)
                       | (A, 12) | (B, 12) | (C, 12) | (D, 12) | (E, 12) | (F, 12)
  ```

## References

* Donald Ervin Knuth: *The TeXbook*, 1986.
* Donald Ervin Knuth: *TeX: The Program*, 1986.
* Petr Olšák: *TeXbook naruby* (TeXbook inside out), 2nd edition, 2001.
