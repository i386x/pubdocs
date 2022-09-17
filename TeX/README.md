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
1. Vertical or internal vertical mode plus category 11 token, category 12
   token, `\char`, `\chardef` defined constant, `\hskip`, `\hfil`, `\hfill`,
   `\hss`, `\hfilneg`, `\unhbox`, `\unhcopy`, `\vrule`, `\valign`, `\accent`,
   `\discretionary`, `\-`, `\<space>`, `\noboundary`, `$`, `$$`, `\indent`, or
   `\noindent`:
   1. enter the horizontal mode
   1. if the current token is distinct from `\indent` and `\noindent`, return
      it back to the token list
   1. initialize an empty horizontal list for incoming material
   1. if the current token was distinct from `\noindent`, insert to the
      horizontal list an empty `\hbox` of the width `\parindent`
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

`\chardef\xyz=<number>` defines an 8-bit (0 to 255) numeric constant `\xyz`,
which serves as an equivalent for `\char<number>`.

`\mathchardef\uvw=<number>` defines a 15-bit (0 to 32767) numeric constant
`\uvw`, which serves as an equivalent for `\mathchar<number>`.

#### Data Types

A number register (`\count`) occupies 4 bytes. The range of stored number is
`-2**31 + 1` (-2147483647) to `2**31 - 1` (2147483647).

A dimension register (`\dimen`) occupies 4 bytes. Dimensions are stored
internally in `sp` units, the range of stored dimension is `(-2**30 + 1) sp`
(-1073741823 `sp`) to `(2**30 - 1) sp` (1073741823 `sp`). When assigning a
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
horizontal (`h`) or vertical (`v`) material, called *box*. Supported operations
with a box register are:
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
     * `X.stretch += Y'.stretch`
     * `X.shrink += Y'.shrink`

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
     // to be also a power of two, so we express K as 2**k. Now observe the
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
1. Using `\kern` or `\/` command.

*Skip* is a space that can shrunk or stretched. Skips usually allows line
breaks. Skip can be inserted to the horizontal list in these ways:
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

`\leaders` behaves like `\hskip` or `\vskip` &ndash; it is a skip which fills
its space by the given pattern. `\leaders` accepts these parameters:
* `\leaders` *\<box or rule\>* *\<glue specification\>*

If *\<box or rule\>* is a rule and when the final size of skip is established:
1. Establish the missing rule dimensions as described in the
   horizontal/vertical list-to-box conversion.
1. In the horizontal mode:
   * Set the rule width to the finally established skip size.
1. In the vertical mode:
   * Set the rule height to the finally established skip size.
   * Set the rule depth to 0pt.
1. If the finally established skip size is non-negative:
   * Draw the rule in the space occupied by the skip with respect to
     baseline/y-axis.

If *\<box or rule\>* is a box and when the final size of skip is established:
1. In case of `\leaders` in the horizontal mode:
   ```python
   # Snippet of Python-like pseudo-code that ships `\leaders` element `e` to
   # the DVI.

   # Get the current position:
   cx = get_current_position_x()
   # Get the final skip width:
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
   # Get the final skip height:
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

   # Get the final skip width:
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

   # Get the final skip height:
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

   # Get the final skip width:
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

   # Get the final skip height:
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
* `\lastskip` gets the last space
* `\penalty` inserts a penalty
* `\unkern` removes the last `\kern` if present
* `\unpenalty` removes the last `\penalty` if present
* `\unskip` removes the last space if present

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
* `\hskip` makes a horizontal space
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
* `\mskip` makes a space in the math list
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
    * otherwise insert an empty box of `\hsize` width, `\vfill`, and
      `\penalty-2^30` to the vertical list, which invokes the page completion
      algorithm, and then `\end` is read again
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
* `\tabskip` is a space between columns or rows

##### Auxiliary Storage

* `\count` gives the access to a register for an integer storage
* `\dimen` gives the access to a register for a dimension storage
* `\muskip` gives the access to a register for a math space storage
* `\skip` gives the access to a register for a space storage
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
* `\leftskip` is a left space before each line of a paragraph
* `\parfillskip` is a space inserted right to the last line of a paragraph
* `\parindent` is an indentation added at the beginning of a paragraph
* `\rightskip` is a right space after each line of a paragraph
* `\spacefactor` is a ratio for spaces multiplied by 1000
* `\spaceskip` is a space between words, if non-zero
* `\xspaceskip` is a space between sentences, if non-zero

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
* `\medmuskip` is the size of the medium math space
* `\nulldelimiterspace` is the width of a null delimiter
* `\predisplaysize` is the length of text preceding a math display
* `\scriptspace` is the space after superscript or subscript
* `\thickmuskip` is the size of the thick math space
* `\thinmuskip` is the size of the thin math space

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
* `\parskip` is the space at the top of a paragraph
* `\splittopskip` is like `\topskip` but for `\vsplit`ted boxes
* `\topskip` is the space at the top of the current page

#### Vertical Contributions

* `\moveleft` moves a box to the left
* `\moveright` moves a box to the right
* `\vadjust` inserts its contributions after paragraph has been completed
* `\vbox` makes a vertical box
* `\vfil` is a shortcut for `\vskip 0pt plus 1fil`
* `\vfill` is a shortcut for `\vskip 0pt plus 1fill`
* `\vfilneg` is a shortcut for `\vskip 0pt plus -1fil`
* `\vrule` makes a vertical rule
* `\vskip` makes a vertical space
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
  * ASCII character with non-negative `\delcode`
  * `\delimiter<27-bit number>`
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
* *glue*
  * `<dimen><stretch>?<shrink>?`
  * `<sign>` `\skip` register
* *glue specification*
  * `\vskip <glue>`, `\vfil`, `\vfill`, `\vss`, `\vfilneg`
  * `\hskip <glue>`, `\hfil`, `\hfill`, `\hss`, `\hfilneg`
  * `\mskip <muglue>`
* *mudimen*
  * `<sign><float><mu-unit>`
  * `<sign>?` `\muskip` register
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
  * `<space>*`
  * `( <width> | <height> | <depth> )+`
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
  ```
* `<space>` is defined as:
  ```
  <space> ::= token of category 10 or its equivalent control sequence
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
