# C Project

* C project (program or library) is consisting of a set of translation units
* translation units are kept in a source files (one unit per file)
* source file can `#include` header files
* source files and header files have usually `.c` and `.h` suffixes,
  respectively
* each source file is first [preprocessed](prep.md) and then compiled
* compiled files can be further passed to linker which produces executable,
  static, or shared library from them
  * when an executable is run, the control is passed to the function `main`,
    which serves as the entry point of the C program

## Translation Unit

A grammar for translation unit is:
```abnf
translation-unit = 1*external-declaration

external-declaration = function-definition
external-declaration =/ declaration
```

* for `function-definition`, see [Functions](func.md)
* for `declaration`, see [Declarations](decl.md)
