# Fedora Package Maintenance Quick Guide

## Contents

## Packaging

### Doing Ordinary Builds and Updates

### Doing a Rebase

1. Fetch the latest rawhide:
   ```bash
   $ git fetch
   $ git checkout rawhide
   $ git pull
   ```
1. Update the spec file
1. Do a scratch built against rawhide
1. Investigate the content of produced RPMs
   * SONAME bumps
   * `rpminspect`
   * `abidiff`
1. Track down the consumers of bumped SONAMEs
   * Ask for the comaintainership
1. Determine the order of builds
1. Request a side tag:
   ```bash
   $ fedpkg request-side-tag
   ```
1. Push the changes to rawhide:
   ```bash
   $ git commit
   $ git push
   $ git fetch
   ```
1. Build:
   ```bash
   $ fedpkg build --target=<side-tag>
   $ koji wait-repo <side-tag>
   ```
1. For all direct consumers:
   ```bash
   $ cd <consumer-directory>
   $ git fetch
   $ git checkout rawhide
   $ git pull
   $ # Update the spec file (bump NVR or do other changes if necessary)
   $ # Do a scratch build against the side tag
   $ git commit
   $ git push
   $ git fetch
   $ fedpkg build --target=<side-tag>
   ```
1. If direct consumers have their own direct consumers:
   1. Wait for the repo:
      ```bash
      $ koji wait-repo <side-tag>
      ```
   1. Build these consumers in the side tag as described a minute ago
   1. Repeat the whole process until all consumers are processed
1. Create an update in [Bodhi](https://bodhi.fedoraproject.org/):
   1. Login
   1. Click `New Update`
   1. Add builds to `Builds` from the side tag (use `Use Side Tag` select)
   1. Fill `Description`
   1. Add bugs to `Bugs`
   1. Click `Submit`
1. After all builds successfully land to rawhide, cleanup the side tag:
   ```bash
   $ fedpkg remove-side-tag <side-tag>
   ```

### Unretiring a Package

1. File a ticket with [release engineering](https://pagure.io/releng/issues):
   ```
   Title: Unretiring rpms/<the-package-name>
   Body:
     I would like to unretire `rpms/<the-package-name>` since <reason>. The
     approved BZ with review: <link-to-BZ>. Thanks in advance.
   ```

## Tooling

### Dependency Tracking

* Fedora Rawhide
  ```bash
  $ # 32-bit library:
  $ (WHAT="libfoo.so.0"; MIRROR=http://ftp.fi.muni.cz/pub/linux/fedora/linux; \
  dnf --repofrompath=Fdr-rawhide,$MIRROR/development/rawhide/Everything/x86_64/os/ \
  --disablerepo='*' --enablerepo='Fdr-*' --refresh repoquery --whatdepends "$WHAT")

  $ # 64-bit library:
  $ (WHAT="libfoo.so.0()(64bit)"; MIRROR=http://ftp.fi.muni.cz/pub/linux/fedora/linux; \
  dnf --repofrompath=Fdr-rawhide,$MIRROR/development/rawhide/Everything/x86_64/os/ \
  --disablerepo='*' --enablerepo='Fdr-*' --refresh repoquery --whatdepends "$WHAT")
  ```
* Fedora Alpha/Beta
  ```bash
  $ # 32-bit library:
  $ (WHAT="libfoo.so.0"; VER=0; MIRROR=http://ftp.fi.muni.cz/pub/linux/fedora/linux; \
  dnf --repofrompath=Fdr-$VER,$MIRROR/development/$VER/Everything/x86_64/os/ \
      --repofrompath=Fdr-$VER-updates,$MIRROR/updates/$VER/Everything/x86_64/ \
  --disablerepo='*' --enablerepo='Fdr-*' --refresh repoquery --whatdepends "$WHAT")

  $ # 64-bit library:
  $ (WHAT="libfoo.so.0()(64bit)"; VER=0; MIRROR=http://ftp.fi.muni.cz/pub/linux/fedora/linux; \
  dnf --repofrompath=Fdr-$VER,$MIRROR/development/$VER/Everything/x86_64/os/ \
      --repofrompath=Fdr-$VER-updates,$MIRROR/updates/$VER/Everything/x86_64/ \
  --disablerepo='*' --enablerepo='Fdr-*' --refresh repoquery --whatdepends "$WHAT")
  ```
* Fedora Released
  ```bash
  $ # 32-bit library:
  $ (WHAT="libfoo.so.0"; VER=0; MIRROR=http://ftp.fi.muni.cz/pub/linux/fedora/linux; \
  dnf --repofrompath=Fdr-$VER,$MIRROR/releases/$VER/Everything/x86_64/os/ \
      --repofrompath=Fdr-$VER-updates,$MIRROR/updates/$VER/Everything/x86_64/ \
  --disablerepo='*' --enablerepo='Fdr-*' --refresh repoquery --whatdepends "$WHAT")

  $ # 64-bit library:
  $ (WHAT="libfoo.so.0()(64bit)"; VER=0; MIRROR=http://ftp.fi.muni.cz/pub/linux/fedora/linux; \
  dnf --repofrompath=Fdr-$VER,$MIRROR/releases/$VER/Everything/x86_64/os/ \
      --repofrompath=Fdr-$VER-updates,$MIRROR/updates/$VER/Everything/x86_64/ \
  --disablerepo='*' --enablerepo='Fdr-*' --refresh repoquery --whatdepends "$WHAT")
  ```
* Fedora ELN Brewroot
  ```bash
  $ # 32-bit library:
  $ (WHAT="libfoo.so.0"; VER=latest; MIRROR=https://kojipkgs.fedoraproject.org/repos/eln-build; \
  dnf --repofrompath=eln-build-$VER,$MIRROR/$VER/x86_64/ \
  --disablerepo='*' --enablerepo='eln-build-*' --refresh repoquery --whatdepends "$WHAT")

  $ # 64-bit library:
  $ (WHAT="libfoo.so.0()(64bit)"; VER=latest; MIRROR=https://kojipkgs.fedoraproject.org/repos/eln-build; \
  dnf --repofrompath=eln-build-$VER,$MIRROR/$VER/x86_64/ \
  --disablerepo='*' --enablerepo='eln-build-*' --refresh repoquery --whatdepends "$WHAT")
  ```

### Doing a Scratch Build

### `rpminspect`
