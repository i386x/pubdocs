# My Collection of Public Documents

Dear Pilgrim, if You accidentally hit this place during Your journey through
The Internet, let me welcome You to my Kingdom of Links to the Stuff I am
interested in. Maybe You find some of these interesting too.

## Contents

1. [Debugging](#debugging)
1. [Fedora Project](#fedora-project)
   1. [GNOME](#gnome)
   1. [Packaging](#packaging)
1. [Linux](#linux)
   1. [Administration](#administration)
   1. [Error Codes](#error-codes)
   1. [Kernel](#kernel)
   1. [Networking](#networking)
   1. [Shared Libraries](#shared-libraries)
   1. [Shells](#shells)
   1. [Terminal](#terminal)
   1. [Utilities](#utilities)
1. [Opensource.com articles](#opensourcecom-articles)
1. [Programming](#programming)
   1. [Curiosities](#curiosities)
   1. [Documenting](#documenting)
   1. [Git](#git)
   1. [Programming Languages](#programming-languages)
1. [Security](#security)
1. [Z-Stuff](#z-stuff-unordered-and-uncategorized-stuff)

## Debugging

* [How to Debug C Program using gdb in 6 Simple Steps](https://www.thegeekstuff.com/2010/03/debug-c-program-using-gdb)
* [List of all function calls made in an application](https://stackoverflow.com/questions/9549693/list-of-all-function-calls-made-in-an-application)
* [StackTraces](https://fedoraproject.org/wiki/StackTraces)
* [Tracing program/function execution on source line level](https://stackoverflow.com/questions/39602306/tracing-program-function-execution-on-source-line-level)
* [Where is core dump located?](https://ask.fedoraproject.org/en/question/98776/where-is-core-dump-located/)

## Fedora Project

* [Legal:Export](https://fedoraproject.org/wiki/Legal:Export)
* [rpms](https://src.fedoraproject.org/projects/rpms/%2A)

### GNOME

* [How to install a GNOME Shell extension](https://fedoramagazine.org/install-gnome-shell-extension/)
* GNOME Shell Extensions
  * [TaskBar](https://extensions.gnome.org/extension/584/taskbar/)

### Packaging

* [Creating RPM Packages with Fedora](https://fedoraproject.org/wiki/How_to_create_an_RPM_package)
* [Fedora Packaging Guidelines](https://fedoraproject.org/wiki/Packaging:Guidelines)
* [Fedora Packaging Guidelines for RPM Scriptlets](https://fedoraproject.org/wiki/Packaging:Scriptlets)
* [How to create a GNU Hello RPM package](https://fedoraproject.org/wiki/How_to_create_a_GNU_Hello_RPM_package)
* [Infrastructure/WhatHappenedToPkgdb](https://fedoraproject.org/wiki/Infrastructure/WhatHappenedToPkgdb)
* [Join the package collection maintainers](https://fedoraproject.org/wiki/Join_the_package_collection_maintainers)
* [Licensing:Main](https://fedoraproject.org/wiki/Licensing:Main)
* [Licensing:MIT](https://fedoraproject.org/wiki/Licensing:MIT?rd=Licensing/MIT)
* [Package Review Process](https://fedoraproject.org/wiki/Package_Review_Process)
* [Staying close to upstream projects](https://fedoraproject.org/wiki/Staying_close_to_upstream_projects)
* [Using the Koji build system](https://fedoraproject.org/wiki/Using_the_Koji_build_system)

#### Go packaging

* [Go Packaging Guidelines Draft (packaging-committee)](https://pagure.io/packaging-committee/issue/382)
* [golang@lists.fedoraproject.org](https://lists.fedoraproject.org/archives/list/golang@lists.fedoraproject.org/)
* [More Go packaging](https://fedoraproject.org/wiki/More_Go_packaging)
* [More Go packaging (packaging-committee)](https://pagure.io/packaging-committee/issue/734)

#### RPM

* [Maximum RPM](http://ftp.rpm.org/max-rpm/)
* [Packaging:RPMMacros](https://fedoraproject.org/wiki/Packaging:RPMMacros)
* [RPM Guide](https://docs-old.fedoraproject.org/en-US/Fedora_Draft_Documentation/0.1/html/RPM_Guide/index.html)
* [RPM HOWTO](http://www.tldp.org/HOWTO/RPM-HOWTO/index.html)
* [RPM Packaging Guide](https://rpm-packaging-guide.github.io/)
* [RPM Package Manager](http://rpm.org/)
* `rpmlint`
  * [Common Rpmlint issues](https://fedoraproject.org/wiki/Common_Rpmlint_issues)
* [Rukověť baliče RPM (in Czech)](http://www.abclinuxu.cz/serialy/rukovet-balice-rpm)

#### Spec files examples

* [rpms/kernel](https://src.fedoraproject.org/rpms/kernel/blob/master/f/kernel.spec)

#### Testing

* [Changes/InvokingTests](https://fedoraproject.org/wiki/Changes/InvokingTests)
* [CI](https://fedoraproject.org/wiki/CI)
* [How to test updates](https://fedoraproject.org/wiki/How_to_test_updates)
* [QA:Updates Testing](https://fedoraproject.org/wiki/QA:Updates_Testing)
* [Taskotron](https://fedoraproject.org/wiki/Taskotron)
  * [pagure.io](https://pagure.io/group/taskotron)
* [Using Mock to test package builds](https://fedoraproject.org/wiki/Using_Mock_to_test_package_builds)

##### Beaker

* [Beaker Home Page](https://beaker-project.org/)
  * [Resources](https://beaker-project.org/docs/)
* [beakerlib (git)](https://github.com/beakerlib/beakerlib)

## Linux

* [Učebnice GNU/Linuxu (in Czech)](http://www.abclinuxu.cz/ucebnice/obsah)

### Administration

* [Configuring Sudo](https://fedoraproject.org/wiki/Configuring_Sudo)
* [Linux Users and Groups](https://www.linode.com/docs/tools-reference/linux-users-and-groups/)

### Error Codes

* [Common Error Codes](https://www.student.cs.uwaterloo.ca/~cs136/seashell/docs/seashell-error-codes.html)

### Kernel

* [linux-0.01](https://github.com/zavg/linux-0.01)

### Networking

* [13 Linux Network Configuration and Troubleshooting Commands](https://www.tecmint.com/linux-network-configuration-and-troubleshooting-commands/)

### Shared Libraries

* [ld.so, ld-linux.so - dynamic linker/loader](http://man7.org/linux/man-pages/man8/ld.so.8.html)
* [Program Library HOWTO](http://tldp.org/HOWTO/Program-Library-HOWTO/index.html)

#### FAQ

* [Can I change 'rpath' in an already compiled binary?](https://stackoverflow.com/questions/13769141/can-i-change-rpath-in-an-already-compiled-binary)
* [What is the order that Linux's dynamic linker searches paths in?](https://unix.stackexchange.com/questions/367600/what-is-the-order-that-linuxs-dynamic-linker-searches-paths-in)
* [Why I cannot override search path of dynamic libraries with LD_LIBRARY_PATH?](https://stackoverflow.com/questions/33519640/why-i-cannot-override-search-path-of-dynamic-libraries-with-ld-library-path)

### Shells

* Bourne Again Shell
  * [Bash Guide for Beginners](http://tldp.org/LDP/Bash-Beginners-Guide/html/index.html)
  * [Advanced Bash-Scripting Guide](http://tldp.org/LDP/abs/html/index.html)
  * [Writing your own Bash Completion Function](http://fahdshariff.blogspot.cz/2011/04/writing-your-own-bash-completion.html)
* [DASH Shell (git)](https://git.kernel.org/pub/scm/utils/dash/dash.git/)
* [FISH Shell](http://fishshell.com/)
  * [git](https://github.com/fish-shell/fish-shell)
  * issues
    * [Killing fish reparents some child processes into CPU hogs](https://github.com/fish-shell/fish-shell/issues/3644)
      * [another attempt to workaround a glibc bug](https://github.com/fish-shell/fish-shell/commit/56e05dab02840443bb29beb6a596e8e3f35c5461#diff-fc9a1225873620e0a6b45c16ed066f27)
* [POSIX](http://pubs.opengroup.org/onlinepubs/009695399/utilities/xcu_chap02.html)

### Terminal

* [Ctrl-s hang terminal emulator?](https://unix.stackexchange.com/questions/72086/ctrl-s-hang-terminal-emulator)
* [How to unfreeze after accidentally pressing Ctrl-S in a terminal?](https://unix.stackexchange.com/questions/12107/how-to-unfreeze-after-accidentally-pressing-ctrl-s-in-a-terminal)
* [ioctl_tty - ioctls for terminals and serial lines](http://man7.org/linux/man-pages/man2/ioctl_tty.2.html)
* [Linux Serial Console](https://github.com/torvalds/linux/blob/master/Documentation/admin-guide/serial-console.rst)
* [Look how to fix column calculation in Python readline if use color prompt](https://stackoverflow.com/questions/9468435/look-how-to-fix-column-calculation-in-python-readline-if-use-color-prompt)
* [Serial Programming/termios](https://en.wikibooks.org/wiki/Serial_Programming/termios)

### Utilities

* `make`
  * [POSIX](http://pubs.opengroup.org/onlinepubs/009695399/utilities/make.html)

## Opensource.com articles

## Programming

### Curiosities

* [Advent of Code](https://adventofcode.com/)

### Documenting

* [Markdown](https://daringfireball.net/projects/markdown/)
* [Mastering Markdown](https://guides.github.com/features/mastering-markdown/)
* Writing manual pages
  * [man page example](https://github.com/karelzak/util-linux/blob/master/Documentation/howto-man-page.txt)
  * [What you need to know to write man pages](https://www.linux.com/news/what-you-need-know-write-man-pages)

### Git

#### FAQ

* [fork forced sync](https://gist.github.com/glennblock/1974465)
* [git branch messed up](https://stackoverflow.com/questions/6984900/git-branch-messed-up)
* [How do I apply rejected hunks after fixing them?](https://stackoverflow.com/questions/17879746/how-do-i-apply-rejected-hunks-after-fixing-them)
* [How to clean up unused side-branches in your commit trees?](https://stackoverflow.com/questions/11756250/how-to-clean-up-unused-side-branches-in-your-commit-trees)
* [How to sync with a remote Git repository?](https://stackoverflow.com/questions/4313125/how-to-sync-with-a-remote-git-repository)
* [How to create patch between two tags with multiple commits between them?](https://stackoverflow.com/questions/9078820/how-to-create-patch-between-two-tags-with-multiple-commits-between-them)
* [Is there a way to squash a number of commits non-interactively?](https://stackoverflow.com/questions/7275508/is-there-a-way-to-squash-a-number-of-commits-non-interactively)
* [Move branch pointer to different commit without checkout](https://stackoverflow.com/questions/5471174/move-branch-pointer-to-different-commit-without-checkout)
* [Source Forge repo gives “denying non-fast-forward refs/heads/master” error](https://stackoverflow.com/questions/12450703/source-forge-repo-gives-denying-non-fast-forward-refs-heads-master-error)

### Programming Languages

#### C

* [SEI CERT C Coding Standard](https://wiki.sei.cmu.edu/confluence/display/c/SEI+CERT+C+Coding+Standard)

## Security

* [Project Zero](https://googleprojectzero.blogspot.cz/2018/01/reading-privileged-memory-with-side.html)

## Z-Stuff (unordered and uncategorized stuff)

* [Brno Observatory and Planetarium (mobile version, in Czech)](http://m.hvezdarna.cz/)
