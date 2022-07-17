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
