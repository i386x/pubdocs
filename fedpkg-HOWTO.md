# Maintaining Packages in Fedora: Cheat Sheet

Assume you have FAS account, Bugzilla account, and sponsorship.

## Setup your environment

As a user, generate your Kerberos ticket:
```sh
KRB5_TRACE=/dev/stdout kinit your_login@FEDORAPROJECT.ORG
```

**Tip**: `klist -A` displays your tickets.

## Getting a package

All Fedora packages are at https://src.fedoraproject.org/ .

Login to https://src.fedoraproject.org/.

Fork the package's repo.

**Tip**: Prefer Firefox for communication with https://src.fedoraproject.org/.

Clone the package by running `fedpkg clone <package>`.

Change `origin` to be the forked repo:
```sh
git remote set-url origin <ssh_url_to_your_forked_repo>
# <ssh_url_to_your_forked_repo> is at the bottom of your forked repo's main
# page
```

**Tip**: `git remote -v` shows your tracked remote repos.

## Doing a changes

Clean all untracked files (if there are any):
```sh
fedpkg clean
```

Prepare the package:
```sh
# Run commands in %prep section; this downloads and unpacks source tarball from
# upstream and applies patches:
fedpkg prep
```

Make the contributions.

Edit the spec file:
```sh
# Bumps up the spec file version:
rpmdev-bumpspec <name>.spec
```

**Tip**: Use `git add` before `fedpkg clean` to start tracking untracked files.

Inform the upstream if you create a new patch.

Commit the changes:
```sh
# Affected branches: master, f27, f26
#
# Assume the recent branch is master.
# 1. Commit the changes to the master branch:
git add <new and modified files>
git commit -m "Commit message"
# If needed, edit the commit message:
git commit --amend
# or if you have more commits:
git rebase -i HEAD~<number of involved commits>
# 2. Switch to other branches and merge them with master:
git checkout <branch_name>
git merge --ff-only master
```

Build/test the package locally:
```sh
# Build the package as local for rawhide:
git checkout master
fedpkg --release master local
fedpkg clean
# Build the package as local for Fedora 27:
git checkout f27
fedpkg --release f27 local
fedpkg clean
# Build the package as local for Fedora 26:
git checkout f26
fedpkg --release f26 local
fedpkg clean
```

Build/test the package using `mock`:
```sh
# Mock build for rawhide:
git checkout master
fedpkg srpm
mock -r fedora-rawhide-x86_64 <path to srpm>
fedpkg clean
# Mock build for Fedora 27:
git checkout f27
fedpkg srpm
mock -r fedora-27-x86_64 <path to srpm>
fedpkg clean
# Mock build for Fedora 26:
git checkout f26
fedpkg srpm
mock -r fedora-26-x86_64 <path to srpm>
fedpkg clean
```

## Sending the changes

Do scratch builds:
```sh
# Do scratch build for all arches. If no srpm is provided, build from most
# recent pushed commit. You should switch to the corresponding branch (master,
# f27, f26 for rawhide, f27, f26, respectively):
git checkout [master|f27|f26]
fedpkg srpm
fedpkg scratch-build --target [rawhide|f27|f26] --srpm <path to srpm>
```

Push the changes (if scratch builds succeeds):
```sh
git checkout <branch_name>
git push
```

Do builds:
```sh
git checkout <branch_name>
fedpkg build
```

**Tip 1**: This takes long time. Sending `SIGINT` by `Ctrl-C` brings building
to background.

**Tip 2**: On `koji.fedoraproject.org`, after signing in and selecting
component, you can see the building progress and status.

## Update system

Go to Bugzilla and write a comment about resolved issue, provide link to the
corresponding commit.

Change the state to modified.

Sign in to `bodhi.fedoraproject.org`.

Choose `Create -> New Update`.

Select package, candidate builds, related bugs, final details (choose a proper
type), write update notes and submit.

## Useful utilities and tricks

### Some useful utilities

```sh
# ld-linux.so.2 dynamic loader:
dnf install /lib/ld-linux.so.2
```

### Git

```sh
# Removing ignored/untracked items:
git clean -xdf
```

### RPM

```sh
# List all the files stored in <package>.rpm:
rpm -qlp <package>.rpm
```

### Debugging

```sh
# GNU debugger:
gdb
# - load program:
gdb ./<elf_executable>
# - load program with arguments:
gdb --args ./<elf_executable> [arguments]
# - gdb commands:
#       `bt`   - print stack back trace
#       `quit` - exit from gdb

# Start tracing shared libraries:
LD_DEBUG
# - show how shared objects are searched, loaded, and initialized:
LD_DEBUG=libs ./<elf_binary> [arguments]
```

**Tip**: If something is missing (debug information, symbols, ...) `gdb`
provides you a hint or command how to install it.

### Shared libraries

```sh
# Involved files and directories:
/etc/ld.so.conf
/etc/ld.so.conf.d
/etc/ld.so.cache
/etc/ld.so.preload
/usr/lib
/usr/lib64

# Dynamic loaders/linkers:
/lib/ld.so
/lib/ld-linux.so.<N> # <N> is vesrion, i.e. 1, 2, ...
# - configuring:
ldconfig

# Library path:
LD_LIBRARY_PATH
# - run program, add the current directory to the library search path:
LD_LIBRARY_PATH=.:$LD_LIBRARY_PATH ./<elf_binary> [arguments]

# Library preloading:
LD_PRELOAD
# - run program with <lib> preloaded:
LD_PRELOAD=<lib> ./<elf_binary> [arguments]
```

See also:
- http://tldp.org/HOWTO/Program-Library-HOWTO/shared-libraries.html
- `man ld.so`

**Tip**: If no from the above works, the shared library is probably plugin.
Consult the documentation and/or source code of maintained package how to deal
with it.

### ELF/COFF/coredumps analysis

```sh
# Read ELF binary:
readelf
# - info about used shared libraries:
readelf -d <elf_binary>
ldd <elf_binary>
# - all information:
readelf -a <elf_binary>

# Patch ELF binary:
patchelf
# - set/add R(UN)PATH:
patchelf --set-rpath <rpath> <elf_binary>

# Scan ELF binary:
scanelf
# - find where the <symbol> is:
scanelf -l -s <symbol>

# RPATH manipulation:
chrpath
# - change RPATH in <elf_binary> to <new_rpath>:
chrpath -r <new_rpath> <elf_binary>

# See lot of info about ELF/COFF binary:
objdump
# - show all headers:
objdump -x <elf_binary>

# List the all coredumps of the program <prog>:
coredumpctl list <prog>
# Open the last coredump in gdb:
coredumpctl gdb
# Show information about a process that dumped core, matching <pid>:
coredumpctl info <pid>
# Extract the last core dump of /usr/bin/<prog> to a file named
# <prog>.coredump:
coredumpctl -o <prog>.coredump dump /usr/bin/<prog>
```

See also:
- https://ask.fedoraproject.org/en/question/98776/where-is-core-dump-located/

### Performance analysis

```sh
# Show system resources usage:
top
```

**Chromium tip**: `Shift + Esc` launches the task manager with PID and CPU
usage information per tab.

### Signals

```C
#define SIGHUP     1
#define SIGINT     2
#define SIGQUIT    3
#define SIGILL     4
#define SIGTRAP    5
#define SIGABRT    6
#define SIGIOT     6
#define SIGBUS     7
#define SIGFPE     8
#define SIGKILL    9
#define SIGUSR1   10
#define SIGSEGV   11
#define SIGUSR2   12
#define SIGPIPE   13
#define SIGALRM   14
#define SIGTERM   15
#define SIGSTKFLT 16
#define SIGCHLD   17
#define SIGCONT   18
#define SIGSTOP   19
#define SIGTSTP   20
#define SIGTTIN   21
#define SIGTTOU   22
#define SIGURG    23
#define SIGXCPU   24
#define SIGXFSZ   25
#define SIGVTALRM 26
#define SIGPROF   27
#define SIGWINCH  28
#define SIGIO     29
#define SIGPOLL   SIGIO
#define SIGLOST   29
#define SIGPWR    30
#define SIGSYS    31
#define SIGUNUSED 31
```

TTY driver settings:
```sh
# Show control character mapping:
stty -a
```

### Tricks

```sh
# View the content of binary file in hex+ASCII:
hexdump -Cv <file> | less

# Unpack tarball:
gzip -dc <tarball.tar.gz> | tar -xvvof -
bzip2 -dc <tarball.tar.bz2> | tar -xvvof -
```
