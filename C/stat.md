# Statements

Grammar for statements:
```abnf
statement = labeled-statement
statement =/ expression-statement
statement =/ compound-statement
statement =/ selection-statement
statement =/ iteration-statement
statement =/ jump-statement
```

* except several cases, statements are executed in sequence
* statements do not have values

## Labeled statements

Grammar for labeled statements:
```abnf
labeled-statement = identifier ":" statement
labeled-statement =/ %x63.61.73.65 constant-expression ":" statement  ; case
labeled-statement =/ %x64.65.66.61.75.6C.74 ":" statement             ; default
```

* labels in themselves don't alter the control flow
* a label consisting of an identifier declares the identifier
  * labels have their own name space
  * the scope of the identifier is the current function
  * the only use of the identifier is as a target of `goto`
* `case` and `default` labels are used with the `switch` statement
  * the constant expression of `case` must have integral type

## Expression Statement

Grammar for an expression statement:
```abnf
expression-statement = [ expression ] ";"
```

* `;` construction is called a *null statement*
* all side effects from expression are completed before the execution of the
  next statement

## Compound Statement

Grammar for a compound statement:
```abnf
compound-statement = "{" [ declaration-list ] [ statement-list ] "}"

declaration-list = 1*declaration
statement-list = 1*statement
```

* also called *block*
* these rules apply to identifiers in the same name space (identifiers in
  different name space are treated distinctly):
  * if an identifier in the *declaration-list* was in scope outside the block,
    the outer declaration is suspended within the block, after which it is
    resumed
  * an identifier may be declared only once in the same block
* initialization of automatic objects is performed each time the block is
  entered at the top and proceeds in the order of the declarators
  * if a jump into the block is executed, these initializations are not
    performed
* initializations of `static` objects are performed only once, before the
  program begins execution

## Selection Statements

Grammar for selection statements:
```abnf
selection-statement = %x69.66 "(" expression ")" statement                           ; if
selection-statement =/ %x69.66 "(" expression ")" statement %x65.6C.73.65 statement  ; if else
selection-statement =/ %x73.77.69.74.63.68 "(" expression ")" statement              ; switch
```

* in `if (E) S1` and `if (E) S1 else S2`
  * `E` must have arithmetic type or be a pointer
  * `E` is evaluated including all side effects
  * if `E != 0`, `S1` is executed
  * if `E == 0`, `S2` is executed
  * `else` is connected with the last encountered `else`-less `if` at the same
    block nesting level
* in `switch (E) S`
  * `E` must have integral type
  * `S` is typically compound
  * any statement within `S` may be labeled with one or more `case` labels
  * `E` undergoes integral promotion and the `case` constants are converted to
    the promoted type
  * after conversion, no two of the `case` constants associated with the same
    `switch` may have the same value
  * at most one `default` label may be associated with a `switch`
  * a `case` or `default` label is associated with the smallest `switch` that
    contains it
* `switch (E) S` is executed as follows:
  1. `E` is evaluated, including all side effects
  1. the value of `E` is compared with each `case` constant
  1. if one of the `case` constants is equal to `E`, control passes to the
     statement of the matched `case` label
  1. if no `case` constant matches `E` and there is a `default` label, control
     passes to the labeled statement
  1. if no `case` matches and there is no `default`, `S` is not executed

## Iteration Statements

Grammar for iteration statements:
```abnf
iteration-statement = %x77.68.69.6C.65 "(" expression ")" statement                                       ; while
iteration-statement =/ %x64.6F statement %x77.68.69.6C.65 "(" expression ")" ";"                          ; do while
iteration-statement =/ %x66.6F.72 "(" [ expression ] ";" [ expression ] ";" [ expression ] ")" statement  ; for
```

* in `while (E) S` and `do S while (E);`
  * `E` must have arithmetic or pointer type
  * `S` is executed repeatedly so long as `E != 0`
  * with `while`, the test, including all side effects from `E`, occurs before
    each execution of `S`
  * with `do`, the test, including all side effects from `E`, follows each
    iteration
* in `for (E1; E2; E3) S`
  * any of `E1`, `E2`, and `E3` may be dropped
    * missing `E2` is equivalent replacing `E2` with a non-zero constant
  * side effects from each `E1`, `E2`, and `E3` are completed immediately after
    its evaluation
  * `E1` can have any type
    * it is evaluated once (initialization for the loop)
  * `E2` must have arithmetic or pointer type
    * it is evaluated before each iteration
    * if `E2 == 0`, the `for` is terminated
  * `E3` can have any type
    * it is evaluated after each iteration (re-initialization for the loop)
  * if `S` does not contain `continue`, the `for` statement is equivalent to
    ```C
    E1;
    while (E2) {
      S
      E3;
    }
    ```

## Jump Statements

Grammar for jump statements:
```abnf
jump-statement = %x67.6F.74.6F identifier ";"             ; goto
jump-statement =/ %x63.6F.6E.74.69.6E.75.65 ";"           ; continue
jump-statement =/ %x62.72.65.61.6B ";"                    ; break
jump-statement =/ %x72.65.74.75.72.6E [ expression ] ";"  ; return
```

* in `goto label;`
  * `label` must be a label located in the current function
  * control transfers to the labeled statement
* a `continue` statement
  * can be used only within an iteration statement
  * it causes control to pass to the loop continuation portion of the smallest
    enclosing iteration statement, i.e. within each of the
    ```C
    while (E) {        do {                for (E1; E2; E3) {
      S                  S                   S
    label: ;           label: ;            label: ;
    }                  } while (E);        }
    ```
    a `continue` not contained in a smaller iteration statement is the same as
    `goto label`
* a `break` statement
  * can be used only within an iteration statement or a `switch` statement
  * terminates execution of the smallest enclosing iteration or `switch`
    statement
    * control passes to the statement following the terminated statement
* a `return` statement
  * causes returning from a function to its caller
  * when `return E;` is used, the value of `E` is returned to the caller of the
    function
    * `E` is converted to the type returned by the enclosing function as if by
      assignment
  * flowing off the end of a function is equivalent to `return;`
  * in `return;`, the returned value is undefined
