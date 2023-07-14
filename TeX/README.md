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
<prefix> <def-command> <cs-or-active> <parameters-specification> ({, 1)
    <balanced-text>
(}, 2)
```
where
* `<prefix>` can be empty or one of `\global`, `\long`, and `\outer`
* `<def-command>` is one of `\def`, `\gdef`, `\edef`, or `\xdef`
* `<cs-or-active>` is either a control sequence or a token with category 13
* `<parameters-specification>` is a specification of parameters of the macro
  (can be empty), and
* `({, 1)` is any token of the category 1
* `<balanced-text>` is a *balanced text*
* `(}, 2)` is any token of the category 2

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

If the `<def-command>` is one of `\edef` or `\xdef` the
`<balanced-text> (}, 2)` undergoes the full expansion.

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
          category 1 &ndash; surrounding category 1 and 2 tokens will not be
          loaded into the parameter
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
* `\number` expands to a sequence of digits representing the decimal value of
  its parameter
* `\or` is a part of a conditional
* `\romannumeral` expands to a sequence of Roman digits representing the value
  of its parameter
* `\splitbotmark` expands the last `\mark` in the `\vsplit`ted box
* `\splitfirstmark` expands the first `\mark` in the `\vsplit`ted box
* `\string` converts its parameter to its string representation composed from
  category 12 and 10 tokens (`\escapechar` drives printing the escape char in
  case of control sequences)
* `\the` gets the value stored in the given register or font
  * if the register is a tokens register its content is not further expanded
  * otherwise the result of an expansion is a sequence of category 10 and 12
    tokens
* `\topmark` expands to the last `\mark` from the previous page

## Main Processor

When TeX starts, the main processor is invoked, processing ongoing commands in
a simple evaluation loop:
1. request a fully-expanded token from the expand processor
1. interpret the token as a command
   1. if the command needs parameters, request other tokens
   1. do the command
1. go to step 1

### Main Processor's Modes

During the document assembling process, the main processor is switching between
six modes:
1. vertical mode
1. internal vertical mode
1. horizontal mode
1. restricted horizontal mode
1. math mode
1. display math mode

In vertical and internal vertical mode, TeX is building a *vertical list*. A
vertical list may content these elements, which are stored vertically in the
top-down manner:
* boxes (`\hbox`, `\vbox`, `\vtop`)
* rules (`\hrule`)
* kerns (`\kern`)
* glues (`\vskip`, `\leaders`, `\cleaders`, `\xleaders`, `\vfil`, `\vfill`,
  `\vfilneg`, `\vss`)
* penalties (`\penalty`)
* marks (`\mark`, `\write`)
* whatsits (`\special`)
* inserts (`\insert`)
The vertical list built in vertical mode is broken by TeX into particular
pages.

In horizontal and restricted horizontal mode, TeX is building a *horizontal
list*. A horizontal list may content these elements, which are stored
horizontally in the left-to-right manner:
* characters or ligatures (category 11 and 12 tokens, `\char`, `\chardef`
  defined constant)
* boxes (`\hbox`, `\vbox`, `\vtop`)
* rules (`\vrule`)
* kerns (`\kern`)
* glues (`\hskip`, `\leaders`, `\cleaders`, `\xleaders`, space token, `\hfil`,
  `\hfill`, `\hfilneg`, `\hss`)
* penalties (`\penalty`)
* discretionaries (`\discretionary`)
* marks (`\mark`, `\write`, `\vadjust`)
* whatsits (`\special`)
* inserts (`\insert`)

In math and display math mode, TeX is building a *math list*.

Switching between these modes are driven using the following rules:
1. When TeX starts, its in vertical mode.
1. Any mode plus `\vbox` or `\vtop`, math mode plus `\vcenter`:
   1. remember box specification
   1. enter a group
   1. enter the internal vertical mode
      * set `\looseness=0`
      * set `\parshape=0`, `\hangindent=0pt`, and `\hangafter=1`
   1. assemble a vertical list for this box
   1. leave the group
   1. complete and adjust the vertical list
   1. leave the internal vertical mode
   1. pass the box for further processing
1. Any mode plus `\hbox`:
   1. remember box specification
   1. enter a group
   1. enter the restricted horizontal mode
   1. assemble a horizontal list for this box
   1. leave the group
   1. complete and adjust the horizontal list
   1. leave the restricted horizontal mode
   1. pass the box for further processing
      * in any vertical mode, append the vertical material from the box right
        after the box just appended to the outer-level vertical list
1. Vertical or internal vertical mode plus category 11 token, category 12
   token, `\char`, `\chardef` defined constant, `\hskip`, `\hfil`, `\hfill`,
   `\hss`, `\hfilneg`, `\unhbox`, `\unhcopy`, `\vrule`, `\valign`, `\accent`,
   `\discretionary`, `\-`, `\<space>`, `\noboundary`, `$`, `$$`, `\indent`, or
   `\noindent`:
   1. enter the horizontal mode
   1. set `\prevgraf` to 0
   1. if the current token is distinct from `\indent` and `\noindent`, return
      it back to the token list
   1. initialize an empty horizontal list for incoming material
   1. if the current token was distinct from `\noindent`, insert to the
      horizontal list an empty `\hbox` of the width `\parindent`
   1. set *current_language* to 0
   1. insert the content of `\everypar` to the beginning of the token list
   1. contribute to the horizontal list with incoming material
      * note that `$$` in the vertical mode results in empty line (i.e. empty
        `\hbox` of the width `\parindent`) before equation
1. Horizontal mode plus `\par`, `\vskip`, `\vfil`, `\vfill`, `\vss`,
   `\vfilneg`, `\end`, `\unvbox`, `\unvcopy`, `\halign`, `\hrule`, or `\dump`:
   1. if the current token is not `\par`
      * insert the current token back to the token list
      * insert `\par` to the token list
      * get the next token from the token list, which is now `\par`
   1. process the current token, which is `\par`
      * if `\par` command has been invoked during the processing
        * invoke paragraph making algorithm, which also results in returning
          back to the vertical or internal vertical mode
1. Horizontal mode plus `}` or `\endgroup` that closes `\vbox`, `\vtop`, or
   `\vcenter`:
   * invoke `\par` command, which results in returning back to the vertical or
     internal vertical mode
1. Horizontal or restricted horizontal mode plus `$`:
   * enter the math mode (note that restricted horizontal mode plus `$$` means
     enter the math mode on the first `$` and then leave math mode on the
     second `$`)
1. Horizontal mode plus `$$`:
   * enter the display math mode
1. Math mode plus `$`:
   * leave the math mode
1. Display math mode plus `$$`:
   * leave the display math mode

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
| `\vadjust` | illegal | put vertical mode material under the current line | put vertical mode material under the current display math |
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

`\chardef\xyz=<number>` defines an 8-bit (0 to 255) numeric constant `\xyz`,
which serves as an equivalent for `\char<number>`.

`\mathchardef\uvw=<number>` defines a 15-bit (0 to 32767) numeric constant
`\uvw`, which serves as an equivalent for `\mathchar<number>`.

#### Data Types

A number register (`\count`) occupies 4 bytes. The range of stored number is
`-2^31 + 1` (-2147483647) to `2^31 - 1` (2147483647).

A dimension register (`\dimen`) occupies 4 bytes. Dimensions are stored
internally in `sp` units, the range of stored dimension is `(-2^30 + 1) sp`
(-1073741823 `sp`) to `(2^30 - 1) sp` (1073741823 `sp`). When assigning a
value to a dimension register, TeX supports various range of units:
| Unit | Meaning |
| ---- | ------- |
| `pt` | point |
| `pc` | pica (1 `pc` = 12 `pt`) |
| `in` | inch (1 `in` = 72.27 `pt`) |
| `bp` | big point, Postscript point (72 `bp` = 1 `in`) |
| `cm` | centimeter (2.54 `cm` = 1 `in`) |
| `mm` | millimeter (10 `mm` = 1 `cm`) |
| `dd` | didot point (1157 `dd` = 1238 `pt`) |
| `cc` | cicero (1 `cc` = 12 `dd`) |
| `sp` | scaled point, base TeX precision unit (65536 `sp` = 1 `pt`) |
| `em` | the width of a quad (the letter M) in the current font (`\fontdimen6\font`) |
| `ex` | the height of the letter x in the current font (`\fontdimen5\font`) |

All units except the `em` and `ex` can be prefixed with `true` keyword, which
cancels the effect of `\mag` (multiplies the dimension by `1000 / \mag`).

A glue register (`\skip`) has 3 dimension components: the base dimension, the
stretch dimension, and the shrink dimension. Besides the standard units, the
stretch and shrink dimensions can be also specified in dimensionless `fil`,
`fill`, or `filll` units. These units are internally stored as `pt`, that is
1.5 `fill` is internally stored as 1.5 `pt`, which is 98304 `sp`. The number of
`l`s in `fil`, `fill`, and `filll` specifies the order of stretching or
shrinking. For the standard units, the order is 0.

A math glue register (`\muskip`) has the same memory layout as a glue register.
The only allowed dimension unit is `mu`, internally stored as `pt`, and also
`fil`, `fill`, and `filll` for stretch and shrink dimensions.

A token register (`\toks`) refers to a list of tokens.

A box register (`\box`) refers to a data structure that holds a list of
horizontal (`h`) or vertical (`v`) mode material, called *box*. Supported
operations with a box register are:
* filling a register with a material (`\setbox`)
* using a register (`\box`, `\copy`)
* unpacking the material from a register (`\unhbox`, `\unvbox`, `\unhcopy`,
  `\unvcopy`)
* manipulating dimensions of a box held in a register (`\wd`, `\ht`, \dp`)
* testing a register (`\ifvoid`, `\ifhbox`, `\ifvbox`)
* inspecting the material in a register (`\showbox`)

`\box`, `\unhbox`, and `\unvbox` globally empties the register after using it.
On the other hand, changes made by `\setbox` are recorded at the current
nesting level of the nesting stack. This has following consequences:
```tex
\setbox0=...
{ \setbox0=... {\box0}  % Register \box0 is emptied
}  % Restoring the value of the \box0 register prior the \setbox0 operation
{ ... {\box0} ...}  % Register \box0 is emptied
```

#### Conversions

Allowed conversions between data types are:
* glue to dimension &ndash; stretch and shrink parts are omitted
* dimension to number &ndash; the stored value remains unchanged, i.e. to the
  number register is assigned the number of `sp` held in a dimension register
* any other conversions composed from the conversions above

#### Arithmetic Operations

`\advance X <optional by> Y`, where `X` is a number, dimension, glue, or math
glue register and `Y` is a number, dimension, glue, or math glue value,
performs `X += Y` as follows:
1. convert `Y` to the type of `X`
   * if it is not possible, the operation is canceled with an error
1. do `X += Y'`, where `Y'` denotes the converted `Y` from step 1
   * if `X` is glue or math glue, then `X += Y'` is computed as
     * `X.base += Y'.base`
     * `X.stretch += Y'.stretch` if stretch values are of the same order;
       otherwise, if `Y'` has higher order and `Y'.stretch` is nonzero, do
       `X.stretch = Y'.stretch`
     * `X.shrink += Y'.shrink` if shrink values are of the same order;
       otherwise, if `Y'` has higher order and `Y'.shrink` is nonzero, do
       `X.shrink = Y'.shrink`

`X Y`, where `X` is a floating-point number and `Y` is a dimension register
evaluates to `X * Y`.

`\multiply X <optional by> Y` and `\divide X <optional by> Y`, where `X` is a
number, dimension, glue, or math glue register and `Y` is a number value
performs `X *= Y` and `X /= Y` (integer division), respectively, as follows:
1. let `op` denotes one of `*` or `/`
1. if `X` is a number register
   * `X op= Y`
1. if `X` is a dimension register
   * `X.sp op= Y` (`.sp` means that the operation is performed on `sp` units)
1. if `X` is a glue or math glue register
   * `X.base.sp op= Y`
   * `X.stretch.sp op= Y`
   * `X.shrink.sp op= Y`

As `\divide` is working only with integers, it cannot be used to compute the
quotient of two floating-point numbers. However, there are several tricks how
to do that:
1. **Division by a floating point constant** *k*. Just multiply by 1/*k*,
   example:
   ```tex
   % Divide \dimen0 by 1.2:
   \dimen0 0.83334\dimen0
   ```
1. **Finding the quotient of two floating-point numbers**. Dimension and glue
   values are stored internally in `sp` units. The primitive `\the` followed by
   dimension or glue register expands to a fixed-point number expressing the
   value of the register in `pt` units. The relation between `pt` and `sp`
   implies that a value in `pt` is stored as a fixed-point number with a
   decimal point between the second and third byte. Therefore, to find a
   quotient between two numbers, it is natural to store these number in
   dimension registers as dimensions in `pt` units. The following algorithm in
   pseudo C++-like language finds a quotient of two dimensions, everything in
   `pt` units:
   ```C++
   void quotient(dimen & x, dimen y)
   {
     // We need the quotient to be in pt units, that is after the computation
     // x should hold (x/y)*65536 internally. As x and y are stored internally
     // as x*65536 and y*65536, respectively, we need to divide y by 65536.
     // However, this may cause a precision loss. Luckily, it is sufficient to
     // divide y only by a fraction of 65536, say 65536/K. To determine the K,
     // note that the largest value that can be held in dimension registers is
     // 16383.9999pt. That is, we can choose K such that x*K will be as close
     // to 16383.9999pt as possible. Since 65536 is a power of 2, we choose K
     // to be also a power of two, so we express K as 2^k. Now observe the
     // following equality:
     //
     //   (x/y)*65536 = [(x*65536)/(y*65536)]*65536
     //               = [(x*65536*K)/(y*65536*K)]*65536
     //               = [(x*65536*K)/(y*K)]
     //               = {(x*65536*K)/[y/(1/K)]}
     //               = {(x*65536*K)/[(y*65536)/(65536/K)]}
     //
     // We can compute (x*65536*K) and (65536/K) quantities simultaneously in a
     // loop:
     //
     //   count z = 65536;
     //   while (x < 8192 * pt)
     //     x *= 2, z /= 2;
     //
     // As y, stored as y*65536, and z are integers, the denominator y/z does
     // not overflow. Moreover, observe that z is always a power of 2 less or
     // equal to 65536, and thus y is always divisible by z. The only case when
     // TeX reports arithmetic overflow is when z reaches zero. This happens
     // when x < 8192sp = 0.125pt. Now that we have denominator, we can use it
     // to divide x, which has now internal quantity old_x*65536*K, by it to
     // get the final result.

     count z = 65536;

     if (y == 0)
       error("Division by zero!");

     if (x == 0)
       return;

     if (x < 0) {
       x = -x;
       z = -z;
     }

     while (x < 8192 * one_pt)
       x *= 2, z /= 2;
     y /= z;
     x /= y;
   }
   ```
1. **Computing the reciprocal value of a floating point number**. The previous
   algorithm can be modified to find a reciprocal value:
   ```C++
   void reciprocal(dimen & x)
   {
     dimen y = x;
     x = 8192 * one_pt;

     if (y == 0)
       error("Division by zero!");

     y /= 8;
     x /= y;
   }
   ```

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

Fonts have parameters accessed via `\fontdimen` *\<number\>*.

Text fonts must have these 7 parameters:
1. `\fontdimen1` &ndash; slant per pt
1. `\fontdimen2` &ndash; interword space
1. `\fontdimen3` &ndash; interword stretch
1. `\fontdimen4` &ndash; interword shrink
1. `\fontdimen5` &ndash; x-height
1. `\fontdimen6` &ndash; quad width
1. `\fontdimen7` &ndash; extra space

### Boxes

A box is a rectangular-shaped object with the *reference point*. From the
reference point are measured three box dimensions:
* a box width &ndash; a distance between reference point and the right edge of
  the box
* a box height &ndash; a distance between reference point and the top edge of
  the box
* a box depth &ndash; a distance between reference point and the bottom edge of
  the box

Some box dimensions may be negative, e.g. both `\hbox{kern-20pt}` and
`\hbox to-20pt{}` make a box of width -20pt. Such a box shifts the current
position 20pt to the left. Note that
* `\hbox` cannot have negative height and depth
* `\vbox` cannot have negative width
unless set by `\ht`, `\dp`, and `\wd`.

Some examples of boxes are:
* group of characters
* line in a paragraph
* table item
* table
* page

Horizontal boxes are positioned so their reference points lie on the
*baseline*.

Vertical boxes are positioned so their reference points lie on the same y-axis.

#### How a Horizontal Box Is Assembled

Lets describe a procedure of how the horizontal list is transformed to the
horizontal box (the reverse process is done by `\unhbox` or `\unhcopy`):
1. First, define procedure variables:
   * `context` &ndash; the context inside which the box is build
   * `horizontal_list` &ndash; the horizontal list
   * `new_hbox` &ndash; the horizontal box just made
1. Determine `new_hbox._expected_width`:
   ```python
   new_hbox._expected_width = None

   if new_hbox._spec["to"]:
       new_hbox._expected_width = new_hbox._spec["to"]
   elif context.width:
       new_hbox._expected_width = context.width
   ```
1. Move `horizontal_list` to `new_hbox.list`
1. Compute `new_hbox.height` and `new_hbox.depth`:
   ```python
   new_hbox.height = 0
   new_hbox.depth = 0

   for element in new_hbox.list:
       if element.type.super not in (BOX, RULE, CHAR):
           continue
       # `\raise x` sets `element.offset` to `x`
       # `\lower x` sets `element.offset` to `-x`
       h = element.height + element.offset
       d = element.depth - element.offset
       if h > new_hbox.height:
           new_hbox.height = h
       if d > new_hbox.depth:
           new_hbox.depth = d
   ```
1. Compute `\vrule` dimensions:
   ```python
   for vrule in new_hbox.list:
       if vrule.type is not VRULE:
           continue
       if vrule.height is None:
           vrule.height = new_hbox.height
       if vrule.depth is None:
           vrule.depth = new_hbox.depth
       if vrule.width is None:
           vrule.width = 0.4*pt
   ```
1. Compute natural width:
   ```python
   new_hbox._natural_width = 0

   for element in new_hbox.list:
       if element.type.super in (BOX, RULE, CHAR):
           new_hbox._natural_width += element.width
       elif element.type.super is SKIP:
           new_hbox._natural_width += element.base
       elif element.type is KERN:
           new_hbox._natural_width += element.size
   ```
1. Adjust expected width:
   ```python
   if new_hbox._spec["spread"]:
       new_hbox._expected_width = new_hbox._spec["spread"] + new_hbox._natural_width
   ```
1. If `new_hbox._expected_width` in not specified:
   * Set `new_hbox.width` to `new_hbox._natural_width`.
   * Set `new_hbox.badness` to 0.
   * Emit `new_hbox`.
1. If `new_hbox.list` is empty:
   * Set `new_hbox.width` to `new_hbox._expected_width`.
   * Set `new_hbox.badness` to 0.
   * Emit `new_hbox`.
1. Compute box spread:
   ```python
   if new_hbox._spec["spread"]:
       new_hbox._spread = new_hbox._spec["spread"]
   else:
       new_hbox._spread = new_hbox._expected_width - new_hbox._natural_width
   ```
1. If `new_hbox._spread` is 0:
   * Set `new_hbox.width` to `new_hbox._expected_width`.
   * Set `new_hbox.badness` to 0.
   * Emit `new_hbox`.
1. Sum up all stretch/shrink values:
   ```python
   new_hbox._total_sx = [0, 0, 0, 0]

   for skip in new_hbox.list:
       if skip.type.super is not SKIP:
           continue
       if new_hbox._spread > 0:
           new_hbox._total_sx[skip.stretch.order] += skip.stretch.value
       else:
           new_hbox._total_sx[skip.shrink.order] += skip.shrink.value
   ```
1. Select stretch/shrink order:
   ```python
   new_hbox._order = 3

   while new_hbox._order >= 0 and new_hbox._total_sx[new_hbox._order] == 0:
       new_hbox._order -= 1
   ```
1. Set `new_hbox.width` to `new_hbox._expected_width`.
1. Compute stretch/shrink sign and scale:
   ```python
   s = new_hbox._spread
   o = new_hbox._order

   if s < 0:
       if o < 0:
           new_hbox._sign = 0
       else:
           new_hbox._sign = -1
       if o < 0 or (o == 0 and -s > new_hbox._total_sx[o]):
           new_hbox._scale = 1.0
       else:
           new_hbox._scale = -s/new_hbox._total_sx[o]
   else:
       if o < 0:
           new_hbox._sign = 0
           new_hbox._scale = 0.0
       else:
           new_hbox._sign = 1
           new_hbox._scale = s/new_hbox._total_sx[o]
   ```
1. Compute badness:
   ```python
   s = new_hbox._spread
   o = new_hbox._order

   new_hbox.badness = 0
   if s < 0:
       if o < 0 or (o == 0 and -s > new_hbox._total_sx[o]):
           new_hbox.badness = INFINITY
           x = -s - new_hbox._total_sx[0]
           if x > context.hfuzz or context.hbadness < 100:
               warning(f"Overfull \\hbox ({x}pt too wide).")
       elif o == 0:
           new_hbox.badness = min(100*abs(s/new_hbox._total_sx[o])**3, 10000)
           if new_hbox.badness > context.hbadness:
               warning(f"Tight \\hbox (badness {new_hbox.badness}).")
   elif o <= 0:
       if new_hbox._total_sx[0] <= 0:
           new_hbox.badness = 10000
       else:
           new_hbox.badness = min(100*abs(s/new_hbox._total_sx[0])**3, 10000)
       if new_hbox.badness > context.hbadness:
           if new_hbox.badness > 100:
               warning(f"Underfull \\hbox (badness {new_hbox.badness}).")
           else:
               warning(f"Loose \\hbox (badness {new_hbox.badness}).")
   ```
1. Shrink or stretch the box:
   ```python
   s = new_hbox._spread
   o = new_hbox._order

   for skip in new_hbox.list:
       if skip.type.super is not SKIP:
           continue
       if s < 0 and skip.shrink.order == o:
           skip.base += new_hbox._sign*new_hbox._scale*skip.shrink.value
       if s > 0 and skip.stretch.order == o:
           skip.base += new_hbox._sign*new_hbox._scale*skip.stretch.value
   ```
1. Emit the box `new_hbox`.

The reference point of the `new_hbox` is the reference point of its first
element.

#### How a Vertical Box Is Assembled

Lets describe a procedure of how the vertical list is transformed to the
vertical box (the reverse process is done by `\unvbox` or `\unvcopy`):
1. First, define procedure variables:
   * `context` &ndash; the context inside which the box is build
   * `vertical_list` &ndash; the vertical list
   * `new_vbox` &ndash; the vertical box just made
1. Determine `new_vbox._expected_height`:
   ```python
   new_vbox._expected_height = None

   if new_vbox._spec["to"]:
       new_vbox._expected_height = new_vbox._spec["to"]
   elif context.height:
       new_vbox._expected_height = context.height
   ```
1. Move `vertical_list` to `new_vbox.list`.
1. Compute `new_vbox.width`:
   ```python
   new_vbox.width = 0

   for element in new_vbox.list:
       if element.type.super not in (BOX, RULE):
           continue
       # `\moveright x` sets `element.offset` to `x`
       # `\moveleft x` sets `element.offset` to `-x`
       w = element.width + element.offset
       if w > new_vbox.width:
           new_vbox.width = w
   ```
1. Compute `\hrule` dimensions:
   ```python
   for hrule in new_vbox.list:
       if hrule.type is not HRULE:
           continue
       if hrule.width is None:
           hrule.width = new_vbox.width
       if hrule.height is None:
           hrule.height = 0.4*pt
       if hrule.depth is None:
           hrule.depth = 0
   ```
1. Compute `new_vbox.depth`:
   ```python
   def get_last_element(list, types):
       seen = None

       for x in list:
           if x.type.super in types:
               seen = x
       return seen


   def get_next_element(list, element, types):
       next = None

       searching = True
       for x in list:
           if searching:
               if x is element:
                   searching = False
               continue
           if x.type.super in types:
               next = x
               break
       return next


   new_vbox.depth = 0

   last_box = get_last_element(new_vbox.list, (BOX, RULE))
   if last_box and not get_next_element(new_vbox.list, last_box, (SKIP, KERN)):
       new_vbox.depth = last_box.depth

   if new_vbox.depth > context.boxmaxdepth:
       new_vbox.depth = context.boxmaxdepth
   ```
1. Compute natural height:
   ```python
   new_vbox._natural_height = 0

   for element in new_vbox.list:
       if element.type.super in (BOX, RULE):
           new_vbox._natural_height += element.height + element.depth
       elif element.type.super is SKIP:
           new_vbox._natural_height += element.base
       elif element.type is KERN:
           new_vbox._natural_height += element.size
   new_vbox._natural_height -= new_vbox.depth
   ```
1. Adjust expected height:
   ```python
   if new_vbox._spec["spread"]:
       new_vbox._expected_height = new_vbox._spec["spread"] + new_vbox._natural_height
   ```
1. If `new_vbox._expected_height` is not specified:
   * Set `new_vbox.height` to `new_vbox._natural_height`.
   * Set `new_vbox.badness` to 0.
   * Emit `new_vbox`.
1. If `new_vbox.list` is empty:
   * Set `new_vbox.height` to `new_vbox._expected_height`
   * Set `new_vbox.badness` to 0.
   * Emit `new_vbox`.
1. Compute box spread:
   ```python
   if new_vbox._spec["spread"]:
       new_vbox._spread = new_vbox._spec["spread"]
   else:
       new_vbox._spread = new_vbox._expected_height - new_vbox._natural_height
   ```
1. If `new_vbox._spread` is 0:
   * Set `new_vbox.height` to `new_vbox._expected_height`.
   * Set `new_vbox.badness` to 0.
   * Emit `new_vbox`.
1. Sum up all stretch/shrink values:
   ```python
   new_vbox._total_sx = [0, 0, 0, 0]

   for skip in new_vbox.list:
       if skip.type.super is not SKIP:
           continue
       if new_vbox._spread > 0:
           new_vbox._total_sx[skip.stretch.order] += skip.stretch.value
       else:
           new_vbox._total_sx[skip.shrink.order] += skip.shrink.value
   ```
1. Select stretch/shrink order:
   ```python
   new_vbox._order = 3

   while new_vbox._order >= 0 and new_vbox._total_sx[new_vbox._order] == 0:
       new_vbox._order -= 1
   ```
1. Set `new_vbox.height` to `new_vbox._expected_height`.
1. Compute stretch/shrink sign and scale:
   ```python
   s = new_vbox._spread
   o = new_vbox._order

   if s < 0:
       if o < 0:
           new_vbox._sign = 0
       else:
           new_vbox._sign = -1
       if o < 0 or (o == 0 and -s > new_vbox._total_sx[o]):
           new_vbox._scale = 1.0
       else:
           new_vbox._scale = -s/new_vbox._total_sx[o]
   else:
       if o < 0:
           new_vbox._sign = 0
           new_vbox._scale = 0.0
       else:
           new_vbox._sign = 1
           new_vbox._scale = s/new_vbox._total_sx[o]
   ```
1. Compute badness:
   ```python
   s = new_vbox._spread
   o = new_vbox._order

   new_vbox.badness = 0
   if s < 0:
       if o < 0 or (o == 0 and -s > new_vbox._total_sx[o]):
           new_vbox.badness = INFINITY
           x = -s - new_vbox._total_sx[0]
           if x > context.vfuzz or context.vbadness < 100:
               warning(f"Overfull \\vbox ({x}pt too high).")
       elif o == 0:
           new_vbox.badness = min(100*abs(s/new_vbox._total_sx[o])**3, 10000)
           if new_vbox.badness > context.vbadness:
               warning(f"Tight \\vbox (badness {new_vbox.badness}).")
   elif o <= 0:
       if new_vbox._total_sx[0] <= 0:
           new_vbox.badness = 10000
       else:
           new_vbox.badness = min(100*abs(s/new_vbox._total_sx[0])**3, 10000)
       if new_vbox.badness > context.vbadness:
           if new_vbox.badness > 100:
               warning(f"Underfull \\vbox (badness {new_vbox.badness}).")
           else:
               warning(f"Loose \\vbox (badness {new_vbox.badness}).")
   ```
1. Shrink or stretch the box:
   ```python
   s = new_vbox._spread
   o = new_vbox._order

   for skip in new_vbox.list:
       if skip.type.super is not SKIP:
           continue
       if s < 0 and skip.shrink.order == o:
           skip.base += new_vbox._sign*new_vbox._scale*skip.shrink.value
       if s > 0 and skip.stretch.order == o:
           skip.base += new_vbox._sign*new_vbox._scale*skip.stretch.value
   ```
1. Emit the box `new_vbox`.

The top edge of the `new_vbox` is the top edge of its first element.

If `new_vbox` is `\vtop`:
```python
h = 0

for element in new_vbox.list:
    if element.type.super in (BOX, RULE, SKIP, KERN):
        break
else:
    element = None

if element and element.type.super in (BOX, RULE):
    h = element.height

d = new_vbox.height + new_vbox.depth - h
new_vbox.height = h
new_vbox.depth = d
```

If `new_vbox` is `\vcenter`:
```python
assert context.mode == MMODE

# Current family 2 font
font = context.font.family[2]
# \fontdimen22\font (math axis from baseline distance in upwards direction)
x = font.dimens[22]

new_vbox.height -= x
new_vbox.depth += x
```

### Spaces

Lets see how spaces are inserted to horizontal/vertical list.

#### How Spaces Are Inserted to the Horizontal List

When a space is inserted to the horizontal list the current position where the
next element will be inserted is shifted to the right about the space width.

*Kern* is a space with the fixed width. Kerns usually disallows line breaks.
Kern can be inserted to the horizontal list in these ways:
1. According to the table of kerning pairs of the current font. This can be
   described by the following algorithm:
   ```python
   # This check is done before the current command is executed
   if last_command.type == CHAR and current_command.type == CHAR:
       kern = context.font.kerning_table.get((last_command.code, current_command.code))
       if kern:
           emit(kern)
   ```
1. Using `\kern` or *italic correction* `\/` command. `\/` command adjusts the
   space between italic and normal character in the following way:
   1. It reads from the current font the amount of space associated with the
      last character.
   1. Then it inserts this space as a `\kern` to the current horizontal list.

*Glue* is a space that can shrunk or stretched. Glues usually allows line
breaks. Glue can be inserted to the horizontal list in these ways:
1. Using space token. This token is converted to `\hskip` according to the
   following algorithm:
   ```python
   # Get the current font:
   font = context.font

   # Gather the current font parameters:
   space = font.dimens[2]
   stretch = font.dimens[3]
   shrink = font.dimens[4]
   extra = font.dimens[7]

   # Get the `\spacefactor`:
   f = context.spacefactor

   # Prefer `\spaceskip` and `\xspaceskip` over `\fontdimen`:
   if f < 2000 and context.spaceskip != 0:
       space = context.spaceskip.base
       stretch = context.spaceskip.stretch
       shrink = context.spaceskip.shrink
   elif f >= 2000 and context.xspaceskip != 0:
       space = context.xspaceskip.base
       stretch = context.xspaceskip.stretch
       shrink = context.xspaceskip.shrink

   # Adjust space dimensions:
   stretch *= f/1000
   shrink *= 1000/f
   if f >= 2000 and context.xspaceskip == 0:
       space += extra

   # Insert `\hskip`:
   emit(hskip(space, stretch, shrink))
   ```
   The value of `\spacefactor` is changing while the horizontal list is
   building. The exact process is described by the following snippet from the
   horizontal list building procedure:
   ```python
   open_hlist(context)

   # At the beginning of the horizontal list, set `\spacefactor` to 1000:
   context.spacefactor = 1000

   while True:
       # Get the element from the token list:
       e = get_element(context)

       # If the element marks the end of the horizontal list, finish the
       # horizontal list:
       if e.type.match(END_HLIST):
           break
       # If the element is a character:
       elif e.type.match(CHAR):
           # - insert the character to the horizontal list:
           emit(e)
           # - read the `\sfcode` of the character:
           sf = context.sfcodes[e.code]
           # - adjust the value:
           if sf == 0:
               sf = context.spacefactor
           elif context.spacefactor < 1000 and 1000 < sf:
               sf = 1000
           # - assign the adjusted value to `\spacefactor`:
           context.spacefactor = sf
       # If the element is a box or rule:
       elif e.type.match(BOX, RULE):
           # - insert the element to the horizontal list:
           emit(e)
           # - set `\spacefactor` to 1000
           context.spacefactor = 1000
       # If the element is `\spacefactor`:
       elif e.type.match(COMMAND) and e.code == CMD_SPACEFACTOR:
           # - scan the new `\spacefactor` value specified as `= <number>`:
           sf = scan(context, NUMBER_ASSIGNMENT)
           # - `\spacefactor` must be positive:
           assert sf > 0
           # - update `\spacefactor`:
           context.spacefactor = sf
       # Other elements are processed without changing `\spacefactor`:
       else:
           process(e)
   close_hlist(context)
   ```
1. Using `\<space>` command. This is an equivalent to
   ```tex
   \hskip \fontdimen2\font plus \fontdimen3\font minus \fontdimen4\font
   ```
1. Using `\hskip`, `\hss`, `\hfil`, `\hfill`, or `\hfilneg` command.

#### How Spaces Are Inserted to the Vertical List

Vertical spaces are measured as a distance between the bottom of the one box
and the top of the second box.

*Automatically inserted interline space*
* inserted automatically between every two boxes in the vertical list
* not inserted if there is a rule between the boxes
* the whole process can be described by the following snippet of the vertical
  list building procedure:
  ```python
  open_vlist(context)

  # The `\prevdepth` register:
  context.prevdepth = -1000*pt

  while True:
      # Get the element to be inserted into the vertical list:
      e = get_element(context)

      # End of the vertical list -> finish the vertical list:
      if e.type.match(END_VLIST):
          break
      # Box arrives:
      elif e.type.match(BOX):
          # Box arrives also previously:
          if context.prevdepth > -1000*pt:
              d = context.prevdepth
              h = e.height
              # Space between boxes is `\baselineskip` minus height of the
              # current minus depth of the previous:
              s = context.baselineskip.base - h - d

              # `\lineskiplimit` is the minimal distance between boxes:
              if s >= context.lineskiplimit:
                  emit(vskip(s, context.baselineskip.stretch, context.baselineskip.shrink))
              else:
                  # If boxes are touching each other use `\lineskip`:
                  emit(vskip(context.lineskip))
          # We have the box -> record its depth to `\prevdepth`:
          context.prevdepth = e.depth
          emit(e)
      elif e.type.match(RULE):
          # Rule between boxes means no interline space is inserted:
          context.prevdepth = -1000*pt
          emit(e)
      else:
          # Other elements of the vertical list have no effect to interline
          # spaces except that `\prevdepth`, `\baselineskip`, `\lineskip`, and
          # `\lineskiplimit` can be changed there:
          process(e)
  close_vlist(context)
  ```

*`\topskip`-based space*
* determines the size of the initial space at the top of the page
* `\topskip` is the distance between the page's box top edge and the first
  baseline on the page
* the following snippet from page builder demonstrates how the initial space is
  inserted:
  ```python
  # If the first box or rule comes to the page:
  if current_page.is_empty() and e.type.match(BOX, RULE):
      h = e.height
      # Insert the initial space based on `\topskip`:
      if h < context.topskip.base:
          emit(vskip(context.topskip.base - h, context.topskip.stretch, context.topskip.shrink))
      else:
          emit(vskip(0, context.topskip.stretch, context.topskip.shrink))
      # And then insert the box or rule:
      emit(e)
  ```

Analogously, `\vsplit` uses `\splittopskip`.

*`\parskip`-based space*
* between two paragraphs is automatically inserted `\vskip\parskip`

### Specifying Fill Pattern with `\leaders`

`\leaders` behaves like `\hskip` or `\vskip` &ndash; it is a glue which fills
its space by the given pattern. `\leaders` accepts these parameters:
* `\leaders` *\<box or rule\>* *\<glue specification\>*

If *\<box or rule\>* is a rule and when the final size of glue is established:
1. Establish the missing rule dimensions as described in the
   horizontal/vertical list-to-box conversion.
1. In the horizontal mode:
   * Set the rule width to the finally established glue size.
1. In the vertical mode:
   * Set the rule height to the finally established glue size.
   * Set the rule depth to 0pt.
1. If the finally established glue size is non-negative:
   * Draw the rule in the space occupied by the glue with respect to
     baseline/y-axis.

If *\<box or rule\>* is a box and when the final size of glue is established:
1. In case of `\leaders` in the horizontal mode:
   ```python
   # Snippet of Python-like pseudo-code that ships `\leaders` element `e` to
   # the DVI.

   # Get the current position:
   cx = get_current_position_x()
   # Get the final glue width:
   tw = e.skip.base
   # Get the box width:
   w = e.box.width
   # Get the x-coord of the left edge of the outer box (box position is its
   # reference point position):
   x = get_outer_box().position.x

   if tw <= 0 or w <= 0:
       set_current_position_x(cx + tw)
       return

   while x < cx:
       x += w
   while x + w < cx + tw:
       draw(e.box)
       x += w
   # Fix the current position:
   set_current_position_x(cx + tw)
   ```
1. In case of `\leaders` in the vertical mode:
   ```python
   # Snippet of Python-like pseudo-code that ships `\leaders` element `e` to
   # the DVI.

   # Get the current position:
   cy = get_current_position_y()
   # Get the final glue height:
   th = e.skip.base
   # Get the total box height:
   h = e.box.height + e.box.depth
   # Get the y-coord of the top edge of the outer box (box position is its
   # reference point position):
   y = get_outer_box().position.y - get_outer_box().height

   if th <= 0 or h <= 0:
       set_current_position_y(cy + th)
       return

   while y < cy:
       y += h
   while y + h < cy + th:
       draw(e.box)
       y += h
   # Fix the current position:
   set_current_position_y(cy + th)
   ```
1. In case of `\cleaders` in the horizontal mode:
   ```python
   # Snippet of Python-like pseudo-code that ships `\leaders` element `e` to
   # the DVI.

   # Get the final glue width:
   tw = e.skip.base
   # Get the box width:
   w = e.box.width

   if tw <= 0 or w <= 0:
       set_current_position_x(get_current_position_x() + tw)
       return

   n = int(tw/w)
   s = (tw - n*w)/2

   set_current_position_x(get_current_position_x() + s)
   while n > 0:
       draw(e.box)
       n -= 1
   set_current_position_x(get_current_position_x() + s)
   ```
1. In case of `\cleaders` in the vertical mode:
   ```python
   # Snippet of Python-like pseudo-code that ships `\leaders` element `e` to
   # the DVI.

   # Get the final glue height:
   th = e.skip.base
   # Get the total box height:
   h = e.box.height + e.box.depth

   if th <= 0 or h <= 0:
       set_current_position_y(get_current_position_y() + th)
       return

   n = int(th/h)
   s = (th - n*h)/2

   set_current_position_y(get_current_position_y() + s)
   while n > 0:
       draw(e.box)
       n -= 1
   set_current_position_y(get_current_position_y() + s)
   ```
1. In case of `\xleaders` in the horizontal mode:
   ```python
   # Snippet of Python-like pseudo-code that ships `\leaders` element `e` to
   # the DVI.

   # Get the final glue width:
   tw = e.skip.base
   # Get the box width:
   w = e.box.width

   if tw <= 0 or w <= 0:
       set_current_position_x(get_current_position_x() + tw)
       return

   n = int(tw/w)
   s = (tw - n*w)/(n + 1)

   set_current_position_x(get_current_position_x() + s)
   while n > 0:
       draw(e.box)
       set_current_position_x(get_current_position_x() + s)
       n -= 1
   ```
1. In case of `\xleaders` in the vertical mode:
   ```python
   # Snippet of Python-like pseudo-code that ships `\leaders` element `e` to
   # the DVI.

   # Get the final glue height:
   th = e.skip.base
   # Get the total box height:
   h = e.box.height + e.box.depth

   if th <= 0 or h <= 0:
       set_current_position_y(get_current_position_y() + th)
       return

   n = int(th/h)
   s = (th - n*h)/(n + 1)

   set_current_position_y(get_current_position_y() + s)
   while n > 0:
       draw(e.box)
       set_current_position_y(get_current_position_y() + s)
       n -= 1
   ```

### `\halign`, `\valign`

`\halign` is used to make a table. It has the following syntax:
* `\halign` *\<box specification\>* *\<alignment material\>*

where *\<alignment material\>* is of the form:
1. *\<preamble\>* `\cr`
1. *\<line 1\>* `\cr`
1. *\<line 2\>* `\cr`
1. *etc.*
1. *\<line n\>* `\cr`
1. optional `\noalign`

*\<preamble\>* is separated into *templates* using a token of the category 4
as a delimiter. Each template must contain just one token of the category 6.

*\<line i\>* is separated into columns using a token of the category 4 as a
delimiter. If a token of the category 4, `\span`, or `\cr` appears inside a
group it is not treated as a delimiter.

`\cr` may be directly followed by `\noalign` `{` *\<vertical mode material\>*
`}`. In this case, *\<vertical mode material\>* is inserted right after the
recently finished line or right before the first line if the `\cr` finishes the
preamble.

`\crcr` command is:
* ignored if it is used right after `\cr` or `\noaling`
* treated as `\cr` elsewhere

Now, lets describe how `\halign` makes a table:
1. Open a group.
1. Load preample:
   * Preamble is loaded in unexpanded form (except `\tabskip` and `\span`) into
     a separate memory.
   * When scanning the preamble, TeX pays attention on these tokens:
     * `\cr` &ndash; ends the preamble;
     * `&` (category 4 token) &ndash; separates templates;
     * `#` (category 6 token) &ndash; where in the template an item should be
       inserted;
     * `\tabskip` &ndash; (1) expand tokens that follows `\tabskip` until
       *\<equals\>* *\<glue specification\>* is given and assign this value to
       `\tabskip`, (2) *ti* has the value of the `\tabskip` in the time of the
       end of scanning the *i*th template, *t0* has the value of the `\tabskip`
       in the time of entering `\halign`, (3) remove `\tabskip` *\<equals\>*
       *\<glue specification\>* from the preamble;
     * `\span` &ndash; the token that follows `\span` is expanded.
1. Convert column items to `\hbox`es:
   1. Let `&` denote a token of the category 4 and let `#` denote a token of
      the category 6. Additionally in line, let `&` denote also `\span`.
   1. Assume the preamble has the form: *x1* `#` *y1* `&` *x2* `#` *y2* `&` ...
      `&` *xn* `#` *yn* `\cr`.
      * A sequence *xi* `#` *yi* is called a *template*.
      * A template can be prefixed with `&` marking a template as a *next round
        starting point*. At most one `&`-marked template is allowed.
   1. Assume every line has the form: *z1* `&` *z2* `&` ... `&` *zm* `\cr`.
   1. Initial spaces and spaces after `&` are ignored.
   1. If *m* > *n* and the preamble has no `&`-marked template, fail with
      `Extra alignment tab has been changed to \cr`.
   1. Let *j* is the index of `&`-marked template. If there is no `&`-marked
      template, set *j* to 0. Define the index function, *f*(*i*), as follows:
      * If *j* = 0 or *i* <= *n*, then *f*(*i*) = *i*.
      * Otherwise set *p* = *j* - 1, *d* = *n* - *p*, *k* = (*i* - *p* - 1) div
        *d*; then *f*(*i*) = *i* - *kd*.
   1. For *i* in 1 to max(*n*, *m*):
      * If *i* <= *m* and the first non-space unexpandable token of *zi* is not
        `\omit`, make the *i*th `\hbox` with *xf*(*i*) *zi* *yf*(*i*) in its
        horizontal list. *xf*(*i*) *zi* *yf*(*i*) is expanded and interpreted
        within the `\hbox`'s group.
      * Otherwise, if *i* <= *m* and the first non-space unexpandable token of
        *zi* is `\omit`, make the *i*th `\hbox` with the fully-expanded and
        interpreted *zi* within `\hbox`'s group, except `\omit`, in its
        horizontal list.
      * Otherwise, make the *i*th `\hbox` empty.
      * The first token of *zi* is expanded before *xi zi yi* is assembled
        together because of the decision based on the `\omit` presence to be
        made. This implies that a macro at the beginning of *zi* can include
        multiple columns and lines to its parameter breaking the table.
      * *xi zi yi* (or *zi* if `\omit` took an effect) is seen by the expand
        processor as a continuous stream of tokens. This implies that macro
        parameter can start in *xi* and ends in *yi*, holding also the whole
        *zi*.
      * A `\noalign` as the first non-space unexpandable token following `\cr`
        is not a part of the `\hbox`'s horizontal list. It is recorded for the
        later use.
1. Let *wi* be the maximal natural width of all entries in the *i*th column.
1. Handle `\span`s:
   1. In some line, if two columns were delimited with `\span`, merge their
      `\hbox`es into one `\hbox`. Repeat until there are no `\hbox`es to be
      merged.
   1. Let *n* be the number of columns after spanning.
   1. For 1 <= *i* <= *j* <= *n*, let *wij* be the maximal natural width of all
      entries that span columns *i* through *j*, including *j*;
      * if there are no such spanned entries, then set *wij* to *-inf*.
   1. Let *tk* be the base width of `\tabskip` between columns *k* and (*k* +
      1), where 1 <= *k* < *n*.
   1. The final width *wj* of column *j* is computed by the formula
      * *wj* = max[1 <= *i* <= *j*](*wij* - sum[*i* <= *k* < *j*](*wk* + *tk*))

      for *j* going through 1 to *n*, including *n*.
   1. Let *wij* = *wi* + *ti* + *w*(*i* + 1) + *t*(*i* + 1) + ... + *wj* be the
      new width of spanned `\hbox` spanning columns *i* to *j*, including *j*.
1. Assemble lines and insert them to the vertical list:
   1. If the first `\cr` is directly followed by `\noalign`, insert the
      vertical mode material from `\noalign` to the vertical list.
   1. For *\<line 1\>*, insert a `\hbox` to the vertical list containing:
      * a glue *t0*;
      * the `\hbox` from the 1st column of this line, resized to *w1*;
      * a glue *t1*;
      * the `\hbox` from the 2nd column of this line, resized to *w2*;
      * a glue *t2*;
      * ...
      * the spanned `\hbox` spanning columns *i* to *j*, including *j*, resized
        to *wij*;
      * the glue *tj*;
      * ...
      * the `\hbox` from the *n*th, last, column of this line, resized to *wn*;
      * a glue *tn*.
   1. If the `\cr` that ends the *\<line 1\>* is directly followed by
      `\noalign`, insert the vertical mode material from `\noalign` to the
      vertical list.
   1. Repeat the previous two steps for the rest of lines.
   1. Width of every line `\hbox` is further adjusted if `\halign` is followed
      by `to` or `spread`:
      * if `to` *w* is given, then every `\hbox` is adjusted to the width *w*
        (`\tabskip`s get their final values);
      * if `spread` *s* is given, then the width of every `\hbox` is advanced
        by *s* (`\tabskip`s get their final values);
      * *wij* of `\hbox` spanning columns *i* to *j*, including *j*, is
        adjusted to accommodate the final value of `\tabskip` *k*, *i* <= *k* <
        *j*.
   1. If an item `\hbox` contains a `\vrule`:
      * if the `\vrule` has unspecified height, its height becomes the height
        of the line `\hbox`;
      * if the `\vrule` has unspecified depth, its depth becomes the depth of
        the line `\hbox`.
   1. If a vertical mode material from `\noalign` contains a `\hrule` with
      unspecified width, its width becomes the width of the final table.
1. Close the group.

`\valign` works like `\halign` except it makes a transposed table. Here are
major differences:
* column items are `\vbox`es
* `\cr` ends a column
* *hi* is the maximal natural height of all `\vbox` entries in the *i*th line
* the depth of a `\vbox` entry is zero; this is because the first step is the
  shift of the baseline to the `\vbox`'s bottom
* `\noalign` contains horizontal mode material
* `\vbox` entries are assembled into `\vbox` column and separated by `\tabskip`
* final `\vbox` columns are inserted into horizontal list with horizontal mode
  material from `\noalign` between them

`\halign` and `\valign` may contain each other.

### Entering/Leaving the Math Mode or Display Mode

When entering the display mode, do:
1. Break the current paragraph like in case of `\par` (append `\parfillskip` to
   the last line and contribute so far paragraph lines to the vertical list).
   Instead of `\widowpenalty`, use `\displaywidowpenalty`.
1. If there is no paragraph so far (e.g. `\noindent $$ ...` or `$$ $$` case):
   * Set *w* to `-\maxdimen`.

   Otherwise:
   * Let *w* is the natural width of the last line in the paragraph.
   * Add the amount of left indentation of the last line to *w*.
   * Add 2em to *w*.
   * If the last line contains a glue that makes that line stretchable or
     shrinkable:
     * Set *w* to `\maxdimen`.
1. If `\parshape` is defined:
   * Let *n* be the number of items in `\parshape`.
   * If `\prevgraf` + 2 < *n*, set *m* to `\prevgraf` + 2. Otherwise, set *m*
     to *n*.
   * Set *l* to the width of the *m*th item of `\parshape`.
   * Set *s* to the indent of the *m*th item of `\parshape`.

   Otherwise, if `\hangindent` is not 0 and ((`\hangafter` >= 0 and `\prevgraf`
   \+ 2 > `\hangafter`) or `\prevgraf` + 1 < `-\hangafter`):
   * Set *l* to `\hsize` - *abs*(`\hangindent`).
   * Set *s* to `\hangindent` if `\hangindent` > 0. Otherwise, set *s* to 0.

   Otherwise:
   * Set *l* to `\hsize`.
   * Set *s* to 0.
1. Open a group.
1. Set `\fam` to -1.
1. Set `\predisplaysize` to *w*.
1. Set `\displaywidth` to *l*.
1. Set `\displayindent` to *s*.
1. Insert tokens from `\everydisplay` to the token stream.
1. Set *C* to *D*.
1. Start [building a math list](#building-a-math-list).

When entering the math mode, do:
1. Open a group.
1. Set `\fam` to -1.
1. Insert tokens from `\everymath` to the token stream.
1. Set *C* to *T*.
1. Start [building a math list](#building-a-math-list).

When leaving the display mode, do:
1. End [building the math list](#building-a-math-list).
1. Check whether we are at the same group level as when entering the display
   mode.
1. Close the group.
1. [Convert the math list to the horizontal list](#converting-a-math-list-to-the-horizontal-list).
1. Pack the horizontal list into the horizontal box, *b*.
1. Set *w* to the width of *b*.
1. If the `\hbox` with the equation number has been produced:
   * Set *e* to the width of the `\hbox`.
   * Set *q* to *e* + `\fontdimen6\textfont2` (the width of the `\hbox` plus
     quad in text size).

   Otherwise, set both *e* and *q* to 0.
1. If *w* + *q* > `\displaywidth`:
   * If *e* is not 0 and *b* can be shrunk to `\displaywidth` - *q*:
     * Shrink *b* to `\displaywidth` - *q*.
   * Otherwise:
     * Set *e* to 0.
     * If *w* > `\displaywidth`, shrink *b* to `\displaywidth`.
   * Set *w* to the new width of *b*.
1. Set *d* to 0.5\*(`\displaywidth` - *w*).
1. If *e* > 0 and *d* < 2\**e*:
   * Set *d* to 0.5\*(`\displaywidth` - *w* - *e*).
   * If the horizontal list starts with a glue, set *d* to 0.
1. If *d* + `\displayindent` <= `\predisplaysize` or `\leqno` is given:
   * Set *ga* to `\abovedisplayskip`.
   * Set *gb* to `\belowdisplayskip`.

   Otherwise:
   * Set *ga* to `\abovedisplayshortskip`.
   * Set *gb* to `\belowdisplayshortskip`.
1. Emit `\penalty \predisplaypenalty` to the vertical list.
1. If *e* is 0 and `\leqno` is given:
   * `\moveright \displayindent` the `\hbox` with the equation number.
   * Add the `\hbox` to the vertical list.
   * Emit `\penalty 10000` to the vertical list.

   Otherwise, emit `\vskip` *ga* to the vertical list.
1. If *e* is not 0:
   * Set `\r` to `\displaywidth` - *w* - *e* - *d*.
   * Set `\a` to the `\hbox` with the equation number.
   * In case of `\leqno`:
     * Set `\b` to `\hbox{\copy \a \kern \r \box \b}`.
     * Set *d* to 0.
   * In case of `\eqno`:
     * Set `\b` to `\hbox{\box \b \kern \r \copy \a}`.
1. `\moveright` the box `\b` about `\displayindent` + *d* and add `\b` to the
   vertical list.
1. If *e* is 0 and `\eqno` is given:
   * Emit `\penalty 10000` to the vertical list.
   * `\moveright` the `\hbox` with the equation number about `\displayindent`
     plus `\displaywidth` minus the width of the `\hbox`.
   * Add the `\hbox` to the vertical list.
   * Set *gb* to 0.
1. Emit `\penalty \postdisplaypenalty` to the vertical list.
1. If *gb* > 0, emit `\vskip` *gb* to the vertical list.
1. Set `\prevgraf` to `\prevgraf` + 3.
1. Start a new empty horizontal list (like when starting a paragraph with
   `\noindent`) and switch to the horizontal mode.
1. Set `\spacefactor` to 1000.
1. Skip space after `$$`.

When leaving the math mode, do:
1. End [building the math list](#building-a-math-list).
1. Check whether we are at the same group level as when entering the math mode.
1. Close the group.
1. [Convert the math list to the horizontal list](#converting-a-math-list-to-the-horizontal-list).
1. Add a space (a math node) of the width `\mathsurround` both at the beginning
   and the end of the horizontal list.
1. Set `\spacefactor` to 1000.

For the meaning of *C*, *D*, and *T* see
[Converting a Math List to the Horizontal List](#converting-a-math-list-to-the-horizontal-list).

### Building a Math List

Math list is build up in math/display mode. Math list is consisting of these
elements:
* an atom &ndash; an atom is consisting of three parts:
  * a nucleus &ndash; an atom core
  * a superscript &ndash; a superscript of the atom core
  * a subscript &ndash; a subscript of the atom core

  each of these parts may contain a math symbol, a box, a math list, or be
  empty
* horizontal mode material (`\hbox`, `\vbox`, `\vtop`, `\vrule`, `\penalty`,
  `\discretionary`)
* vertical mode material (`\mark`, `\insert`, `\vadjust`, `\write`, `\special`)
* a glue (`\hskip`, `\mskip`, `\nonscript`)
* a kern (`\kern`, `\mkern`)
* a style change (`\displaystyle`, `\textstyle`, ...)
* a generalized fraction (`\above`, `\over`, ...)
* a boundary (`\left`, `\right`)
* a four-way choice (`\mathchoice`)

There are thirteen types of atoms, summarized in the following table:
| Id | Type | Meaning |
| -- | ---- | ------- |
| 0 | Ord | ordinary atom, like `x`, `y` |
| 1 | Op | large operator atom, like `\sum`, `\int`, `\lim` |
| 2 | Bin | binary operation atom, like `+`, `-` |
| 3 | Rel | relation atom, like `=`, `<` |
| 4 | Open | opening atom, like `(` |
| 5 | Close | closing atom, like `)` |
| 6 | Punct | punctuation atom, like `,` |
| 7 | Inner | inner atom, like `{1\over 2}` |
| 8 | Over | overline atom, like `\overline{x}` |
| 9 | Under | underline atom, like `\underline{x}` |
| 10 | Acc | accented atom, like `\mathaccent"7016 x` |
| 11 | Rad | radical atom, like `\radical"270370 x` |
| 12 | Vcent | `\vbox` to be centered, produced by `\vcenter` |

Atoms 0 to 6, inclusive, are made when TeX in math/display mode processes a
math symbol. A math symbol can be:
* a symbol with assigned `\mathcode`, like ``\mathcode`\-=X`` or
  ``\mathcode`\'="8000``
* a `\chardef`-defined control sequence
* `\char` command, like `\char C`
* a `\mathchardef`-defined control sequence, like `\mathchardef\alpha=X`
* `\mathchar` command, like `\mathchar X`
* `\delimiter` command, like `\delimiter D`

where `C` is an 8-bit number, `X` is a 15-bit number of a form
`<atom id><font family><character position>`, where
* `<atom id>` is a 3-bit number specifying an atom based on its id from the
  previous table except 7 which has a special meaning
* `<font family>` is a 4-bit number specifying a font family
* `<character position>` is an 8-bit number specifying a position of the
  character to be typeset in the given font

and `D` is a 27-bit number of a form
`<atom id><font family #1><character position #1><font family #2><character position #2>`,
where `<atom id>`, `<font family #x>`, and `<character position #x>` are
specified as usual.

Besides `\mathcode`, a symbol can have assigned also `\delcode`, which is a
24-bit number of a form
`<font family #1><character position #1><font family #2><character position #2>`.
A symbol with a positive `\delcode` assigned works, when associated with
`\left` or `\right`, as a delimiter.

IniTeX assigns each ASCII character `\mathcode<ASCII>=<ASCII>`, except letters
have `\mathcode<ASCII>="71<ASCII>` and digits have
`\mathcode<ASCII>="70<ASCII>`. Furthermore, IniTeX assigns each ASCII character
`\delcode<ASCII>=-1` (invalid delimiter), except `.` which has `\delcode` set
to 0 (null delimiter).

Atomic fields nucleus, superscript, and subscript are specified by *\<math
field\>*. *\<math field\>* is converted to the atomic field following these
rules:
1. If it is a math symbol, then the atomic field is that math symbol.
1. Otherwise, open a group, convert a math mode material to the math list, and
   close the group.
   1. If the math list is empty, then the atomic field is empty.
   1. Otherwise, if the math list contains a single Ord atom with no
      superscript or subscript, then the atomic field is the nucleus of this
      Ord atom. That is, `x^{y}` yields `x^y`.
   1. Otherwise, if the math list containing a single Acc atom is a nucleus of
      an Ord atom, then the Ord atom is replaced with that Acc atom. That is,
      `x^{\bar a}` yields `x^\bar a`.
   1. Otherwise, the math list is stored to the atomic field.

Lets describe how TeX converts a formula into the math list. First, some
conventions:
* `delcode(d)`, where `d` is a *\<delimiter\>*, returns a `\delcode` of `d` if
  `d` is a category 11 or 12 token; otherwise, it returns a lower 24 bits from
  the 27-bit number following `\delimiter`.
* `Symbol(family, position)` denotes a math symbol. `family` is a font family,
  `position` is the symbol position in a font.
* `Left(d)` and `Right(d)` denotes a left and right boundary item,
  respectively, where `d` is a 24-bit delimiter code.
* `Atom(id, nucleus, superscript, subscript)` denotes an atom. `id` is atom id;
  `nucleus`, `superscript`, and `subscript` are eponymous atomic fields. Empty
  field is denoted `None`. Some atoms also contain extra fields:
  * `Atom(Op, nucleus, superscript, subscript, limits)` &ndash; `limits` is an
    extra field specifying limits conventions, one of `DISPLAYLIMITS`
    (default), `LIMITS`, or `NOLIMITS`;
  * `Atom(Acc, nucleus, superscript, subscript, symbol)` &ndash; `symbol` is an
    extra field for `Symbol` holding an accent information;
  * `Atom(Rad, nucleus, superscript, subscript, delimiter)` &ndash; `delimiter`
    is an extra field holding *\<24-bit number\>* as a delimiter information.
* `Fraction(numerator, denominator, thickness, left, right)` denotes a
  generalized fraction. `numerator` and `denominator` are math lists (`None`
  denotes an empty list), `thickness` is the fraction line thickness (`DEFAULT`
  or measure), and `left` and `right` are delimiters (24-bit number or `None`
  if not present).
* `Choice(a, b, c, d)` denotes a choice item produced by `\matchoice`.

Now the method (all steps are done in the display/math mode):
1. If *\<space\>* is read, nothing happens.
1. If `\<space>` is read, append a `\hskip` *g*, where *g* is the same amount
   as produced by *\<space\>* in horizontal mode with space factor 1000, to the
   math list.
1. If `\hskip` *\<glue\>*, `\hfil`, `\hfill`, `\hss`, `\hfilneg`, or `\mskip`
   *\<muglue\>* is read, append the glue to the math list.
1. If `\leaders`, `\cleaders`, or `\xleaders`, followed by *\<box or rule\>*
   *\<mathematical glue\>*, is read, append the leaders to the math list.
   *\<mathematical glue\>* is one of `\hskip` *\<glue\>*, `\hfil`, `\hfill`,
   `\hss`, `\hfilneg`, or `\mskip` *\<muglue\>*.
1. If `\nonscript` is read, append the special zero-width glue to the math
   list. If the next item to be appended is glue and the `\nonscript` had been
   typeset in *script* style or *script script* style, then the glue is
   canceled.
1. If a category 11 or 12 token, `\chardef`-defined control sequence, or
   `\char` *\<8-bit number\>* is read, then:
   1. Replace the character number, `c`, by its `\mathcode` value, `v`.
   1. If `v` is `"8000`, put the active char (`c`, 13) back to the token
      stream.
   1. Otherwise, put `\mathchar v` to the token stream.
1. If `\delimiter` *\<27-bit number\>* is read, then:
   1. Put `\mathchar <atom id><font family #1><character position #1>` to the
      token stream.
1. If `\mathchar` *\<15-bit number\>* or `\mathchardef`-defined control
   sequence is read, then:
   1. Extract atom id, font family, and a character position from it as `id`,
      `family`, and `position`, respectively.
   1. If `id` is 7, then:
      * Set `id` to 0.
      * If 0 <= `\fam` <= 15, then set `family` to `\fam`.
   1. Append `Atom(id, Symbol(family, position), None, None)` to the math list.
1. If `\/` is read, append a `\kern` of zero-width to the math list.
1. If `\noboundary` is read, nothing happens.
1. If `{` *\<math mode material\>* `}` is read, then:
   1. Open a group, convert the math mode material to the math list `l`, and
      close the group.
   1. If `l` is empty, append `Atom(Ord, None, None, None)` to the math list.
   1. Otherwise, if `l` has only one element which is an Ord or Acc atom, then
      append this atom to the math list.
   1. Otherwise, append `Atom(Ord, l, None, None)` to the math list.
1. If `^` (a token of the category 7) *\<math field\>* is read, then:
   1. If the math list does not end with atom, append
      `Atom(Ord, None, None, None)` to the math list.
   1. If the superscript field of the last element of the math list is not
      empty, issue `Double superscript` error.
   1. Otherwise, change the superscript field to the result of the given
      *\<math field\>*.
1. If `_` (a token of the category 8) *\<math field\>* is read, then:
   1. If the math list does not end with atom, append
      `Atom(Ord, None, None, None)` to the math list.
   1. If the subscript field of the last element of the math list is not empty,
      issue `Double subscript` error.
   1. Otherwise, change the subscript field to the result of the given *\<math
      field\>*.
1. If `\displaylimits`, `\limits`, or `\nolimits` is read, then:
   1. If the last item in the math list is not an Op atom, complain.
   1. Otherwise, set the `limits` field of the Op atom to `DISPLAYLIMITS`,
      `LIMITS`, or `NOLIMITS` accordingly.
1. If *\<box\>* is read, then:
   1. If the constructed box is void, nothing happens.
   1. Otherwise, append `Atom(Ord, box, None, None)` to the math list.
1. If `\raise` *\<dimen\>* *\<box\>* or `\lower` *\<dimen\>* *\<box\>* is read,
   then the offset of *\<box\>* is adjusted accordingly and the next actions
   taken are those as if *\<box\>* had been read.
1. If `\mathord` *\<math field\>* is read, then:
   1. Append `Atom(Ord, r, None, None)`, where `r` is the result of the given
      math field, to the math list.
1. If `\mathop` *\<math field\>* is read, then:
   1. Append `Atom(Op, r, None, None)`, where `r` is the result of the given
      math field, to the math list.
1. If `\mathbin` *\<math field\>* is read, then:
   1. Append `Atom(Bin, r, None, None)`, where `r` is the result of the given
      math field, to the math list.
1. If `\mathrel` *\<math field\>* is read, then:
   1. Append `Atom(Rel, r, None, None)`, where `r` is the result of the given
      math field, to the math list.
1. If `\mathopen` *\<math field\>* is read, then:
   1. Append `Atom(Open, r, None, None)`, where `r` is the result of the given
      math field, to the math list.
1. If `\mathclose` *\<math field\>* is read, then:
   1. Append `Atom(Close, r, None, None)`, where `r` is the result of the given
      math field, to the math list.
1. If `\mathpunct` *\<math field\>* is read, then:
   1. Append `Atom(Punct, r, None, None)`, where `r` is the result of the given
      math field, to the math list.
1. If `\mathinner` *\<math field\>* is read, then:
   1. Append `Atom(Inner, r, None, None)`, where `r` is the result of the given
      math field, to the math list.
1. If `\overline` *\<math field\>* is read, then:
   1. Append `Atom(Over, r, None, None)`, where `r` is the result of the given
      math field, to the math list.
1. If `\underline` *\<math field\>* is read, then:
   1. Append `Atom(Under, r, None, None)`, where `r` is the result of the given
      math field, to the math list.
1. If `\left` *\<delimiter\>* *\<math mode material\>* `\right` *\<delimiter\>*
   is read, then:
   1. Open a group.
   1. Start a new math list `l`.
   1. Append `Left(dl)` to `l`, where `dl == delcode(<delimiter>)` for the
      `\left` *\<delimiter\>*.
   1. Process the *\<math mode material\>*, appending its elements to `l`.
   1. Append `Right(dr)` to `l`, where `dr == delcode(<delimiter>)` for the
      `\right` *\<delimiter\>*.
   1. Close the group.
   1. Append `Atom(Inner, l, None, None)` to the math list.
1. If `\over`, `\atop`, `\above` *\<dimen\>*, `\overwithdelims` *\<delimiter\>*
   *\<delimiter\>*, `\atopwithdelims` *\<delimiter\>* *\<delimiter\>*, or
   `\abovewithdelims` *\<delimiter\>* *\<delimiter\>* *\<dimen\>* is read,
   then:
   1. Save the current math list to `l`.
   1. If a special holding place, `f`, associated with the current group
      nesting level is not empty, complain (constructions like
      `{1\over 2\over 3}` are not allowed).
   1. Make `Fraction(l, None, th, dl, dr)`, where
      * if `\over` has been read: `th = DEFAULT`, `dl = dr = None`;
      * if `\atop` has been read: `th = 0`, `dl = dr = None`;
      * if `\above` *v* has been read: `th = v`, `dl = dr = None`;
      * if `\overwithdelims` *da* *db* has been read: `th = DEFAULT`,
        `dl = delcode(da)`, `dr = delcode(db)`;
      * if `\atopwithdelims` *da* *db* has been read: `th = 0`,
        `dl = delcode(da)`, `dr = delcode(db)`;
      * if `\abovewithdelims` *da* *db* *v* has been read: `th = v`,
        `dl = delcode(da)`, `dr = delcode(db)`;

      and store it to `f`.
   1. Clear the current math list.
   1. Process the math mode material following `\over` until matching `}`, `$`,
      or `\right`.
   1. Put the current math list to `f.denominator`.
   1. Replace the entire content of the current math list with `f`.
   1. If `f.numerator` starts with `Left(d)`:
      * Remove `Left(d)` from the `f.numerator` start.
      * Insert it before `f` in the current math list so the current math list
        have now two elements: `Left(d)` directly followed by `f`.
1. If `\mathaccent` *\<15-bit number\>* *\<math field\>* is read, then:
   1. Extract atom id, font family and character position from *\<15-bit
      number\>* and store them to `id`, `ff`, and `cp`, respectively.
   1. Append `Atom(Acc, r, None, None, Symbol(ff, cp))`, where `r` is the
      result of the given *\<math field\>*, to the math list.
1. If `\radical` *\<27-bit number\>* *\<math field\>* is read, then:
   1. Convert *\<27-bit number\>* to *\<24-bit number\>* by discarding the 3
      highest bits and store the result to `d`.
   1. Append `Atom(Rad, r, None, None, d)`, where `r` is the result of the
      given *\<math field\>*, to the math list.
1. If `\vcenter` *\<box specification\>* `{` *\<vertical mode material\>* `}`
   is read:
   1. Form a `\vbox`, `b`, as if `\vcenter` had been `\vbox`.
   1. Append `Atom(Vcent, b, None, None)` to the math list.
1. If `\mathchoice` *\<filler\>* `{` *\<math mode material\>* `}` *\<filler\>*
   `{` *\<math mode material\>* `}` *\<filler\>* `{` *\<math mode material\>*
   `}` *\<filler\>* `{` *\<math mode material\>* `}` is read, append
   `Choice(a, b, c, d)` to the math list, where `a`, `b`, `c`, and `d` are
   results of `{` *\<math mode material\>* `}` as if it had been *\<math
   field\>*.
1. If `\displaystyle`, `\textstyle`, `\scriptstyle`, or `\scriptscriptstyle` is
   read, append this style change item to the mast list.
1. If `\eqno` or `\leqno`, followed by *\<math mode material\>* `$` is read,
   then:
   1. If the current mode is not the display mode, complain.
   1. Open a group.
   1. Insert tokens from `\everymath` to the token stream.
   1. Enter the math mode.
   1. Convert *\<math mode material\>* to the horizontal list and store it to
      the `\hbox` `b` that will be used as the equation number of the current
      display. Do not add spaces implied by `\mathsurround`.
   1. Close the group.
   1. Insert `$` back to the token stream, where it will terminate the display
      mode.
1. If `\halign` *\<box specification\>* `{` *\<alignment material\>* `}` is
   read, then:
   1. If the mode is not the display mode or the current math list is not
      empty, complain.
   1. If `}` is not followed by *\<assignment\>* commands except `\setbox` or
      `$$`, complain.
   1. Process optional *\<assignment\>* commands.
   1. Process `$$` &ndash; close group, exit display mode, skip the following
      space.
   1. Insert `\penalty \predisplaypenalty` to the vertical list.
   1. Insert `\vskip \abovedisplayskip` to the vertical list.
   1. Insert lines resulted from the `\halign` to the vertical list, each line
      is shifted right about `\displayindent`.
   1. Insert `\penalty \postdisplaypenalty` to the vertical list.
   1. Insert `\vskip \belowdisplayskip` to the vertical list.
   1. Set `\prevgraf` to `\prevgraf` + 3.
   1. Set `\spacefactor` to 1000.
   1. Continue in the horizontal mode.
1. If `\vrule` *\<rule specification\>* is read, append it to the math list.
1. If `\indent` is read, append `Atom(Ord, b, None, None)`, where `b` is an
   empty `\hbox` of the width `\parindent`, to the math list.
1. If `\noindent` is read, nothing happens.
1. If `\discretionary` *\<general text\>* *\<general text\>* *\<general text\>*
   is read, then:
   1. Check that the last *\<general text\>* produces an empty list.
   1. Append the `\discretionary` to the math list.
1. If `\-` is read, treat it as `\discretionary{-}{}{}`
1. If `\unhbox` *\<8-bit number\>* or `\unhcopy` *\<8-bit number\>* is read,
   then:
   1. If a box is not void, complain.
   1. Otherwise, nothing happens.
1. If `$` is read, then:
   1. In display mode, read another `$` that must follows.
   1. Close the group.
   1. Finish the math list and convert it into a horizontal list.
   1. After completing a displayed formula, read one optional space.

### Converting a Math List to the Horizontal List

Lets first describe parameters and auxiliary algorithms involved in
math-to-horizontal list conversion.

#### Fonts and Styles

There are eight styles in math formulas:
* a display style, denoted *D*
* a text style, denoted *T*
* a script style, denoted *S*
* a script-script style, denoted *SS*
* a cramped display style, denoted *D'*
* a cramped text style, denoted *T'*
* a cramped script style, denoted *S'*
* a cramped script-script style, denoted *SS'*

The current style will be denoted *C*.

Transition between styles are realized either by `\displaystyle`, `\textstyle`,
`\scriptstyle`, or `\scriptscriptstyle` to set *C* to *D*, *T*, *S*, or *SS*,
respectively, or implicitly by the rules summarized in the following table:
| *C* | Superscript | Subscript | Numerator | Denominator |
| --- | ----------- | --------- | --------- | ----------- |
| *D* | *S* | *S'* | *T* | *T'* |
| *T* | *S* | *S'* | *S* | *S'* |
| *S* | *SS* | *SS'* | *SS* | *SS'* |
| *SS* | *SS* | *SS'* | *SS* | *SS'* |
| *D'* | *S'* | *S'* | *T'* | *T'* |
| *T'* | *S'* | *S'* | *S'* | *S'* |
| *S'* | *SS'* | *SS'* | *SS'* | *SS'* |
| *SS'* | *SS'* | *SS'* | *SS'* | *SS'* |

The ordering of these styles is *D* > *D'* > *T* > *T'* > *S* > *S'* > *SS* >
*SS'*. Define *C'* to be *X'* if and only if *C* = *X* or *C* = *X'*.
Furthermore, define *C^* and *C_* to be the superscript and subscript style for
*C*, respectively. Note that *C_* is (*C^*)'.

Math mode algorithms are dealing with up to 16 font families ranging from 0 to
15 inclusive. Each font family contains three fonts:
* `\textfont` for math formulas at ordinary level (not in subscripts and
  superscripts, styles *D*, *D'*, *T*, and *T'*)
* `\scriptfont` for math formulas at the first subscript and superscript level
  (styles *S* and *S'*)
* `\scriptscriptfont` for math formulas at the second and more subscript and
  superscript level (styles *SS* and *SS'*)

Font families 0, 1, 2, and 3 follow these conventions:
* Font family 0 is for roman letters used in math formulas.
* Font family 1 is for math italic letters used in math formulas.
* Font family 2 is for math symbols.
* Font family 3 is for math extensions.

Font families 2 and 3 have special meaning. TeX requires that the font family 2
must have these parameters specified:
* `\fontdimen8` &ndash; numerator up-shift relative to the axis in *D* or *D'*
* `\fontdimen9` &ndash; numerator up-shift relative to the axis in *T*, *T'*,
  *S*, *S'*, *SS*, or *SS'*
* `\fontdimen10` &ndash; like `\fontdimen9`, used in case of `\atop` or
  `\atopwithdelims`
* `\fontdimen11` &ndash; denominator down-shift relative to the axis in *D* or
  *D'*
* `\fontdimen12` &ndash; denominator down-shift relative to the axis in *T*,
  *T'*, *S*, *S'*, *SS*, or *SS'*
* `\fontdimen13` &ndash; superscript up-shift in *D*
* `\fontdimen14` &ndash; superscript up-shift in *T*, *S*, or *SS*
* `\fontdimen15` &ndash; superscript up-shift in *D'*, *T'*, *S'*, or *SS'*
* `\fontdimen16` &ndash; subscript down-shift if the superscript is empty
* `\fontdimen17` &ndash; subscript down-shift if the superscript is not empty
* `\fontdimen18` &ndash; used to compute a minimal superscript up-shift
* `\fontdimen19` &ndash; used to compute a minimal subscript down-shift
* `\fontdimen20` &ndash; minimal delimiter size in *D* or *D'*
* `\fontdimen21` &ndash; minimal delimiter size in *T*, *T'*, *S*, *S'*, *SS*,
  or *SS'*
* `\fontdimen22` &ndash; baseline to axis distance measured in up-wards
  direction

Additionally, TeX requires that the font family 3 must have these parameters
specified:
* `\fontdimen8` &ndash; default rule line thickness (`\overline`, `\underline`,
  fractions)
* `\fontdimen9` &ndash; spacing around big operators #1
* `\fontdimen10` &ndash; spacing around big operators #2
* `\fontdimen11` &ndash; spacing around big operators #3
* `\fontdimen12` &ndash; spacing around big operators #4
* `\fontdimen13` &ndash; spacing around big operators #5

#### Auxiliary Algorithms

* *Set box x to the y field in style z* algorithm. In the conversion algorithm,
  this will be referred as *field_to_box*(*y*, *z*):
  1. If *y* field is empty, *x* is set equal to a null (empty) `\hbox`.
  1. If *y* field contains a symbol `\Sym`:
     1. Set *x* to `\hbox{\Font\Sym}`, where `\Font` is the font associated
        with *z*.
     1. Add the italic correction from `\Font` associated with `\Sym` to the
        width of *x*.
  1. If *y* field contains a math or horizontal list:
     1. If *y* field contains a math list, convert it to a horizontal list with
        the starting style *z*.
     1. Let `\Content` be the horizontal list, either given or converted.
     1. Remove stretching and shrinking from glues in `\Content`.
     1. If `\Content` is a `\hbox`, set *x* to `\Content`.
     1. Otherwise, set *x* to `\hbox{\Content}`.
     1. If *x* contains a single character, remove unneeded italic correction.
* *Set box x to a variable delimiter d, having z minimum height plus depth*
  algorithm (in C++ like pseudo-code). In the conversion algorithm, this will
  be referred as *delimiter_to_box*(*d*, *s*, *z*):
  ```C++
  enum DelimiterVariant {
    SMALL_VARIANT,
    LARGE_VARIANT
  };

  enum MathFontSize {
    TEXT_SIZE,
    SCRIPT_SIZE,
    SCRIPT_SCRIPT_SIZE
  };

  // Structure representing a symbol.
  struct Symbol {
    Symbol(Font f, CharPos c) : font(f), cpos(c) {}
    Dimen width();
    Dimen italic_correction();
    Dimen height();
    Dimen depth();
    Symbol get_repeatable_part();
    Symbol get_top_part();
    Symbol get_middle_part();
    Symbol get_bottom_part();

    Font font;
    CharPos cpos;
  };

  // Return true if `s` exists in the associated font.
  bool symbol_exists(Symbol & s);
  // Return true if `s` is extensible within the associated font.
  bool is_extensible(Symbol & s);
  // If `s` has the successor in the associated font, return it. Otherwise,
  // return the non-existing symbol (Symbol(nullfont, 0)).
  Symbol symbol_successor(Symbol & s);
  // Extract <font family #1> (v == SMALL_VARIANT) or <font family #2>
  // (v == LARGE_VARIANT) from `d`.
  FontFamily get_font_family(Delimiter d, DelimiterVariant v);
  // Extract <character position #1> (v == SMALL_VARIANT) or <character position #2>
  // (v == LARGE_VARIANT) from `d`.
  CharPos get_character_position(Delimiter d, DelimiterVariant v);
  // If `sz == TEXT_SIZE`, return `\textfont ff`.
  // If `sz == SCRIPT_SIZE`, return `\scriptfont ff`.
  // If `sz == SCRIPT_SCRIPT_SIZE`, return `\scriptscriptfont ff`.
  // Otherwise, return `nullfont`.
  Font get_family_font(FontFamily ff, MathFontSize sz);

  // Enclose `s` with \hbox.
  Box symbol_to_box(Symbol s)
  {
    Box b = HBox();

    b.append(s);
    b.width = s.width() + s.italic_correction();
    b.height = s.height();
    b.depth = s.depth();
    return b;
  }

  // Return true if `symbol` or some of its successors is extensible or its
  // total height is at least `expected_height`. Update `result` and
  // `max_height_so_far` accordingly.
  bool best_delimiter_lookup_inner(
    Symbol symbol, Dimen expected_height, Symbol & result, Dimen & max_height_so_far
  )
  {
    while (symbol_exists(symbol)) {
      if (is_extensible(symbol)) {
        result = symbol;
        return true;
      }
      Dimen height = symbol.height() + symbol.depth();
      if (height > max_height_so_far) {
        result = symbol;
        max_height_so_far = height;
        if (height >= expected_height)
          return true;
      }
      symbol = symbol_successor(symbol);
    }
    return false;
  }

  // Lookup for the best fitting delimiter and store it to `best_delimiter`.
  void best_delimiter_lookup(
    Delimiter delimiter, MathFontSize size, Dimen expected_height,
    Symbol & best_delimiter
  )
  {
    struct {
      FontFamily ff;
      CharPos cp;
    } delimiters[2] = {
      {
        get_font_family(delimiter, SMALL_VARIANT),
        get_character_position(delimiter, SMALL_VARIANT)
      },
      {
        get_font_family(delimiter, LARGE_VARIANT),
        get_character_position(delimiter, LARGE_VARIANT)
      }
    };

    Dimen max_height_so_far = 0;

    for (int i = 0; i < 2; i++) {
      auto d = delimiters[i];

      if (d.ff == 0 && d.cp == 0)
        continue;

      auto sz = size;
      while (sz >= TEXT_SIZE) {
        auto f = get_family_font(d.ff, sz);

        if (
          f != nullfont
          && best_delimiter_lookup_inner(
            Symbol(f, d.cp), expected_height, best_delimiter, max_height_so_far
          )
        ) return;
        sz--;
      }
    }
  }

  // The algorithm itself. Here `delimiter` coincides with `d`, `size` is the
  // current math size derived from the current math style, `expected_height`
  // coincides with `z`, and the return value coincides with `x`.
  Box delimiter_to_box(Delimiter delimiter, MathFontSize size, Dimen expected_height)
  {
    Symbol best_delimiter = Symbol(nullfont, 0);

    best_delimiter_lookup(delimiter, size, expected_height, best_delimiter);

    // If no suitable delimiter was found, return an empty \hbox with width set
    // to \nulldelimiterspace.
    if (best_delimiter.font == nullfont) {
      Box b = HBox();
      b.width = nulldelimiterspace;
      return b;
    }

    // Compose delimiter from parts.
    if (is_extensible(best_delimiter)) {
      // Repeatable part (always present).
      Symbol rep = best_delimiter.get_repeatable_part();
      // Top, middle, and bottom parts (optional).
      Symbol top = best_delimiter.get_top_part();
      Symbol mid = best_delimiter.get_middle_part();
      Symbol bot = best_delimiter.get_bottom_part();

      // Guess how many parts are needed to fulfill `expected_height`.
      Dimen th = 0, rep_th = rep.height() + rep.depth();
      int nreps = 0;

      if (top.font != nullfont)
        th += top.height() + top.depth();
      if (mid.font != nullfont)
        th += mid.height() + mid.depth();
      if (bot.font != nullfont)
        th += bot.height() + bot.depth();
      if (rep_th > 0) {
        while (th < expected_height) {
          th += (mid.font == nullfont) ? rep_th : rep_th*2;
          nreps++;
        }
      }

      // Compose the \vbox with the delimiter.
      Box b = VBox();

      if (bot.font != nullfont)
        // Insert a box to the `b`'s vertical list begin.
        b.pushbox(symbol_to_box(bot));
      for (int i = 0; i < nreps; i++)
        b.pushbox(symbol_to_box(rep));
      if (mid.font != nullfont) {
        b.pushbox(symbol_to_box(mid));
        for (int i = 0; i < nreps; i++)
          b.pushbox(symbol_to_box(rep));
      }
      if (top.font != nullfont)
        b.pushbox(symbol_to_box(top));

      // Set box dimensions.
      b.width = rep.width() + rep.italic_correction();
      b.height = b.list.empty() ? 0 : b.list[0].height;
      b.depth = th - b.height;

      return b;
    }

    // Delimiter is a symbol.
    return symbol_to_box(best_delimiter);
  }
  ```
* *Rebox a given box to a given width* algorithm (in C++ like pseudo-code). In
  the conversion algorithm, this will be referred as *rebox*(*b*, *w*):
  ```C++
  // Return true if `x` is `Symbol`.
  bool is_symbol(Node & x);
  // Return true if `b` is \vbox.
  bool is_vbox(Box & b);

  // Rebox a given box `b` to a given width `w` and return the reboxed box.
  Box rebox(Box b, Dimen w)
  {
    // The `b`'s width matches `w` => nothing to do.
    if (b.width == w)
      return b;

    // `b` is empty => set `b`'s width to `w`.
    if (b.list.empty()) {
      b.width = w;
      return b;
    }

    // We are going to unpack `b` unless `b` is a \vbox. Therefore, pack \vbox
    // into protective \hbox.
    if (is_vbox(b))
      b = HBox(b);

    // Unpack the \hbox (get the horizontal list).
    List l = b.list.copy();

    // If there is a just one symbol, add a \kern for an italic correction.
    if (l.length() == 1 && is_symbol(l[0]) && l[0].width() != b.width)
      l += Kern(b.width - l[0].width());

    // Center the horizontal list `l` in a new \hbox of width `w`.
    return HBox(Hss() + l + Hss(), w);
  }
  ```

#### The Conversion

Here is the conversion algorithm itself. The algorithm uses auxiliary
algorithms introduced in [Auxiliary Algorithms](#auxiliary-algorithms) and also
several auxiliary functions and macros:
* *abs*(*x*) returns the absolute value of *x*.
* *floor*(*x*) returns the integral part of *x*.
* *deref*(*p*) returns the value which *p* points to.
* *prev*(*I*) refers to the previous item in the math list immediately
  preceding *I*.
* *next*(*I*) refers to the next item in the math list immediately following
  *I*.
* *size*(*S*) maps the math font style *S* to the math font size as follows:
  * *size*(*D*) = *size*(*D'*) = *size*(*T*) = *size*(*T'*) = `TEXT_SIZE`
  * *size*(*S*) = *size*(*S'*) = `SCRIPT_SIZE`
  * *size*(*SS*) = *size*(*SS'*) = `SCRIPT_SCRIPT_SIZE`
* *get_family_font*(*ff*, *sz*) returns
  * `\textfont`*ff* if *sz* is `TEXT_SIZE`
  * `\scriptfont`*ff* if *sz* is `SCRIPT_SIZE`
  * `\scriptscriptfont`*ff* if *sz* is `SCRIPT_SCRIPT_SIZE`
* *symbol_exists*(*font*, *cp*) returns true if a symbol exists on a given
  character position *cp* in a given *font*.
* *is_symbol*(*x*) returns true if *x* is a single character.
* `Font::kern_amount(CharPos cp1, CharPos cp2)` is a method of `Font` class
  representing a TeX font. It returns a kern amount for a kerning pair (`cp1`,
  `cp2`) if it is defined in the font's kerning table. Otherwise, it returns 0.
  If `cp2` is `SKEWCHAR` it returns a kern amount between `cp1` and
  `\skewchar`.
* *char*(*font*, *cp*) returns a single character from *font* at *cp*.
* *charpos*(*c*) returns the character position of *c* in the given font.
* *charbox*(*c*) returns a character (horizontal) box for *c* with this
  properties:
  * the list of the box contains only one element, *c*, defined as
    *char*(*font*, *cp*);
  * the width of the box is the width of *c* plus *italic_correction*(*c*);
  * the height of the box is the height of *c*;
  * the depth of the box is the depth of *c*.
* *width*(*c*) returns the width of a single character *c*.
* *has_successor*(*c*) returns true if *c* has a successor in the given font.
* *successor*(*c*) returns the successor of *c*.
* *min*(*a1*, *a2*, ..., *an*) returns the lesser of *a1*, *a2*, ..., *an*.
* *max*(*a1*, *a2*, ..., *an*) returns the greater of *a1*, *a2*, ..., *an*.
* *get_charinfo*(*font*, *cp*) returns *charinfo*[*cp*], where *charinfo* is an
  array of character information of a given *font*. If *font* or *charinfo*[
  *cp*] are undefined, issue an error. Character information is a 4 bytes
  structure having the following layout:
  * *width_index* (1 byte)
  * *height_index* (4 bits)
  * *depth_index* (4 bits)
  * *italic_index* (6 bits)
  * *tag* (2 bits)
  * *remainder* (1 byte)
* *has_ligkern_program*(*info*) returns true if *info.tag* is 1.
* *get_ligkern_program*(*font*, *info*) returns pointer to *lig_kern* array of
  *font*. Issue an error if the pointer cannot be computed or is not valid
  (e.g. outside of *lig_kern* array). The pointer is computed as follows:
  * Set *prog* to *lig_kern* + *info.remainder*.
  * Set *inst* to *deref*(*prog*).
  * If *inst.skip* <= 128, return *prog*.
  * Otherwise, return *lig_kern* + (256 * *inst.op*) + *inst.rem*.

  An item of *lig_kern* array has the following layout:
  * *skip* (1 byte): the number of intervening steps to be skipped; if the
    number is 128 or greater, it means that this step is final
  * *next_char* (1 byte): do *op* and stop if *next_char* follows the current
    character, continue otherwise
  * *op* (1 byte): ligature step if less than 128, kern step otherwise
  * *rem* (1 byte): remainder
* *is_kern_op*(*inst*) returns true if *inst.op* >= 128.
* *get_kern_from_kern_op*(*font*, *inst*) return a kern from *kern* array of
  *font* at index 256 * (*inst.op* - 128) + *inst.rem*.
* *italic_correction*(*font*, *cp*) returns italic correction for a character
  at position *cp* in font *font*.
* *italic_correction*(*c*) returns *italic_correction*(*font of c*, *character
  position of c*).
* `\Cfont` expands to `\textfont`, `\scriptfont`, or `\scriptscriptfont`
  depending on the current math font size used.

The conversion algorithm:
* **[Step 0.]** Let *I* be the current (initially the first) item in the math
  list. If *I* points behind the math list, go to **End of the first pass**.
* **[Case 1.]** If *I* is a rule (`\hrule`, `\vrule`), `\discretionary`,
  `\penalty`, `\special`, `Left(d)`, or `Right(d)`:
  * Set *I* to *next*(*I*) and go to **Step 0**.
* **[Case 2.]** If *I* is a glue or kern:
  * If *I* is a glue that comes from `\nonscript` and *C* <= *S* and the
    *next*(*I*) is glue or kern:
    * Remove *next*(*I*) from the math list.
  * Otherwise, if *I* is `\mskip` or `\mkern`:
    * Convert `\mskip` or `\mkern` to `\hskip` or `\kern`, respectively, by
      converting all measures in `mu` to `pt` using the following formula:
      *units_in_pt* = 1.0/18 * `\fontdimen6\Cfont2` * *units_in_mu*
  * Set *I* to *next*(*I*) and go to **Step 0**.
* **[Case 3.]** If *I* is `\displaystyle`, `\textstyle`, `\scriptstyle`, or
  `\scriptscriptstyle`:
  * Set *C* to *D*, *T*, *S*, or *SS*, respectively.
  * Set *I* to *next*(*I*) and go to **Step 0**.
* **[Case 4.]** If *I* is `Choice(a, b, c, d)`:
  * Replace *I* with `a`, `b`, `c`, or `d`, depending on *C*.
  * Set *I* to the next unprocessed item (*next*(*prev*(*I*))) and go to **Step
    0**.
* **[Case 5.]** If *I* is `Atom(Bin, ...)`:
  * If *I* is the first `Atom` or the previous `Atom` was `Bin`, `Op`, `Rel`,
    `Open`, or `Punct`:
    * Change *I* to `Atom(Ord, ...)`.
    * Go to **Case 14**.
  * Otherwise, go to **Step 17**.
* **[Case 6.]** If *I* is `Atom(Rel, ...)`, `Atom(Close, ...)`, or
  `Atom(Punct, ...)`:
  * If the previous `Atom` was `Bin`:
    * Change that previous `Atom` to `Ord`.
  * Go to **Step 17**.
* **[Case 7.]** If *I* is `Atom(Open, ...)` or `Atom(Inner, ...)`:
  * Go to **Step 17**.
* **[Case 8.]** If *I* is `Atom(Vcent, nucleus, ...)`:
  * Let `nucleus` be `\vbox` with height plus depth equal to `v`.
  * Let `a` is `\fontdimen22\Cfont2`.
  * Set `nucleus.height` to 0.5*`v` + `a`.
  * Set `nucleus.depth` to 0.5*`v` - `a`.
  * Change *I*'s type (`id`) to `Ord`.
  * Go to **Step 17**.
* **[Case 9.]** If *I* is `Atom(Over, nucleus, ...)`:
  * Set box `\x` to *field_to_box*(`nucleus`, *C'*).
  * Set `\d` to `\fontdimen8\Cfont3`.
  * Set box `\y` to `\vbox{\kern \d \hrule height \d \kern 3\d \copy \x}`.
  * Replace `nucleus` with `\y`.
  * Go to **Step 16**.
* **[Case 10.]** If *I* is `Atom(Under, nucleus, ...)`:
  * Set box `\x` to *field_to_box*(`nucleus`, *C*).
  * Set `\d` to `\fontdimen8\Cfont3`.
  * Set box `\y` to `\vtop{\copy \x \kern 3\d \hrule height \d}`.
  * Do `\advance \dp\y by \d`.
  * Replace `nucleus` with `\y`.
  * Got to **Step 16**.
* **[Case 11.]** If *I* is `Atom(Rad, nucleus, _, _, delimiter)`:
  * Set box `\x` to *field_to_box*(`nucleus`, *C'*).
  * Set `\d` to `\fontdimen8\Cfont3`.
  * Set `\e` to
    * `\fontdimen5\Cfont2` if *C* > *T*;
    * `\d` otherwise.
  * Set `\f` to `\d` + 0.25\**abs*(`\e`).
  * Set box `\y` to *delimiter_to_box*(`delimiter`, *size*(*C*), `\ht\x` +
    `\dp\x` + `\f` + `\d`).
  * Set `\d` to `\ht\y`.
  * If `\dp\y` > `\ht\x` + `\dp\x` + `\f`:
    * Set `\f` to 0.5*(`\f` + `\dp\y` - `\ht\x` - `\dp\x`).
  * Set box `\z` to `\vbox{\kern \d \hrule height \d \kern \f \copy \x}`.
  * Set `\r` to `\ht\x` + `\f`.
  * Replace `nucleus` with `\raise \r \box \y \box \z`.
  * Go to **Step 16**.
* **[Case 12.]** If *I* is `Atom(Acc, nucleus, superscript, subscript,
    Symbol(ff, cp))`:
  * Set `\af` to *get_family_font*(`ff`, *size*(*C*)).
  * If not *symbol_exists*(`\af`, `cp`):
    * Go to **Step 16**.
  * Set box `\x` to *field_to_box*(`nucleus`, *C'*).
  * Set `u` to `\wd\x`.
  * Set `s` to
    * 0 if not *is_symbol*(`nucleus`);
    * `get_family_font(nucleus.ff, size(C)).kern_amount(nucleus.cp, SKEWCHAR)`
      otherwise.
  * Set `\ac` to *char*(`\af`, `cp`).
  * While *has_successor*(`\ac`) and *width*(*successor*(`\ac`)) < `u`:
    * Set `\ac` to *successor*(`\ac`).
  * Set `\d` to *min*(`\ht\x`, `\fontdimen5\af`).
  * If *is_symbol*(`nucleus`):
    * Set `nucleus` to `[Atom(Ord, nucleus, superscript, subscript)]`.
    * Set both `superscript` and `subscript` to `None`.
    * Set `\t` to `\ht\x`.
    * Set box `\x` to *field_to_box*(`nucleus`, *C*).
    * Set `\d` to `\d` + (`\ht\x` - `\t`).
  * Set box `\y` to *charbox*(`\ac`).
  * Set `\t` to `s` + 0.5*(`u` - `\wd\y`).
  * Set box `\z` to `\vbox{\moveright \t \copy \y \kern -\d \copy \x}`.
  * If `\ht\z` < `\ht\x`:
    * Set `\t` to `\ht\x` - `\ht\z`.
    * Set box `\z` to `\vbox{\kern \t \unvbox \z}`.
    * Set `\ht\z` to `\ht\x`.
  * Set `\wd\z` to `\wd\x`.
  * Replace `nucleus` with `\z`.
  * Go to **Step 16**.
* **[Case 13.]** If *I* is `Atom(Op, nucleus, superscript, subscript, limits)`:
  * If `limits` is `DISPLAYLIMITS` and *C* > *T*:
    * Set `limits` to `LIMITS`.
  * If not *is_symbol*(`nucleus`):
    * Set `\d` to 0.
    * Go to **Step 13a**.
  * Set `\f` to *get_family_font*(`nucleus.ff`, *size*(*C*)).
  * Set `\c` to *char*(`\f`, `nucleus.cp`).
  * If *C* > *T* and *has_successor*(`\c`):
    * Set `\c` to *successor*(`\c`).
  * Set `nucleus.cp` to *charpos*(`\c`).
  * Set box `\x` to *field_to_box*(`nucleus`, *C*).
  * Set `\d` to *italic_correction*(`\f`, `nucleus.cp`).
  * If `limits` is `LIMITS` or `subscript` is `None`:
    * Do `\advance \wd\x by \d`.
  * Let `a` be `\fontdimen22\Cfont2`.
  * Set `\t` to 0.5*(`\ht\x` - `\dp\x`) - `a`.
  * Replace `nucleus` with `\lower \t \box \x`.
* **[Step 13a.]**
  * If `limits` is not `LIMITS`:
    * Go to **Step 17**.
  * Set box `\x` to *field_to_box*(`superscript`, *C^*).
  * Set box `\y` to *field_to_box*(`nucleus`, *C*).
  * Set box `\z` to *field_to_box*(`subscript`, *C_*).
  * Set `w` to *max*(`\wd\x`, `\wd\y`, `\wd\z`).
  * Set box `\x` to *rebox*(`\x`, `w`).
  * Set box `\y` to *rebox*(`\y`, `w`).
  * Set box `\z` to *rebox*(`\z`, `w`).
  * Set box `\v` to `\vbox{\copy \y}`.
  * If `superscript` is not `None`:
    * Set `\k` to *max*(`\fontdimen9\Cfont3`, `\fontdimen11\Cfont3` - `\dp\x`).
    * Set `\l` to `\fontdimen13\Cfont3`.
    * Set `\t` to 0.5*`\d`.
    * Set box `\v` to `\vbox{\kern \l \moveright \t \copy \x \kern \k \unvbox
      \v}`.
  * If `subscript` is not `None`:
    * Set `\k` to *max*(`\fontdimen10\Cfont3`, `\fontdimen12\Cfont3` -
      `\ht\z`).
    * Set `\l` to `\fontdimen13\Cfont3`.
    * Set `\t` to 0.5*`\d`.
    * Set box `\v` to `\vbox{\unvbox \v \kern \k \moveleft \t \copy \z \kern
      \l}`.
  * Replace `nucleus` with `\v`.
  * Set *I* to *next*(*I*) and go to **Step 0**.
* **[Case 14.]** If *I* is `Atom(Ord, nucleus, superscript, subscript)`:
  * Unless *is_symbol*(`nucleus`) and `superscript` is `None` and `subscript`
    is `None` and *next*(*I*) is `Atom(X, nucleusA, superscriptA, subscriptA)`,
    where `X` is `Ord`, `Op`, `Bin`, `Rel`, `Open`, `Close`, or `Punct`, and
    *is_symbol*(`nucleusA`) and `nucleus.ff` is `nucleusA.ff`:
    * Go to **Step 17**.
  * Mark `nucleus` as a *text symbol*.
  * Set *font* to *get_family_font*(`nucleus.ff`, *size*(*C*)).
  * Set *info* to *get_charinfo*(*font*, `nucleus.cp`).
  * If not *has_ligkern_program*(*info*):
    * Go to **Step 17**.
  * Set *c_next* to `nucleusA.cp`.
  * Set *prog* to *get_ligkern_program*(*font*, *info*).
  * Set *inst* to *deref*(*prog*).
  * **[loop]** If *inst.next_char* is *c_next* and *inst.skip* <= 128:
    * If *is_kern_op*(*inst*):
      * Set *k* to *get_kern_from_kern_op*(*font*, *inst*).
      * Insert `\kern` *k* between *I* and *next*(*I*).
      * Go to **Step 17**.
    * Otherwise:
      * If *inst.op* is:
        * 1 or 5, then set `nucleus.cp` to *inst.rem*.
        * 2 or 6, then set `nucleusA.cp` to *inst.rem*.
        * 3 or 7 or 11, then
          * let `nucleusB` be `Symbol(nucleus.ff, inst.rem)`;
          * let *R* be `Atom(Ord, nucleusB, None, None)`;
          * if *inst.op* is 11, mark `nucleusB` as a *text symbol*;
          * insert *R* between *I* and *next*(*I*).
        * 0, then
          * set `nucleus.cp` to *inst.rem*;
          * set `superscript` to `superscriptA`;
          * set `subscript` to `subscriptA`;
          * remove *next*(*I*).
      * If *inst.op* > 3:
        * Go to **Step 17**.
      * Unmark `nucleus` as a *text symbol*.
      * Go to **Case 14**.
  * If *inst.skip* >= 128:
    * Go to **Step 17**.
  * Set *prog* to *prog* + *inst.skip* + 1.
  * Set *inst* to *deref*(*prog*).
  * Go to **loop**.
* **[Case 15.]** If *I* is `Fraction(numerator, denominator, \thickness, left,
  right)`:
  * If `\thickness` is `DEFAULT`:
    * Set `\thickness` to `\fontdimen8\Cfont3`.
* **[Step 15a.]**
  * If *C* is *D*, set *s* to *T*.
  * Otherwise, if *C* is *D'*, set *s* to *T'*.
  * Otherwise, set *s* to *C^*.
  * Set box `\x` to *field_to_box*(`numerator`, *s*).
  * If *C* > *T*, set *s* to *T'*.
  * Otherwise, set *s* to *C_*.
  * Set box `\z` to *field_to_box*(`denominator`, *s*).
  * If `\wd\x` < `\wd\z`, set box `\x` to *rebox*(`\x`, `\wd\z`).
  * If `\wd\z` < `\wd\x`, set box `\z` to *rebox*(`\z`, `\wd\x`).
* **[Step 15b.]**
  * If *C* > *T*:
    * Set `\u` to `\fontdimen8\Cfont2`.
    * Set `\v` to `\fontdimen11\Cfont2`.
  * Otherwise:
    * If `\thickness` is not 0, set `\u` to `\fontdimen9\Cfont2`.
    * If `\thickness` is 0, set `\u` to `\fontdimen10\Cfont2`.
    * Set `\v` to `\fontdimen12\Cfont2`.
* **[Step 15c.]** If `\thickness` is 0:
  * If *C* > *T*, set `\f` to `7\fontdimen8\Cfont3`.
  * If *C* <= *T*, set `\f` to `3\fontdimen8\Cfont3`.
  * Set `\g` to (`\u` - `\dp\x`) - (`\ht\z` - `\v`).
  * If `\g` < `\f`:
    * Set `\u` to `\u` + 0.5\*(`\f` - `\g`).
    * Set `\v` to `\v` + 0.5\*(`\f` - `\g`).
  * Set `\k` to (`\u` + `\v`) - (`\dp\x` + `\ht\z`).
  * Set box `\y` to `\vbox{\copy \x \kern \k \copy \z}`.
  * Set `\ht\y` to `\ht\x` + `\u`.
  * Set `\dp\y` to `\dp\z` + `\v`.
* **[Step 15d.]** If `\thickness` is not 0:
  * If *C* > *T*, set `\f` to 3\*`\thickness`.
  * If *C* <= *T*, set `\f` to `\thickness`.
  * Set `\a` to `\fontdimen22\Cfont2`.
  * Set `\g` to (`\u` - `\dp\x`) - (`\a` + 0.5\*`\thickness`).
  * If `\g` < `\f`, set `\u` to `\u` + (`\f` - `\g`).
  * Set `\h` to (`\a` - 0.5\*`\thickness`) - (`\ht\z` - `\v`).
  * If `\h` < `\f`, set `\v` to `\v` + (`\f` - `\h`).
  * Set `\k` to `\u` - `\dp\x` - `\a` - 0.5\*`\thickness`.
  * Set `\l` to `\v` - `\ht\z` + `\a` - 0.5\*`\thickness`.
  * Set box `\y` to `\vbox{\copy \x \kern \k \hrule height \thickness \kern \l
    \copy \z}`.
  * Set `\ht\y` to `\ht\x` + `\u`.
  * Set `\dp\y` to `\dp\z` + `\v`.
* **[Step 15e.]**
  * If *C* > *T*, set *h* to `\fontdimen20\Cfont2`.
  * If *C* <= *T*, set *h* to `\fontdimen21\Cfont2`.
  * Set box `\x` to *delimiter_to_box*(`left`, *size*(*C*), *h*).
  * Set box `\z` to *delimiter_to_box*(`right`, *size*(*C*), *h*).
  * Set `\r` to 0.5\*(`\dp\x` - `\ht\x`) + `\a`.
  * Set `\s` to 0.5\*(`\dp\z` - `\ht\z`) + `\a`.
  * Replace *I* with `Atom(Inner, \raise \r \box \x \box \y \raise \s \box \z,
    None, None)`.
  * Set *I* to *next*(*I*) and go to **Step 0**.
* **[Step 16.]**
  * Let *I* be `Atom(X, ...)`. Change `X` to `Ord`.
* **[Step 17.]** Let *I* be `Atom(_, nucleus, superscript, subscript)`:
  * If `nucleus` is a math list *L*:
    * Convert *L* to a horizontal list *H* in style *C*.
    * Set box `\x` to the `\hbox` containing *H*.
    * Replace `nucleus` with `\x`.
  * If not *is_symbol*(`nucleus`):
    * Go to **Step 18**.
  * Set `\f` to *get_family_font*(`nucleus.ff`, *size*(*C*)).
  * Set `\c` to *char*(`\f`, `nucleus.cp`).
  * If `nucleus` is not marked as a *text symbol* or `\fontdimen2\f` is 0:
    * Set `\d` to *italic_correction*(`\f`, `nucleus.cp`).
  * Otherwise, set `\d` to 0.
  * If `\d` is not 0 and `subscript` is `None`:
    * Replace `nucleus` with `\c \kern \d`.
    * Set `\d` to 0.
  * Otherwise, replace `nucleus` with `\c`.
* **[Step 18.]**
  * If `superscript` is `None` and `subscript` is `None`:
    * Set *I* to *next*(*I*) and go to **Step 0**.
* **[Step 18a.]**
  * If `nucleus` is a character box optionally followed by a kern:
    * Set `\u` to 0.
    * Set `\v` to 0.
  * Otherwise, `nucleus` is a box `\x`:
    * Set `\qf` to *get_family_font*(2, *size*(*C^*)).
    * Set `\rf` to *get_family_font*(2, *size*(*C_*)).
    * Set `q` to `\fontdimen18\qf`.
    * Set `r` to `\fontdimen19\rf`.
    * Set `\u` to `\ht\x` - `q`.
    * Set `\v` to `\dp\x` + `r`.
* **[Step 18b.]** If `superscript` is `None`:
  * Set box `\x` to *field_to_box*(`subscript`, *C_*).
  * Set `\wd\x` to `\wd\x` + `\scriptspace`.
  * Set `\s` to *max*(`\v`, `\fontdimen16\Cfont2`, `\ht\x` -
    0.8\**abs*(`\fontdimen5\Cfont2`)).
  * Append `\lower \s \box \x` to the horizontal list of `nucleus`.
  * Set *I* to *next*(*I*) and go to **Step 0**.
* **[Step 18c.]**
  * Set box `\x` to *field_to_box*(`superscript`, *C^*).
  * Set `\wd\x` to `\wd\x` + `\scriptspace`.
  * Set `p` to
    * `\fontdimen13\Cfont2` if *C* = *D*;
    * `\fontdimen15\Cfont2` if *C* = *C'*;
    * `\fontdimen14\Cfont2` otherwise.
  * Set `\u` to *max*(`\u`, `p`, `\dp\x` + 0.25\**abs*(`\fontdimen5\Cfont2`)).
* **[Step 18d.]**
  * If `subscript` is `None`:
    * Append `\raise \u \box \x` to the horizontal list of `nucleus`.
    * Set *I* to *next*(*I*) and go to **Step 0**.
  * Set box `\y` to *field_to_box*(`subscript`, *C_*).
  * Set `\wd\y` to `\wd\y` + `\scriptspace`.
  * Set `\v` to *max*(`\v`, `\fontdimen17\Cfont2`).
* **[Step 18e.]**
  * Set `\t` to `\fontdimen8\Cfont3`.
  * If (`\u` - `\dp\x`) - (`\ht\y` - `\v`) >= 4\*`\t`:
    * Go to **Step 18f**.
  * Set `\v` to 4\*`\t` + `\ht\y` - (`\u` - `\dp\x`).
  * Set `\p` to 0.8\**abs*(`\fontdimen5\Cfont2`) - (`\u` - `\dp\x`).
  * If `\p` > 0:
    * Set `\u` to `\u` + `\p`.
    * Set `\v` to `\v` - `\p`.
* **[Step 18f.]**
  * If `\d` is not known from **Case 13** or **Step 17**:
    * Set `\d` to 0.
  * Set `\k` to (`\u` + `\v`) - (`\dp\x` + `\ht\y`).
  * Set box `\z` to `\vbox{\moveright \d \box \x \kern \k \box \y}`.
  * Set `\ht\z` to `\ht\x` + `\u`.
  * Set `\dp\z` to `\dp\y` + `\v`.
  * Append `\box \z` to the horizontal list of `nucleus`.
  * Set *I* to *next*(*I*) and go to **Step 0**.
* **[End of the first pass.]**
  * Let `Atom(X, ...)` be the last atom.
  * If `X` is `Bin`, then set `X` to `Ord`.
  * Set *C* to the value that *C* had when first entering **Step 0**.
* **[Step 19.]** If the math list begins with `Left(dl)` and ends with
  `Right(dr)`:
  * Let `\x` be a `\hbox` containing every box and rule found in the math list
    or in the nucleus of an atom from the math list. The order of appearance of
    boxes/rules is kept. Note that this procedure uses `\hbox{...}` algorithm
    and hence rules gets their final dimensions.
  * Set `\hmax` to `\ht\x` and `\dmax` to `\dp\x`.
  * Set `a` to `\fontdimen22\Cfont2`.
  * Set `\d` to *max*(`\hmax` - `a`, `\dmax` + `a`).
  * Set `g` to *max*(*floor*(`\d`/500)\*`\delimiterfactor`, 2\*`\d` -
    `\delimitershortfall`).
  * Set box `\x` to *delimiter_to_box*(`dl`, *size*(*C*), `g`).
  * Set box `\z` to *delimiter_to_box*(`dr`, *size*(*C*), `g`).
  * Set `\u` to 0.5\*(`\dp\x` - `\ht\x`) + `a`.
  * Set `\v` to 0.5\*(`\dp\z` - `\ht\z`) + `a`.
  * Replace `Left(dl)` with `Atom(Open, \raise \u \box \x, None, None)`.
  * Replace `Right(dr)` with `Atom(Close, \raise \v \box \z, None, None)`.
  * Let *L* be the current math list.
  * Set *L* to `[Atom(Inner, L, None, None)]`.
* **[Step 20.]** Iterate over the math list for the second time, denote the
  current item as *I*:
  * If *I* is `Atom(Y, ...)` and there exists an atom, `Atom(X, ...)`, that is
    the closest predecessor of *I*:
    * If `X` (`Y`) is not one of `Ord`, `Op`, `Bin`, `Rel`, `Open`, `Close`,
      `Punct`, or `Inner`:
      * Set `X` (`Y`) to `Ord`.
    * Set `\s` to `None`.
    * Let *Z* be a value selected from the following table at [`X`, `Y`]
      ([row, column]):
      | `X`/`Y` | `Ord` | `Op` | `Bin` | `Rel` | `Open` | `Close` | `Punct` | `Inner` |
      | ------- | ----- | ---- | ----- | ----- | ------ | ------- | ------- | ------- |
      | `Ord` | 0 | 1 | (2) | (3) | 0 | 0 | 0 | (1) |
      | `Op` | 1 | 1 | \* | (3) | 0 | 0 | 0 | (1) |
      | `Bin` | (2) | (2) | \* | \* | (2) | \* | \* | (2) |
      | `Rel` | (3) | (3) | \* | 0 | (3) | 0 | 0 | (3) |
      | `Open` | 0 | 0 | \* | 0 | 0 | 0 | 0 | 0 |
      | `Close` | 0 | 1 | (2) | (3) | 0 | 0 | 0 | (1) |
      | `Punct` | (1) | (1) | \* | (1) | (1) | (1) | (1) | (1) |
      | `Inner` | (1) | 1 | (2) | (3) | (1) | 0 | (1) | (1) |
    * If *Z* is 1, set `\s` to `\thinmuskip`.
    * Otherwise, if *Z* is (1) and *C* > *S*, set `\s` to `\thinmuskip`.
    * Otherwise, if *Z* is (2) and *C* > *S*, set `\s` to `\medmuskip`.
    * Otherwise, if *Z* is (3) and *C* > *S*, set `\s` to `\thickmuskip`.
    * If `\s` is not `None`:
      * Set `\s` to `\s` * 1.0/18 * `\fontdimen6\Cfont2`.
      * Insert `\hskip \s` right before *I*.
  * Otherwise, if *I* is `\displaystyle`, `\textstyle`, `\scriptstyle`, or
    `\scriptscriptstyle`:
    * Set *C* to *D*, *T*, *S*, or *SS*, respectively.
    * Remove *I* from the math list.
* **[Step 21.]** If the math list is part of a paragraph, iterate over the math
  list again, denote the current item as *I*:
  * If *I* is `Atom(X, ...)`, where `X` is `Bin` or `Rel`:
    * If `X` is `Bin`, set `\p` to `\binoppenalty`.
    * Otherwise, set `\p` to `\relpenalty`.
    * If *next*(*I*) and `\p` < 10000 and *next*(*I*) is not `\penalty` and
      *next*(*I*) is not `Atom(Rel, ...)`:
      * Insert `\penalty \p` right after *I*.
* **[Step 22.]**
  * Let *H* be empty horizontal list.
  * For every item *I* in the math list:
    * If *I* is not an atom, append *I* to *H*.
    * Otherwise, *I* is `Atom(_, nucleus, ...)`, where `nucleus` must be a
      horizontal list. Append the content of `nucleus` to *H*.
  * Return *H*.

### Breaking Terminology

*Discardable* and *nondiscardable* elements. Given the types of elements
(nodes) that can appear in a horizontal or vertical list:
* a box or a rule
* a `\discretionary` node
* a whatsit node (produced by `\special` or `\setlanguage`)
* vertical material (produced by `\mark` or `\vadjust` or `\insert`)
* a glue or `\leaders`
* a kern
* a penalty
* a math node (produced by `$...$` and `\mathsurround`)

The last four elements are *discardable*, the rest is *non-discardable*. The
exception to this rule is a penalty with a value lesser or equal to -10000 is
non-discardable in the horizontal list.

### `\discretionary` Breakpoints

`\discretionary` has three parameters:
```tex
\discretionary{<pre-break>}{<post-break>}{<no-break>}
```
* all three parameters must contain only boxes, rules, and kerns
* when a line/word is broken at discretionary
  * `<pre-break>` goes to the end of the line
  * `<post-break>` goes to the start of the next line
* `<no-break>` goes to the place where `\discretionary` is used when there is
  no break in that place
* in math/display mode
  * `<pre-break>` and `<post-break>` are interpreted in horizontal mode
  * `<no-break>` must be empty

`\-` is a shorthand for `\discretionary{\char <hyphenchar>}{}{}`:
* `\char <hyphenchar>` is a hyphenation character of the current font
  * for the font `\somefont` a hyphenation character is chosen by
    `\hyphnechar\somefont=<hyphenchar>` assignment
* if `<hyphenchar>` is -1, no hyphenation in the associated font is allowed;
  more precisely, if `<hyphenchar>` is outside of 0 to 255, `\-` is a shorthand
  for `\discretionary{}{}{}`

TeX inserts `\discretionary{}{}{}` after every `\char <hyphenchar>` and after
every ligature that ends with `\char <hyphenchar>`.

### Hyphenation

In (unrestricted) horizontal mode, whenever is a character to be added to the
horizontal list:
* Let *l* be `(\language <= 0 || \language > 255 ? 0 : \language)`.
* If *current_language* is not equal to *l*:
  * Set *current_language* to *l*.
  * Insert a whatsit node `WhatsItNode(LANGUAGE, l, \lefthyphenmin,
    \righthyphenmin)` to the horizontal list right before the just added
    character.

Using the `\setlanguage`\<number\> primitive, the `LANGUAGE` whatsit node can
be inserted to the horizontal list explicitly, even in restricted horizontal
mode. This command also set *current_language* to \<number\> (\<number\> is
normalized in the same way as `\language`).

Given a horizontal list, *H*, a word to be hyphenated is found following these
steps:
1. **[Find the Starting Letter]** Walk through *H* until a glue item that is
   not in a math formula is encountered and call it *g*.
   * From *g*, walk through *H* until an item which
     * is not a character with the zero `\lccode`
     * is not a ligature starting with a character with the zero `\lccode`
     * is not a whatsit node
     * is not an implicit kern

     is encountered and call it *c*.
   * If a `LANGUAGE` whatsit node has been walked through on the path to *c*,
     update *current_language*, `\lefthyphenmin`, and `\righthyphenmin`
     accordingly.
   * If all of this is fulfilled:
     * *c* is a character with a nonzero `\lccode` or a ligature starting with
       such a character
     * the `\lccode` of *c* is *c* or `\uchyph` is positive

     then *c* is the *starting letter*.
   * Otherwise, go to **Find the Starting Letter** and continue the journey
     from *c*.
1. **[Draft a Trial Word]** Let *c* be in font *f*.
   * If the `\hyphenchar` of *f* is not in range from 0 to 255, inclusive, go
     to **Find the Starting Letter** and continue the journey from *c*.
   * From *c*, walk through *H* until none of these are encountered:
     * (a) a character in font *f* with nonzero `\lccode`
     * (b) a ligature formed entirely from characters of type (a)
     * (c) an implicit kern
   * Call *p* the recent position in *H*.
   * The trial word *w* is a sequence of characters and implicit kerns in *H*
     starting from *c*, inclusive, and ending at *p*, exclusive.
1. **[Test the Trial Word]** Express the trial word *w* as *w*[1] *w*[2] ...
   *w*[*n*], where *n* is the number of characters of *w* and *w*[*i*] is the
   *i*th character of *w*.
   * Let *l* is *max*(1, `\lefthyphenmin`) and *r* is *max*(1,
     `\righthyphenmin`).
   * If *n* < *l* + *r*, go to **Find the Starting Letter** and continue the
     journey from *p*.
   * Suppose that *x y* is a sequence of items immediately following *w*[*n*]
     such that
     * *x* consists of zero or more characters, ligatures, and implicit kerns;
     * *y* is glue, explicit kern, penalty, whatsit, `\mark`, `\insert`, or
       `\vadjust`.
   * If such *x y* does not exist, go to **Find the Starting Letter** and
     continue the journey from *p*.
   * Otherwise, *w* is a word that can be hyphenated.

The hyphenation is then made following these steps:
1. Let *w* be a word to be hyphenated.
1. Let *E* be the *exception dictionary* number *current_language*.
1. Convert *w* using `\lccode` to *w'*.
1. If *w'* matches any word in `\lccode` form from the *E*:
   * Hyphenate *w* by inserting `\-` into places where `-` occurs in the
     matching word, giving a special treatment to (implicit) kerns and
     ligatures.
1. Otherwise, hyphenate *w* using [Hyphenation
   Algorithm](#hyphenation-algorithm).
1. Finally, ensure that these parts of *w* does not contain `\-`:
   * the prefix of *w* of the length *max*(1, `\lefthyphenmin`)
   * the suffix of *w* of the length *max*(1, `\righthyphenmin`)

#### `\hyphenation`

`\hyphenation{`\<words\>`}` adds \<words\> to the *exception dictionary* whose
number corresponds to *current_language*.
* \<words\> is a sequence of \<word\> items separated by spaces.
* A \<word\> is a sequence of \<hletter\> and \<hyphen\> items.
* A \<hyphen\> is a token (`-`, 12).
* A \<hletter\> is one of:
  * a character token of category 11 or 12;
  * a `\chardef` defined control sequence;
  * a `\char`\<8-bit number\>;

  such that the corresponding character has nonzero `\lccode`.
* A \<word\> is converted to its `\lccode` form before it is added to the
  *exception dictionary*.
* There can be up to 256 *exception dictionaries*, numbered from 0 to 255.
* The `\language` register denotes which *exception dictionary* is currently in
  use. Changing the value of this register picks up another *exception
  dictionary*. Assigning a value outside of 0 to 255 to this register is the
  same as assigning the zero.
* The change is global and additive.
* If two or more same words are added, the hyphenation of the most recent one
  is used.

#### Hyphenation Algorithm

1. Let `W` be a word to be hyphenated.
1. Let `n` be the length of `W`. For `1 <= i <= n`, let `W[i]` be the `i`th
   symbol of `W`.
1. Let `W'` be a sequence `. I[1] W[1] I[2] W[2] ... I[n] W[n] I[n+1] .`, where
   `I[i] = 0` is an interletter value meaning the desirability of hyphen on
   the `I[i]`'s position, for all `1 <= i <= n+1`.
1. Let `P` be the *pattern dictionary* whose number is *current_language*.
1. For every pattern `Ip[1] Wp[1] Ip[2] Wp[2] ... Ip[m] Wp[m] Ip[m+1]` in `P`,
   where `Wp[i]`, `1 <= i <= m`, is a symbol and `Ip[j]`, `1 <= j <= m+1`, is
   an interletter value, for some `m >= 1`, do:
   * If `Wp[i] = W[k+i-1]` for all `1 <= i <= m` and some `k >= 1` (that is,
     the pattern is a subword of `W`), then:
     * For all `1 <= i <= m+1`, if `Ip[i] > I[k+i-1]`, set `I[k+i-1]` to
       `Ip[i]`.
1. For all `1 <= i <= n+1`, if `I[i]` is odd, insert `\-` into `W` at the
   position of `I[i]`, giving a special treatment to (implicit) kerns and
   ligatures.

#### `\patterns`

`\patterns{`\<patterns\>`}` makes the *pattern dictionary*, whose number
corresponds to *current_language*, from \<patterns\>.
* The command is available only in IniTeX and it is not additive.
* \<patterns\> is a sequence of \<pattern\> items separated by spaces.
* A \<pattern\> is a sequence of one or more \<value\>\<pletter\>, followed by
  \<value\>.
* A \<value\> is either a digit ((`0`, 12) to (`9`, 12)) or empty. Empty
  \<value\> stands for 0.
* A \<pletter\> is a character token of category 11 or 12 with nonzero
  `\lccode`.
  * `.` (the dot character) is treated as \<pletter\> with code 0 (TeX uses
    code 0 to represent the left or right edge of a word being hyphenated).
* A \<pattern\> is converted to its `\lccode` form before it is added to the
  *pattern dictionary*.
* There can be up to 256 *pattern dictionaries*, numbered from 0 to 255.
* The `\language` register denotes which *pattern dictionary* is currently in
  use. Changing the value of this register picks up another *pattern
  dictionary*. Assigning a value outside of 0 to 255 to this register is the
  same as assigning the zero.

### Making Paragraphs

In a horizontal list, a *break point*, or a *line break*, can occur at these
five places:
* **(a)** at glue (its left edge) that is immediately preceded by a
  non-discardable element (penalty: 0)
  * glues inside a math formula do not count
* **(b)** at a kern that is immediately followed by glue (penalty: 0)
  * kerns inside a math formula do not count
* **(c)** at a math node that closes a math formula if the node is immediately
  followed by glue (penalty: 0)
* **(d)** at a penalty (penalty: explicitly given)
  * a penalty less or equal to -10000 forces a line break
  * a penalty greater or equal to 10000 forbids a line break
* **(e)** at a discretionary node (penalty: `\hyphenpenalty` if the pre-break
  text is nonempty, `\exhyphenpenalty` otherwise)

Define the *width of the ith line* and the *indentation (left offset) of the
ith line* of a paragraph as follows:
* If `\parshape` is specified, then
  * let *s[1] w[1] s[2] w[2] ... s[n] w[n]* be the `\parshape` specification
    (see [Line Breaking Parameters Summary](#line-breaking-parameters-summary)
    below);
  * if 1 <= *i* <= *n*, then
    * the indentation of the *i*th line is *s[i]*;
    * the width of the *i*th line is *w[i]*;
  * if *i* > *n*, then
    * the indentation of the *i*th line is *s[n]*;
    * the width of the *i*th line is *w[n]*.
* Otherwise, if `\hangafter` < 0, then
  * if 1 <= *i* <= *abs*(`\hangafter`), then
    * the indentation of the *i*th line is *max*(`\hangindent`, 0);
    * the width of the *i*th line is `\hsize` - *abs*(`\hangindent`);
  * if *i* > *abs*(`\hangafter`), then
    * the indentation of the *i*th line is 0;
    * the width of the *i*th line is `\hsize`.
* Otherwise,
  * if 1 <= *i* <= `\hangafter`, then
    * the indentation of the *i*th line is 0;
    * the width of the *i*th line is `\hsize`;
  * if *i* > `\hangafter`, then
    * the indentation of the *i*th line is *max*(`\hangindent`, 0);
    * the width of the *i*th line is `\hsize` - *abs*(`\hangindent`).

Define a function *break*(*L*) returning a list of lines:
* Let *L* be a horizontal list with *n+1* given breakpoints.
* Let *L'* denote *L* where all discardable elements following the breakpoint
  up to the next breakpoint or non-discardable element were removed.
* Express *L'* as *b[0] a[1] b[1] a[2] b[2] ... a[n] b[n]*, where *b[i]* is
  the *i*th breakpoint and *a[i]* is the *i*th chunk of horizontal material, 1
  <= *i* <= *n*. Note that line breaking algorithm always ensures that *L'*
  ends with a breakpoint. Line breaking algorithm also adds implicit breakpoint
  *b[0]* to the beginning of *L'*.
* If `\leftskip` is non-zero, set *a[i]* to `\hskip \leftskip` *a[i]* for every
  1 <= *i* <= *n*.
* For every 1 <= *i* <= *n*, append `\hskip \rightskip` to *a[i]*.
* Let *l[i]*, the (`\prevgraf` + *i*)th line of the paragraph, be `\moveright
  \s \hbox to \w {`*a[i]*`}`, where `\s` is the indentation and `\w` is the
  width of the (`\prevgraf` + *i*)th line of the paragraph.
* Return *l[1], l[2], ..., l[n]*.

Recall the *badness*(*line*) function:
* Let *we* be the expected width of *line* and *wn* be the natural width of
  *line*.
* Set *s* to *we* - *wn*.
* Let *total_stretch*[*i*] be the sum of all stretch values of order *i* of all
  glues inside *line*. Similarly for *total_shrink*[*i*]. Recall that order *i*
  is:
  * 0 if the value has a unit convertible to `pt`;
  * 1 if the value has the `fil` unit;
  * 2 if the value has the `fill` unit;
  * 3 if the value has the `filll` unit.
* Define *rank*(*x*) to be
  * 3 if *x*[3] is not 0;
  * otherwise, 2 if *x*[2] is not 0;
  * otherwise, 1 if *x*[1] is not 0;
  * otherwise, 0 if *x*[0] is not 0;
  * otherwise, -1.
* If *s* = 0, *badness*(*line*) is 0.
* If *s* < 0 (shrinking), *badness*(*line*) is
  * 0 if *rank*(*total_shrink*) > 0;
  * otherwise, infinity if *rank*(*total_shrink*) < 0 or
    *abs*(*s*) > *total_shrink*[0];
  * otherwise, *min*(100|*s*/*total_shrink*[0]|^3, 10000).
* If *s* > 0 (stretching), *badness*(*line*) is
  * 0 if *rank*(*total_stretch*) > 0;
  * otherwise, 10000 if *total_stretch*[0] <= 0;
  * otherwise, *min*(100|*s*/*total_stretch*[0]|^3, 10000).

Define a *fitness*(*line*) function as follows:
* Set *b* to *badness*(*line*).
* *fitness*(*line*) is
  * 3 (tight) if *b* >= 13 while shrinking;
  * 2 (decent) if *b* < 13;
  * 1 (loose) if 13 <= *b* < 100 while stretching;
  * 0 (very loose) if *b* >= 100 while stretching.

Define *visually_incompatible*(*line1*, *line2*) as *abs*(*fitness*(*line1*) -
*fitness*(*line2*)) > 1.

Define *demerits*(*line*) function as follows:
* Let *breakpoint* be the breakpoint associated (i.e. immediately after) with
  *line*.
* Let *p* be a penalty at *breakpoint*.
* Set *b* to *badness*(*line*).
* *demerits*(*line*) is
  * (`\linepenalty` + *b*)^2 + *p*^2, if 0 <= *p* < 10000;
  * (`\linepenalty` + *b*)^2 - *p*^2, if -10000 < *p* < 0;
  * (`\linepenalty` + *b*)^2, if *p* <= -10000.

Define *total_demerits*(*L*) function as follows:
* Set *d* to 0.
* For every *line* in *L*, add *demerits*(*line*) to *d*.
* For every pair of consecutive lines, *line1* and *line2*, in *L*:
  * If *visually_incompatible*(*line1*, *line2*), add `\adjdemerits` to *d*.
  * If both *line1* and *line2* ends with discretionary breaks, add
    `\doublehyphendemerits` to *d*.
* If the second last line from *L* ends with a discretionary, add
  `\finalhyphendemerits` to *d*.
* Return *d*.

Define *penalty*(*j*, *P*) function as follows:
* Let *n* be the number of elements (lines) in *P*.
* Set *p* to `\interlinepenalty`.
* If *j* = 1, add `\clubpenalty` to *p*.
* If *j* = *n* - 1 and the *n*th line immediately precedes the display, add
  `\displaywidowpenalty` to *p*.
* If *j* = *n* - 1 and the *n*th line does not immediately precede a display,
  add `\widowpenalty` to *p*.
* If the *j*th line in *P* ended at a discretionary break, add `\brokenpenalty`
  to *p*.
* Return *p*.

When TeX processes `\par` primitive or display appears in a horizontal mode it
converts the current horizontal list to the paragraph by following these steps:
1. **[Prepare]** Let *L* be the current horizontal list and *V* be the current
   vertical list.
   * If *V* is not empty and `\parskip` is not zero, append `\vskip \parskip`
     to *V*.
   * If *L* ends with a glue item, remove it.
   * Append `\penalty 10000 \hskip \parfillskip \penalty -10000` to *L*.
   * Set *nreq* to *unset*.
1. **[Set Line Width]** Define the initial width of each line of the paragraph
   to be the sum of bases of `\leftskip` and `\rightskip`.
1. **[Find Breakpoints (A)]** Find breakpoints in *L* such that:
   * a breakpoint is not chosen at a discretionary node
   * if *nreq* is not *unset*, *break*(*L*) has exactly *nreq* lines
   * for every *line* in *break*(*L*), *badness*(*line*) <= `\pretolerance`
1. If **Find Breakpoints (A)** succeeded, go to **Break**.
1. Hyphenate *L* as described in [Hyphenation](#hyphenation).
1. **[Find Breakpoints (B)]** Find breakpoints in *L* such that:
   * if *nreq* is not *unset*, *break*(*L*) has exactly *nreq* lines
   * for every *line* in *break*(*L*), *badness*(*line*) <= `\tolerance`
1. If **Find Breakpoints (B)** succeeded, go to **Break**.
1. **[Find Breakpoints (C)]** If `\emergencystretch` > 0, find breakpoints in
   *L* such that:
   * if *nreq* is not *unset*, *break*(*L*) has the number of lines as close to
     *nreq* as possible
   * the *total_stretch[i]*, 0 <= *i* <= 3, of every line in *break*(*L*) is
     increased about `\emergencystretch`
   * for every *line* in *break*(*L*), *badness*(*line*) <= `\tolerance`
1. If **Find Breakpoints (C)**, or eventually **Find Breakpoints (B)** did not
   succeed:
   * some lines from *break*(*L*) may be overfull
   * if *nreq* is not *unset*, *break*(*L*) has the number of lines as close to
     *nreq* as possible
1. **[Break]** Let *Ls* be set of *break*(*L*) results satisfying either (A) or
   (B) or (C) or none of them.
   * Let *P* be an element of *Ls* such that *total_demerits*(*P*) <=
     *total_demerits*(*X*) for all *X* in *Ls*. That is, *P* is an element from
     *Ls* with the smallest total demerits. If breakpoints were found using
     (C), *total_demerits* are computed with `\emergencystretch` kept in mind
     and after that computation the badness of each line is returned to its
     pre-`\emergencystretch` value. Note that as a consequence, given a
     sequence of penalties, the line breaking algorithm prefers the smallest
     one.
   * If `\looseness` is not 0 and *nreq* is *unset*:
     * Set *nreq* to the number of elements (lines) in *P* increased about
      `\looseness`.
     * Go to **Find Breakpoints (X)**, where **(X)** is either (A) or (B) or
       (C), depending in which pass the optimum breakpoints were found.
   * Then, *P* is the list of lines that will form a final paragraph.
   * Increase `\prevgraf` about the number of elements (lines) in *P*.
1. **[Emit Lines]** Emit all lines from *P* to *V* using these rules:
   * Insert a glue (`\vskip`) between two adjacent lines as described in
     [How Spaces Are Inserted to the Vertical List](#how-spaces-are-inserted-to-the-vertical-list).
   * Given that two adjacent lines are numbered *j* and (*j* + 1),
     respectively, let *p* be *penalty*(*j*, *P*). If *p* is not 0, insert
     `\penalty` *p* just before the glue (`\vskip`) that has been inserted
     between lines *j* and (*j* + 1).
   * If the *j*th line contains `\vadjust{`\<vertical mode material\>`}`, add
     \<vertical mode material\> converted to vertical list to *V* immediately
     after the *j*th line. Similarly for `\mark`, `\insert`, `\openout`,
     `\closeout`, and `\write`.
1. **[Finalize]**
   * Set `\looseness=0`.
   * Set `\parshape=0`, `\hangindent=0pt`, and `\hangafter=1`.

### Line Breaking Parameters Summary

* `\hsize=<dimen>` specifies the line width in horizontal mode
* `\leftskip=<glue>` specifies glue at left of justified lines
* `\rightskip=<glue>` specifies glue at right of justified lines
* `\hangindent=<dimen>` specifies the paragraph's hanging indentation
* `\hangafter=<number>` specifies the duration of `\hangindent`
* `\parshape=<n> <s1> <w1> <s2> <w2> ... <sn> <wn>` defines a shape of a
  paragraph
  * `<n>` (`<number>`) is the number (positive integer) of affected lines of
    the paragraph
  * `<si>` (`<dimen>`) is the indentation of the `i`th line of the paragraph
  * `<wi>` (`<dimen>`) is the width of the `i`th line of the paragraph

  `\parshape=0` cancels the previous `\parshape`; when read, `\parshape`
  returns the number of lines controlled
* `\parindent=<dimen>` specifies the width of `\indent`
* `\parfillskip=<glue>` specifies the additional `\rightskip` at end of
  paragraphs
* `\font` contains parameters that specify the inter-word space
* `\pretolerance=<number>` specifies the badness tolerance before hyphenation
* `\tolerance=<number>` specifies the badness tolerance after hyphenation
* `\emergencystretch=<glue>` reduces badnesses on third pass of line-breaking
* `\linepenalty=<number>` specifies the amount added to badness of every line
  in a paragraph
* `\patterns{<patterns>}` makes the *pattern dictionary*
* `\hyphenation{<words>}` adds `<words>` to the *exception dictionary*
* `\language=<number>` specifies the current set of hyphenation rules
* `\hyphenpenalty=<number>` specifies the penalty for line break after
  discretionary hyphen
* `\exhyphenpenalty=<number>` specifies the penalty for line break after
  explicit hyphen
* `\hfuzz=<dimen>` specifies the maximum overrun before `Overfull \hbox`
  message occur
* `\hbadness=<number>` specifies the badness above which bad `\hbox`es will be
  shown
* `\adjdemerits=<number>` specifies demerits for adjacent incompatible lines
* `\doublehyphendemerits=<number>` specifies demerits for consecutive broken
  lines
* `\finalhyphendemerits=<number>` specifies demerits for a penultimate broken
  line

### Making Pages

The *main vertical list* contains elements from which pages are build. The main
vertical list is populated in vertical mode. Only these types of items can
occur in a vertical list:
* a box or a rule
* a *whatsit*
* a mark
* an insertion
* a glue item or `\leaders`
* a kern
* a penalty

The last three types are *discardable*, the rest is *nondiscardable*.

In a vertical list, *break point*, or *page break*, can occur in these places:
* at glue which is immediately preceded by a nondiscardable item (penalty: 0)
* at a kern which is immediately followed by glue (penalty: 0)
* at a penalty (penalty: explicitly given)

A penalty greater or equal to 10000 forbids a break, a penalty less or equal to
-10000 forces a break.

Recall the *badness*(*page*) function:
* Let *he* be the expected height of *page* (`\pagegoal`) and *hn* be the
  natural height of *page* (`\pagetotal`).
* Set *s* to *he* - *hn*.
* Let *total_stretch*[*i*] be the sum of all stretch values of order *i* of all
  glues inside *page*. Similarly for *total_shrink*[*i*]. Recall that order *i*
  is:
  * 0 if the value has a unit convertible to `pt`;
    * *total_stretch*[0] is referred as `\pagestretch`;
    * *total_shrink*[0] is referred as `\pageshrink`;
  * 1 if the value has the `fil` unit;
    * *total_stretch*[1] is referred as `\pagefilstretch`;
  * 2 if the value has the `fill` unit;
    * *total_stretch*[2] is referred as `\pagefillstretch`;
  * 3 if the value has the `filll` unit;
    * *total_stretch*[3] is referred as `\pagefilllstretch`.
* Define *rank*(*x*) to be
  * 3 if *x*[3] is not 0;
  * otherwise, 2 if *x*[2] is not 0;
  * otherwise, 1 if *x*[1] is not 0;
  * otherwise, 0 if *x*[0] is not 0;
  * otherwise, -1.
* If *s* = 0, *badness*(*page*) is 0.
* If *s* < 0 (shrinking), *badness*(*page*) is
  * error if *rank*(*total_shrink*) > 0;
  * otherwise, infinity if *rank*(*total_shrink*) < 0 or
    *abs*(*s*) > *total_shrink*[0];
  * otherwise, *min*(100|*s*/*total_shrink*[0]|^3, 10000).
* If *s* > 0 (stretching), *badness*(*page*) is
  * 0 if *rank*(*total_stretch*) > 0;
  * otherwise, 10000 if *total_stretch*[0] <= 0;
  * otherwise, *min*(100|*s*/*total_stretch*[0]|^3, 10000).

Analogously, recall the *vbadness*(*bp*) function:
* Let *ch* be the current height and *h* be the expected height at *bp*,
  excluding *bp*.
* Set *s* to *h* - *ch*.
* Let *total_stretch*[*i*] be the sum of all stretch values of order *i* of all
  glues up to *bp*, exclusive. Similarly for *total_shrink*[*i*]. Recall that
  order *i* is:
  * 0 if the value has a unit convertible to `pt`;
  * 1 if the value has the `fil` unit;
  * 2 if the value has the `fill` unit;
  * 3 if the value has the `filll` unit.
* Define *rank*(*x*) to be
  * 3 if *x*[3] is not 0;
  * otherwise, 2 if *x*[2] is not 0;
  * otherwise, 1 if *x*[1] is not 0;
  * otherwise, 0 if *x*[0] is not 0;
  * otherwise, -1.
* If *s* = 0, *vbadness*(*bp*) is 0.
* If *s* < 0 (shrinking), *vbadness*(*bp*) is
  * error if *rank*(*total_shrink*) > 0;
  * otherwise, infinity if *rank*(*total_shrink*) < 0 or
    *abs*(*s*) > *total_shrink*[0];
  * otherwise, *min*(100|*s*/*total_shrink*[0]|^3, 10000).
* If *s* > 0 (stretching), *vbadness*(*bp*) is
  * 0 if *rank*(*total_stretch*) > 0;
  * otherwise, 10000 if *total_stretch*[0] <= 0;
  * otherwise, *min*(100|*s*/*total_stretch*[0]|^3, 10000).

Define *cost*(*pb*) function as follows:
* Let *b* be the badness at the potential page break *pb*.
* Let *p* be the penalty associated with *pb*.
* Set *q* to `\insertpenalties`.
* The value of *cost*(*pb*) is
  * *p* if *b* < infinity and *p* <= -10000 and *q* < 10000;
  * *b* + *p* + *q* if *b* < 10000 and -10000 < *p* < 10000 and *q* < 10000;
  * 100000 if *b* = 10000 and -10000 < *p* < 10000 and *q* < 10000;
  * infinity if (*b* = infinity or *q* >= 10000) and *p* < 10000.

Analogously, define *vcost*(*bp*) function as follows:
* Let *b* be the *vbadness*(*bp*) and *p* be the penalty associated with the
  potential breakpoint *bp*.
* The value of *vcost*(*bp*) is
  * *p* if *b* < infinity and *p* <= -10000;
  * *b* + *p* if *b* < 10000 and -10000 < *p* < 10000;
  * 100000 if *b* = 10000 and -10000 < *p* < 10000;
  * infinity if *b* = infinity and *p* < 10000.

The main vertical list exists in two parts:
1. *recent contributions*, containing items that will be moved to the current
   page when the time comes up
1. the *current page*, containing items that are candidates for the next page

Insertion items are put to a vertical or horizontal list using `\insert` *n*
`{` \<vertical mode material\> `}`. *n* is a \<number\> denoting an *insertion
class*. There are 255 classes of insertions, 0 to 254. Each insertion class is
tied with the four registers of the same number:
* `\box` *n* is where the material appears when a page is shipped;
* `\count` *n* is the magnification factor for page breaking (1000 times the
  factor by which the natural height plus depth of `\insert` *n* affects the
  page goal);
* `\dimen` *n* is the maximum insertion size per page;
* `\skip` *n* is the extra space to allocate on a page.

Insertion classes do not collide with each other. The order of insertions is
preserved within a class.

`\mark {` \<balanced text\> `}` puts a mark item to the list that is just
building.
* \<balanced text\> is expanded before the mark item is put into the list.
* Mark in the vertical mode goes to the main vertical list.
* Mark in the horizontal mode goes to the main vertical list too. It is placed
  right behind the box with a line where it appears.
* Mark in a restricted horizontal mode may migrate to the enclosing vertical
  list (like `\insert` or `\vadjust`). This happens when a `\hbox` enters a
  vertical list: a vertical material from the horizontal list is appended to
  the vertical list right after the `\hbox`. A vertical material also migrates
  from displayed equations.
* Mark that is buried to deeply in a box or mark in internal mode stays in a
  box where it appears.

Recall the functions used in the page building algorithm below:
* *height*(*x*) returns the height of box or rule *x*
* *depth*(*x*) returns the depth of box or rule *x*
* *base*(*x*) returns the base dimension of glue *x* or the amount of kern *x*

Define an auxiliary function *vert_break*(*V*, *h*, *d*):
1. Let *V* is a vertical list, *h* is the expected height, and *d* is the
   maximal depth.
   * Set *best_vcost* to infinity.
   * Set *best_vbreak* to null.
   * Set *best_vsize* to 0.
   * Set *ch* and *cd* to 0.
   * Set all components of *total_stretch* and *total_shrink* to 0.
1. Temporarily, for the scope of this function, append `\penalty -10000` to
   *V*.
1. For *I* in *V*:
   * If *I* is a box or a rule:
     * Add (*cd* + *height*(*I*)) to *ch*.
     * Set *cd* to *depth*(*I*).
     * If *cd* > *d*, add (*cd* - *d*) to *ch* and set *cd* to *d*.
   * If *I* is a legitimate breakpoint:
     * Set *c* to *vcost*(*I*).
     * If *c* <= *best_vcost*, set *best_vcost* to *c*, *best_vbreak* to *I*,
       and *best_vsize* to (*ch* + *cd*).
     * If *c* is infinity or the penalty associated with *I* is less or equal
       to -10000, return (*best_vbreak*, *best_vsize*).
   * If *I* is glue or a kern:
     * Add (*cd* + *base*(*I*)) to *ch*.
     * Add stretch and shrink values from *I*, if any, to *total_stretch* and
       *total_shrink*.
     * Set *cd* to 0.

The page building algorithm works as follows:
1. Express the main vertical list, *V*, as *P C*, where *C* is the list of
  recent contributions and *P* is the current page.
  * If *P* is not empty (continuing in page building after new items arrive in
    *C*), go to **Emptiness Test**.
1. **[Start New Page]** *P* is empty, prepare to populate it:
   * Set *best_cost* to infinity.
   * Set *best_break* to null.
   * Set *best_size* to 0.
   * Set `\pagegoal` to `\vsize`.
   * Set `\pagetotal` and `\pagedepth` to 0.
   * Set `\pagestretch`, `\pagefilstretch`, `\pagefillstretch`,
     `\pagefilllstretch`, and `\pageshrink` to 0.
   * Set `\lastskip`, `\lastkern`, and `\lastpenalty` to 0.
   * Set `\insertpenalties` to 0.
   * Set all components of *page_ins_height* to 0.
1. **[Emptiness Test]** If *C* is empty, terminate.
1. Move the first (top) item from *C* and append it to *P*.
   * If *C* does not contain any box or rule so far and the item is
     discardable, discard the item and go to **Emptiness Test**.
   * Call the item *I*.
1. Update `\lastskip`, `\lastkern`, and `\lastpenalty` with respect to *I*.
1. If *I* is a box or a rule:
   * If *I* is the first box or rule that comes to *P* and the `\topskip` space
     has not been made yet:
     * Move *I* back to *C*.
     * If `\topskip` > *height*(*I*):
       * Insert `\vskip` (`\topskip` - *height*(*I*)) right before *I*.
     * Otherwise, insert `\vskip 0pt plus \topskip stretch minus \topskip
       shrink` right before *I*.
     * Go to **Emptiness Test**.
   * Add (`\pagedepth` + *height*(*I*)) to `\pagetotal`.
   * Set `\pagedepth` to *depth*(*I*).
   * If `\pagedepth` > `\maxdepth`:
     * Add (`\pagedepth` - `\maxdepth`) to `\pagetotal`.
     * Set `\pagedepth` to `\maxdepth`.
1. If *I* is insertion:
   * Let *f* be (`\count` *n* / 1000).
   * Let *h* be (*height*(`\box` *n*) + *depth*(`\box` *n*)).
   * If there is no previous `\insert` *n* in *P*:
     * Let *w* be *base*(`\skip` *n*).
     * Set *page_ins_height*[*n*] to *h*.
     * Decrease `\pagegoal` by (*f h* + *w*).
     * Add stretch and shrink values from `\skip` *n* to `\pagestretch`,
       `\pagefilstretch`, `\pagefillstretch`, `\pagefilllstretch`, and
       `\pageshrink`.
   * If the previous `\insert` *n* in *P* has been split:
     * Add `\floatingpenalty` associated with *I* to `\insertpenalties`.
     * Skip the remaining steps.
   * If (*h* + *page_ins_height*[*n*] <= `\dimen` *n*) and (*f h* <= 0 or
     `\pagetotal` + `\pagedepth` + *f h* - `\pageshrink` <= `\pagegoal`):
     * *I* will fit to *P* without splitting.
     * Decrease `\pagegoal` by *f h*.
     * Add *h* to *page_ins_height*[*n*].
     * Skip the remaining steps.
   * If *f* <= 0, set *v* to (`\dimen` *n* - *page_ins_height*[*n*]).
     Otherwise, set *v* to 1/*f* (`\pagegoal` - `\pagetotal` - `\pagedepth`).
   * If *v* + *page_ins_height*[*n*] > `\dimen` *n*, set *v* to (`\dimen` *n* -
     *page_ins_height*[*n*]).
   * Let *d* be `\splitmaxdepth` associated with `\insert` *n* right after the
     main processor has processed `\insert` *n* `{...}` command.
   * Let *U* be the vertical list of `\insert` *n*.
   * Set (*best_vbreak*, *best_vsize*) to *vert_break*(*U*, *v*, *d*).
   * Add *best_vsize* to *page_ins_height*[*n*].
   * Decrease `\pagegoal` by (*f* times *best_vsize*).
   * Add the penalty associated with *best_vbreak* to `\insertpenalties`.
1. If *I* is a legitimate breakpoint:
   * Set *c* to *cost*(*I*).
   * If *c* <= *best_cost*, set *best_cost* to *c*, *best_break* to *I*, and
     *best_size* to `\pagegoal`.
   * If *c* is infinity or the penalty associated with *I* is less or equal to
     -10000, go to **Ship the Page**.
1. If *I* is glue or a kern:
   * Add (`\pagedepth` + *base*(*I*)) to `\pagetotal`.
   * Add stretch and shrink values from *I*, if any, to `\pagestretch`,
     `\pagefilstretch`, `\pagefillstretch`, `\pagefilllstretch`, and
     `\pageshrink`.
   * Set `\pagedepth` to 0.
1. Go to **Emptiness Test**.
1. **[Ship the Page]** Express *P* as *Q best_break R*.
   * If *best_break* is a `\penalty` *p*:
     * Set `\outputpenalty` to *p*.
     * Set *best_break* to `\penalty 10000`.
   * Otherwise, set `\outputpenalty` to 10000.
   * If `\botmark` is not empty:
     * Set `\topmark` to `\botmark`.
     * Clear `\firstmark`.
   * Fail if `\box 255` is not void.
   * Let *hold_ins_list* be the list of inserts to be held until the next page,
     at this point empty.
   * If `\holdinginserts` <= 0, then for every sequence *S* of `\insert` *n*
     items in *Q*:
     * Express *S* as *norm_ins split_ins wait_ins*, where *norm_ins* is a
       sequence of `\insert` *n* items that fit on a page without splitting,
       *split_ins* is an `\insert` *n* item that need to be split, and
       *wait_ins* is a sequence of `\insert` *n* items that do not fit on a
       page and will go back to recent contributions. Each of these three can
       be empty.
     * If *norm_ins* is not empty, append vertical material from inserts to
       `\box` *n*.
     * If *split_ins* is not empty:
       * Express its vertical material as *Y best_vbreak Z*.
       * Append *Y* to `\box` *n*.
       * Remove all discardable items from the top of *Z*.
       * Let `\splittopskip` has a value that `\splittopskip` have in time of
         completing `\insert` *n* `{` ... `}`, which is now referred as
         *split_ins*, by the main processor.
       * Add a space based on `\splittopskip` right before the first box or
         rule in *Z*.
       * If *Z* is not empty, make `\insert` *n* `{` *Z `}` and append it to
         *hold_ins_list*.
     * If *wait_ins* is not empty, append its inserts to *hold_ins_list*.
     * Recompute the `\box` *n* properties to reflect recent contributions, if
       any.
     * Remove *S* from *Q*.
   * Set `\insertpenalties` to the number of items in *hold_ins_list*.
   * For every mark node *M* in *Q*:
     * If `\firstmark` is empty, set `\firstmark` to *M*.
     * Set `\botmark` to *M*.
   * Set *C* to *best_break R C*. Empty *R*.
   * Pack *Q* into `\box 255` with the expected height set to *best_size*,
     maximal depth set to `\maxdepth`, `\vbadness` set to 10000, and `\vfuzz`
     set to maximal dimension. Empty *Q*.
   * If `\firstmark` is empty and `\topmark` is not empty, set `\firstmark` to
     `\topmark`.
   * If `\output` is not empty:
     * Fail if `\deadcycles` (no `\shipout` during `\output`) >=
       `\maxdeadcycles`.
     * Increase `\deadcycles` by one (reset back to zero during `\shipout`).
     * Open a group.
     * Set `\prevdepth` to -1000pt.
     * Set `\looseness` to 0, `\hangindent` to 0, `\hangafter` to 1, and
       `\parshape` to 0.
     * Put the content of `\output` to the token stream.
     * Enter the internal vertical mode.
     * Process token stream until the matching end of group is hit. This may
       produce a list of vertical mode contributions, let call them *T*.
     * Leave the group and the internal vertical mode.
     * Fail if `\box 255` is not void.
     * Append *T* to *hold_ins_list*.
   * Set *C* to *hold_ins_list C*.
   * Empty *hold_ins_list*.
   * If `\output` is empty, `\shipout\box255`.
   * Go to **Start New Page**.

Notes to the page building algorithm:
* Changing the `\vsize` and `\maxdepth` takes effect on the next page, not on
  the current one.
* If the first item on the page is `\insert` followed by a box or a rule, the
  space according to `\topskip` become a legitimate breakpoint.
* Reading the value of `\pagegoal` when the current page is empty returns the
  maximal dimension (16383.99998pt). This is handled by scanning routines, not
  by algorithm itself.
* The summary of when an output routine can be invoked:
  * At the beginning or end of a paragraph that is being contributed to the
    main vertical list.
  * At the beginning or end of a displayed equation that is a part of such a
    paragraph.
  * After an `\halign` is completed in vertical mode.
  * After a box, a rule, a penalty or an insertion is contributed to the main
    vertical list.
  * After an output routine has ended.
* Before the first page, `\topmark`, `\botmark`, and `\firstmark` are empty.

Summary of the page building algorithm parameters:
* `\voffset` is a vertical offset in `\shipout`
* `\hoffset` is a horizontal offset in `\shipout`
* `\vsize` is a page height in vertical mode
* `\maxdepth` is the maximum depth of boxes on main pages
* `\topskip` is glue at top of main pages
* `\floatingpenalty` is a penalty for insertions that are split
* `\splitmaxdepth` is the maximum depth of boxes on split pages
* `\holdinginserts` is positive if insertions are not discarded from the output
  box
* `\splittopskip` is glue at top of split pages
* `\output` is the user's output routine
* `\maxdeadcycles` is the upper bound on `\deadcycles`

Summary of the page building algorithm registers:
* `\pagegoal` is the desired height of the current page
* `\pagetotal` is the recent height of the current page
* `\pagedepth` is the depth of a box with a page material
* `\pagestretch` is the sum of all `\vskip` stretches on the current page
* `\pagefilstretch` is the sum of all `\vskip plus fil`s on the current page
* `\pagefillstretch` is the sum of all `\vskip plus fill`s on the current page
* `\pagefilllstretch` is the sum of all `\vskip plus filll`s on the current
  page
* `\pageshrink` is the sum of all `\vskip` shrinks on the current page
* `\lastskip` is the last glue on the current page if the last item is glue,
  otherwise it is zero
* `\lastkern` is the last kern on the current page if the last item is kern,
  otherwise it is zero
* `\lastpenalty` is the last penalty on the current page if the last item is
  penalty, otherwise it is zero
* `\insertpenalties` is the sum of all penalties for split insertions on the
  page or the number of held insertions when processing an `\output` routine
* `\outputpenalty` is a penalty at the current page break
* `\firstmark` expands to the first `\mark` on the current page
* `\topmark` expands to the last `\mark` from the previous page
* `\botmark` expands to the last `\mark` on the current page
* `\deadcycles` holds the number of output routines that ship no pages to DVI

#### `\shipout`

`\shipout <box>` sends the contents of `<box>` to the DVI file. In particular:
* When called for the first time, setup the DVI file header with the value of
  `\mag` recorded.
* Record `\count 0` to `\count 9` to the DVI file.
* Perform all `\openout`, `\closeout`, and `\write` commands in their order of
  appearance in `<box>`. At this point `\write` expands its *\<balanced text\>*
  parameter. If `\openout`, `\closeout`, or `\write` are in a box that is a
  part of `\leaders`, nothing happens.
* Convert the contents of `<box>` to the DVI commands and write them into the
  DVI file.
* Reset `\deadcycles` to 0.

The page is placed to (`\hoffset`, `\voffset`) relatively to the position of
DVI reference point which itself has coordinates (1in, 1in) from (left, top) of
paper margins/edges.

#### `\vsplit`

`\vsplit <number> to <dimen>` splits off a vertical material of natural height
`<dimen>` from a `\vbox <number>`. In a greater detail:
* Clear `\splitfirstmark` and `\splitbotmark`.
* If `\box <number>` is void, return the void box.
* Let *V* be the vertical list of `\vbox <number>`.
* Find a *best_break* using *vert_break*(*V*, `<dimen>`, `\splitmaxdepth`).
* Let *V* = *S best_break R*.
* If *S* is not empty, then for every mark *M* in *S*:
  * If `\splitfirstmark` is empty, set both `\splitfirstmark` and
    `\splitbotmark` to *M*.
  * Otherwise, set `\splitbotmark` to *M*.
* Remove all discardable items from the top of *R*.
* Add a space based on `\splittopskip` right before the first box or rule in
  *R*.
* If *R* is empty, make `\vbox <number>` void. Otherwise, make `\vbox <number>`
  a `\vbox` with *R* as its content.
* If *S* is empty, return the void box.
* Otherwise, pack *S* into a `\vbox` with height `<dimen>` and maximal depth
  `\splitmaxdepth`.
* Return the box made in the previous step.

Summary of `\vsplit` registers and parameters:
* `\splittopskip` is glue at top of split pages
* `\splitmaxdepth` is the maximum depth of boxes on split pages
* `\splitfirstmark` expands the first `\mark` in the `\vsplit`ted box
* `\splitbotmark` expands the last `\mark` in the `\vsplit`ted box

### `\special`

* `\special{landscape}` tells `dvips` to rotate page about 90 degrees

### Main Processor's Registers and Primitives

#### Alignment

* `\cleaders` works like *centered* `\leaders`
* `\cr` ends row or column
* `\crcr` works like `\cr`, but if the last command was `\cr`, `\crcr`, or
  `\noalign` then this command has no effect
* `\halign` makes a table
* `\leaders` makes a space repeatedly filled with a rule or a box
* `\noalign` specifies what to insert between columns or rows
* `\omit` ignores a template
* `\span` spans columns in a row
* `\valign` makes a transposed table
* `\xleaders` works like `\cleaders` but the left and right filler is divided
  between all boxes

#### Arithmetic

* `\advance` performs `x += y`
* `\divide` performs `x /= y`
* `\multiply` performs `x *= y`

#### Boxes

* `\box` inserts the box to the contributions list and clears the box register
* `\copy` works like `\box` but without clearing the register
* `\lastbox` removes the last box (does not work if the box is in the page
  list)
* `\setbox` assigns a box to the box register
* `\unhbox` unpacks a `\hbox` and clears the box register
* `\unhcopy` works like `\unhbox` but without clearing the register
* `\unvbox` unpacks a `\vbox` and clears the box register
* `\unvcopy` works like `\unvbox` but without clearing the register

#### Contributions

* `\kern` makes a solid space
* `\lastkern` gets the last `\kern`
* `\lastpenalty` gets the last `\penalty`
* `\lastskip` gets the last glue
* `\penalty` inserts a penalty
* `\unkern` removes the last `\kern` if present
* `\unpenalty` removes the last `\penalty` if present
* `\unskip` removes the last glue if present

#### Debugging

* `\message` writes an expanded text to the log file and terminal window
* `\show` shows the meaning of the following token
* `\showbox` shows the box's content
* `\showlists` shows lists with page material
* `\showthe` shows the token

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
* `\outer` means that macro cannot be used inside an other macro
* `\read` defines a macro with a content of the entire file as its body
* `\skipdef` gives a control sequence a meaning of `\skipXX`
* `\toksdef` gives a control sequence a meaning of `\toksXX`
* `\xdef` defines a macro globally, expands tokens inside its body

#### Error Management

* `\batchmode` tells TeX to skip all errors and to not display error messages
  on terminal
* `\errmessage` issues user-defined error
* `\errorstopmode` tells TeX to stop on error and ask a user for what to do
  next
* `\nonstopmode` tells TeX to not stop on any error
* `\scrollmode` tells TeX to scroll error messages on the terminal

#### File Operations

* `\closein` closes the input file
* `\closeout` closes the output file
* `\immediate` states that file operations are performed immediately
* `\openin` opens a file for input
* `\openout` opens a file for output
* `\write` writes a fully expanded text to a given file
  * unless `\immediate` prefix is used, the expansion is deferred until
    `\shipout`
  * if the file number is outside the range 0 to 15, including 15, the text is
    written to the log file
  * if the file number is greater than 15, the text is written also to terminal

#### Floats

* `\insert` makes a float

#### Fonts

* `\nullfont` is a void font without any characters and with seven parameters
  set to zero

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
* `\hskip` makes a horizontal space (glue)
* `\hss` is a shortcut for `\hskip 0pt plus 1fil minus 1fil`
* `\hyphenation` defines a user-defined hyphenation of words
* `\indent` starts the horizontal mode and inserts an empty box about
  `\parindent` width
* `\lower` lowers the box
* `\mark` puts a mark to the contributions list
* `\noboundary` suppresses implicit ligatures and `\kern`s
* `\noindent` starts the horizontal mode (paragraph) without indentation
* `\par` finishes the paragraph
* `\parshape` sets the shape of a paragraph
* `\patterns` defines a hyphenation table (IniTeX only)
* `\raise` raises the box
* `\setlanguage` inserts a mark according it the hyphenation table is chosen

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
* `\mskip` makes a glue in the math list
* `\nolimits` sets the *nolimits* flag to Op atom
* `\nonscript` tells TeX that the following space can be used only in
  non-script styles
* `\overline` makes an Over atom
* `\radical` makes a radical
* `\right` makes a right parentheses
* `\underline` makes an Under atom
* `\vcenter` makes a Vcent atom

##### Fonts

* `\displaystyle` sets display (D) style
* `\scriptscriptstyle` sets *script script* (SS) style
* `\scriptstyle` sets script (S) style
* `\textstyle` sets text (T) style

##### Fractions

* `\above` makes a fraction line, accepts thickness
* `\abovewithdelims` works like `\above` plus adds parentheses to sides
* `\over` makes a fraction
* `\overwithdelims` works like `\over` plus adds parentheses to sides

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
    * otherwise insert `\hbox to \hsize{}`, `\vfill`, and `\penalty-2^30` to
      the main vertical list, which invokes the page building algorithm, and
      then `\end` is read again
* `\global` states that the following action (assignment, definition) will be
  done at the global level
* `\ignorespaces` tells to the main processor to ignore all spaces until
  non-space command occurs
* `\lowercase` translates characters according to `\lccode`, category remains
  unchanged
* `\relax` performs no action
* `\uppercase` translates characters according to `\uccode`, category remains
  unchanged

#### Page

* `\shipout` ships the box to DVI
* `\special` inserts a whatsit node into DVI

#### Parameters and Registers

##### Alignment

* `\everycr` is a list of tokens inserted to the token list after `\cr` or
  `\crcr` that ends a line
* `\tabskip` is a space (glue) between columns or rows

##### Auxiliary Storage

* `\count` gives the access to a register for an integer storage
* `\dimen` gives the access to a register for a dimension storage
* `\muskip` gives the access to a register for a math glue storage
* `\skip` gives the access to a register for a glue storage
* `\toks` gives the access to a register for a token sequence storage

##### Boxes

* `\badness` keeps a badness of the last completed box
* `\boxmaxdepth` keeps the maximal allowed depth of a box
* `\dp` sets/gets a box depth
* `\ht` sets/gets a box height
* `\wd` sets/gets a box width

##### Codes

* `\catcode` sets/gets the category code of the given character
* `\delcode` tells to TeX how a character should be treated if it appears in
  math formula as a delimiter (parentheses)
* `\lccode` defines translation mapping for `\lowercase`
* `\mathcode` associates a math code with a character, i.e. it tells TeX how to
  translate `\char` to `\mathchar`
* `\sfcode` associates a space factor with a character
* `\uccode` defines translation mapping for `\uppercase`

##### Debugging

* `\pausing` tells TeX to stop on every input line
* `\showboxbreadth` determines maximum items per level when showing a box
* `\showboxdepth` determines maximum levels when showing a box
* `\tracingcommands` enables tracing of what primitives do
* `\tracinglostchars` enable tracing of characters that were not found in the
  current font
* `\tracingmacros` enables tracing macro expansions
* `\tracingonline` enables sending tracing messages also to terminal
* `\tracingoutput` enables tracing boxes that were sent to DVI
* `\tracingpages` enables tracing of how pages are assembled
* `\tracingparagraphs` enables tracing of how paragraphs are assembled
* `\tracingrestores` enables tracing of what is restored after leaving a group
* `\tracingstats` enables reporting of overall TeX statistics (memory used,
  fonts loaded, etc.)

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
* `\scriptfont` sets the font for the script style
* `\scriptscriptfont` sets the font for the *script script* style
* `\skewchar` sets/gets the skew character of the font
* `\textfont` sets the font for the text style

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
* `\overfullrule` is the width of a rule used to mark overfull `\hbox`es
* `\pretolerance` is a badness tolerance before hyphenation
* `\righthyphenmin` is a minimal number of characters in the right-hand side of
  the split word
* `\tolerance` is a badness tolerance after hyphenation
* `\uchyph` enables hyphenation of words starting with an upper-case letter
  * upper-case letters are those with `\lccode` different from zero and ASCII
    value of the character

##### Horizontal Spaces

* `\hangindent` is an indentation of lines given by `\hangafter`
* `\leftskip` is a left space (glue) before each line of a paragraph
* `\parfillskip` is a space (glue) inserted right to the last line of a
  paragraph
* `\parindent` is an indentation added at the beginning of a paragraph
* `\rightskip` is a right space (glue) after each line of a paragraph
* `\spacefactor` is a ratio for spaces multiplied by 1000
* `\spaceskip` is a space (glue) between words, if non-zero
* `\xspaceskip` is a space (glue) between sentences, if non-zero

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
* `\relpenalty` is the amount of penalty inserted after every Rel atom

##### Math Spaces

* `\delimitershortfall` is a maximum space not covered by a delimiter
* `\displayindent` is an indentation for lines in math displays
* `\displaywidth` is the length of line in math displays
* `\mathsurround` is the size of a solid space around `$...$`
* `\medmuskip` is the size of the medium math space (glue)
* `\nulldelimiterspace` is the width of a null delimiter
* `\predisplaysize` is the length of text preceding a math display
* `\scriptspace` is the space after superscript or subscript
* `\thickmuskip` is the size of the thick math space (glue)
* `\thinmuskip` is the size of the thin math space (glue)

##### Miscellaneous

* `\everyjob` is a list of tokens inserted to the token list when TeX starts

##### Page

* `\deadcycles` holds the number of output routines that ship no pages to DVI
* `\hoffset` is a horizontal offset in `\shipout`
* `\hsize` determines the width of the box with a page material
* `\mag` is a document magnification (1000 means 1); must be set before the
  first `\shipout`
* `\maxdeadcycles` is a maximal number of *dead cycles* (a call of the output
  routine where no page was shipped to DVI)
* `\maxdepth` is the maximal depth of the page box
* `\output` is a list of tokens representing output routine
* `\outputpenalty` is a penalty at the current page break
* `\pagedepth` is the depth of a box with a page material
* `\pagefilllstretch` is the sum of all `\vskip plus filll`s on the current
  page
* `\pagefillstretch` is the sum of all `\vskip plus fill`s on the current page
* `\pagefilstretch` is the sum of all `\vskip plus fil`s on the current page
* `\pagegoal` is the desired height of the current page
* `\pageshrink` is the sum of all `\vskip` shrinks on the current page
* `\pagestretch` is the sum of all `\vskip` stretches on the current page
* `\pagetotal` is the recent height of the current page
* `\voffset` is a vertical offset in `\shipout`
* `\vsize` determines the height of the box with a page material

##### Time

* `\day` keeps a day of a month
* `\month` keeps a month number
* `\time` keeps the number of minutes since midnight
* `\year` keeps the year

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
* `\postdisplaypenalty` is a penalty inserted after a math display
* `\predisplaypenalty` is a penalty inserted before a math display
* `\prevdepth` is the depth of the previously added box
* `\prevgraf` is a number of paragraph lines added so far
* `\splitmaxdepth` is the maximal depth of the `\vsplit`ted box
* `\vbadness` is a maximal vertical badness
* `\vfuzz` is a maximum `\vbox` overrun
* `\widowpenalty` is the amount of penalty added after the line before the last
  line of a paragraph

##### Vertical Spaces

* `\abovedisplayshortskip` is the space (glue) between a text and the top of
  the math display if the last line of text do not collide with the equation
* `\abovedisplayskip` is the space (glue) between a text and the top of the
  math display
* `\baselineskip` is the desired space (glue) between baselines
* `\belowdisplayshortskip` is the space (glue) between a text and the bottom of
  the math display if the last line of text do not collide with the equation
* `\belowdisplayskip` is the space (glue) between a text and the bottom of the
  math display
* `\lineskip` is the space (glue) between two lines if `\baselineskip` cannot
  be used
* `\lineskiplimit` is used to choose between `\lineskip` and `\baselineskip`
* `\parskip` is the space (glue) at the top of a paragraph
* `\splittopskip` is like `\topskip` but for `\vsplit`ted boxes
* `\topskip` is the space (glue) at the top of the current page

#### Vertical Contributions

* `\moveleft` moves a box to the left
* `\moveright` moves a box to the right
* `\vadjust` inserts its contributions after paragraph has been completed
* `\vbox` makes a vertical box
* `\vfil` is a shortcut for `\vskip 0pt plus 1fil`
* `\vfill` is a shortcut for `\vskip 0pt plus 1fill`
* `\vfilneg` is a shortcut for `\vskip 0pt plus -1fil`
* `\vrule` makes a vertical rule
* `\vskip` makes a vertical space (glue)
* `\vsplit` splits a vertical box
* `\vss` is a shortcut for `\vskip 0pt plus 1fil minus 1fil`
* `\vtop` makes a vertical box with the reference point on the baseline of the
  first line

## Terminology and Syntax Rules

In the following, `at`, `scaled`, `bp`, `cc`, `cm`, `dd`, `in`, `mm`, `pc`,
`pt`, `sp`, `em`, `ex`, `by`, `depth`, `height`, `width`, `fil`, `fill`,
`filll`, `minus`, `plus`, `spread`, `to`, and `true` are considered *keywords*.
Keywords are case insensitive. In `fil`, `fill`, and `filll` are allowed spaces
between the `l`s.

* *assignment*
  * macro definition
  * assignment to a register or parameter
  * `\advance`, `\multiply`, `\divide` commands
  * `\catcode`, `\delcode`, etc. assignment
  * `\let` command
  * `\chardef`, `\mathchardef`, `\countdef`, etc. commands
  * `\font`-defined control sequence or `\nullfont`
  * `\textfont`, `\scriptfont`, etc. assignment
  * `\parshape` specification
  * `\read` command
  * `\setbox` command
  * `\font` definition
  * `\fontdimen`, `\hyphenchar`, and `\skewchar` assignments
  * `\hyphenation` and `\patterns` commands
  * `\wd`, `\ht`, and `\dp` assignments
  * `\errorstopmode`, `\batchmode`, etc. commands
* *at clause*
  * `<space>*`
  * `<space>* at<dimen>`
  * `<space>* scaled<number>`
* *balanced text*
  * tokens between a token with category 1 and a token with category 2,
    excluding these tokens, where every token with category 1 must have a
    matching token with category 2
* *box*
  * is a result of these commands, including their arguments: `\box`, `\copy`,
    `\vsplit`, `\hbox`, `\vbox`, `\vtop`, and `\lastbox`
* *box or rule*
  * `<box>`
  * `\hrule <rule specification>`
  * `\vrule <rule specification>`
* *box specification*
  * `<space>* to<dimen><filler>`
  * `<space>* spread<dimen><filler>`
  * `<filler>`
* *control sequence*
  * a token with the 13 category is a control sequence
  * if the token processor sees a category 0 character followed by category 11
    characters, the these category 11 characters form a control sequence 
  * if the expand processor sees `\csname<something>\endcsname`,
    then `<something>` becomes a control sequence
* *delimiter*
  * `<filler>` ASCII character with non-negative `\delcode`
  * `<filler> \delimiter<27-bit number>`
* *dimen*
  * `<sign><float><unit>`
  * `<sign>` `\dimen` register
  * `<sign>` `\skip` register
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
* *general text*
  * `<filler> ({, 1) <balanced text> (}, 2)`
* *glue*
  * `<dimen><stretch>?<shrink>?`
  * `<sign>` `\skip` register
* *glue specification*
  * `\vskip <glue>`, `\vfil`, `\vfill`, `\vss`, `\vfilneg`
  * `\hskip <glue>`, `\hfil`, `\hfill`, `\hss`, `\hfilneg`
  * `\mskip <muglue>`
* *horizontal mode material*
  * a sequence of commands, together with their parameters, allowed in
    a horizontal mode
* *math field*
  * `<filler> <math-symbol>`
  * `<filler> ({, 1) <math mode material> (}, 2)`
* *math mode material*
  * a sequence of commands, together with their parameters, allowed in math or
    display mode
* *mudimen*
  * `<sign><float><mu-unit>`
  * `<sign>` `\muskip` register
* *muglue*
  * `<mudimen><mu-stretch>?<mu-shrink>?`
  * `<sign>` `\muskip` register
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
* *optional by*
  * `<space>*` `( by )?`
* *rule specification*
  * `( <width> | <height> | <depth> )* <space>*`
* *vertical mode material*
  * a sequence of commands, together with their parameters, allowed in a
    vertical mode
* `<depth>` is defined as:
  ```
  <depth> ::= <space>* depth <dimen>
  ```
* `<digit>` is defined as:
  ```
  <digit> ::= <odigit> | (8, 12) | (9, 12)
  ```
* `<fill-unit>` is defined as:
  ```
  <fill-unit> ::= <space>* fil <space>* ( l <space>* ( l <space>* )? )?
  ```
* `<float>` is defined as:
  ```
  <float> ::= <digit>+
  <float> ::= (', 12) <odigit>+
  <float> ::= (", 12) <xdigit>+
  <float> ::= (`, 12) <single-char-token>
  <float> ::= \count register
  <float> ::= \chardef constant
  <float> ::= \mathchardef constant
  <float> ::= <digit>+ <float-point> <digit>*
  <float> ::= <float-point> <digit>+
  ```
* `<float-point>` is defined as:
  ```
  <float-point> ::= (., 12) | (,, 12)
  ```
* `<generic-dimen>` is defined as:
  ```
  <generic-dimen> ::= <dimen>
  <generic-dimen> ::= <sign> <float> <fill-unit>
  ```
* `<generic-mu-dimen>` is defined as:
  ```
  <generic-mu-dimen> ::= <mu-dimen>
  <generic-mu-dimen> ::= <sign> <float> <fill-unit>
  ```
* `<height>` is defined as:
  ```
  <height> ::= <space>* height <dimen>
  ```
* `<math-symbol>` is defined as:
  ```
  <math-symbol> ::= a token of the category 11 or 12
  <math-symbol> ::= \char<8-bit number>
  <math-symbol> ::= \chardef constant
  <math-symbol> ::= \mathchar<15-bit number>
  <math-symbol> ::= \mathchardef constant
  <math-symbol> ::= \delimiter<27-bit number>
  ```
* `<mu-shrink>` is defined as:
  ```
  <mu-shrink> ::= <space>* ( minus <generic-mu-dimen> )?
  ```
* `<mu-stretch>` is defined as:
  ```
  <mu-stretch> ::= <space>* ( plus <generic-mu-dimen> )?
  ```
* `<mu-unit>` is defined as:
  ```
  <mu-unit> ::= <space>* mu <space>?
  <mu-unit> ::= <space>* \muskip register
  ```
* `<odigit>` is defined as:
  ```
  <odigit> ::= (0, 12) | (1, 12) | (2, 12) | (3, 12)
            |  (4, 12) | (5, 12) | (6, 12) | (7, 12)
  ```
* `<shrink>` is defined as:
  ```
  <shrink> ::= <space>* ( minus <generic-dimen> )?
  ```
* `<sign>` is defined as:
  ```
  <sign> ::= [ <space>, (+, 12), (-, 12) ]*
  ```
* `<single-char-token>` is defined as:
  ```
  <single-char-token> ::= single character control sequence
                       |  token (ASCII value, category)
                       |  active character
  ```
* `<space>` is defined as:
  ```
  <space> ::= token of category 10 or its equivalent control sequence/active
              character
  ```
* `<stretch>` is defined as:
  ```
  <stretch> ::= <space>* ( plus <generic-dimen> )?
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
* `<width>` is defined as:
  ```
  <width> ::= <space>* width <dimen>
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
