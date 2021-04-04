# C Project

* C project (program or library) is consisting of a set of translation units
* translation units are kept in a source files (one unit per file)
* source file can `#include` header files
* source files and header files have usually `.c` and `.h` suffixes,
  respectively
* each source file is first [preprocessed](preprocessor.md) and then compiled
* compiled files can be further passed to linker which produces executable,
  static, or shared library from them
