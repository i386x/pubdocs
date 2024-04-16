# Fedora Package Maintenance Quick Guide

## Contents

## Packaging

### Doing Ordinary Builds and Updates

### Doing a Rebase

1. Fetch the latest `rawhide`:
   ```bash
   $ git fetch
   $ git checkout rawhide
   $ git pull
   ```
1. Update the spec file
1. Do a scratch built against `rawhide`
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
1. Push the changes to `rawhide`:
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
   1. Build these consumers in the side tag as described one step ago
   1. Repeat the whole process until all consumers are processed
1. Create an update in [Bodhi](https://bodhi.fedoraproject.org/):
   1. Login
   1. Click `New Update`
   1. Add builds to `Builds` from the side tag (use `Use Side Tag` select)
   1. Fill `Description`
   1. Add bugs to `Bugs`
   1. Click `Submit`
1. After all builds successfully land to `rawhide`, cleanup the side tag:
   ```bash
   $ fedpkg remove-side-tag <side-tag>
   ```

### Unretiring a Package

1. Clone the package anonymously:
   ```bash
   $ fedpkg clone -a <the-package-name>
   ```
1. Enter the package directory and checkout the commit before the
   `dead.package` commit:
   ```bash
   $ cd <the-package-directory>
   $ git checkout <commit-id>
   ```
1. Make an adjustments that need to be done to make a successful build in
   `rawhide`:
   ```bash
   $ git branch unretire-pkg
   $ git checkout unretire-pkg
   $ fedpkg --release rawhide srpm
   $ fedpkg --release rawhide scratch-build --srpm <the-source-rpm-produced-by-the-previous-command>
   $ # Analyze scratch build logs, make adjustments:
   $ vim <spec-and-other-files>
   $ # Repeat scratch build:
   $ fedpkg --release rawhide srpm
   $ fedpkg --release rawhide scratch-build --srpm <the-source-rpm-produced-by-the-previous-command>
   $ # Repeat the last three steps (commands) until you are satisfied with the
   $ # result, then commit your changes:
   $ git add <changed-files>
   $ git commit -m 'Unretiring the package'
   ```
1. File a review request ticket in [Bugzilla](https://bugzilla.redhat.com/bugzilla/enter_bug.cgi?product=Fedora&format=fedora-review):
   1. Explain why you are opening the ticket:
      ```
      Hello, please review `<the-package-name>` which I am going to unretire
      because of <reason>. The spec file and the source RPM needed for the
      review are attached to this bugzilla. In the case of interest, I am also
      providing the link to the last successful scratch build: <link-to-the-scratch-build>

      Thanks in advance.
      ```
   1. Attach the spec file needed for the review:
      ```
      Attachment description:
        Spec file needed for the review
      ```
   1. Attach the source RPM needed for the review:
      ```
      Attachment description:
        Source RPM needed for the review
      ```
   1. Add `Unretirement` to the *Whiteboard* field
1. Announce on Fedora devel list that you are going to unretire the package:
   ```
   Title: Unretiring <the-package-name>
   Body:
     Hello,

     I am going to unretire <the-package-name> because of <reason>. The review
     request ticket is <link-to-the-review-request-ticket>

     Regards
     <your-name>
   ```
1. Watch the review request ticket:
   1. Fix all addressed issues
   1. Re-upload the updated spec file and source RPM
   1. If the *Whiteboard* field contains *NotReady*, clear it
1. Once the `fedora-review` flag is set to `+`, the package has passed the
   review
1. File a ticket with [release engineering](https://pagure.io/releng/issues):
   ```
   Title: Unretiring rpms/<the-package-name>
   Body:
     Hello,

     I would like to unretire `rpms/<the-package-name>` since <reason>.

     The approved BZ with review: <link-to-BZ>.

     The branches that need to be unretired/unblocked: rawhide, <all-active-fedora-and-epel-branches>

     Thanks in advance.
   ```
1. Once the ticket above has been resolved and closed, go to
   `https://src.fedoraproject.org/rpms/<the-package-name>` and check that you
   are the owner (main admin) of the package
1. Change `origin` of the recently cloned package repository to the SSH URL
   (you can find it under the `Clone` menu in the upper right corner of the
   page):
   ```bash
   $ git remote set-url origin <ssh-repository-url>
   ```
1. Switch to `rawhide` and fetch the latest changes (usually the `dead.package`
   commit has been reverted):
   ```bash
   $ git checkout rawhide
   $ git fetch
   $ git pull
   ```
1. If the `unretire-pkg` branch is in agreement with the `rawhide` branch you
   can go directly to the building the package part, including deleting the
   `unretire-pkg` branch
1. Rebase the changes you made to make the package to pass the review on the
   top of the `rawhide` branch. Resolve possible conflicts:
   ```bash
   $ git checkout unretire-pkg
   $ git rebase rawhide
   ```
1. Update the commit message of the changes if necessary:
   ```bash
   $ git commit --amend
   ```
   * Commit message in case the `dead.package` commit was not reversed:
     ```
     Unretiring the package

     Unretiring the package since <reason>.

     Release engineering ticket: <link-to-the-releng-ticket>

     Related: #<bz-number-of-the-review-request-ticket>
     ```
   * Commit message in case the additional changes were needed:
     ```
     Fix flaws preventing the build

     Fix several issues that prevent the package to be successfully build.
     In particular:
     * <the description of the issue #1>
     * <the description of the issue #2>
     * ...
     * <the description of the issue #n>

     Related: #<bz-number-of-the-review-request-ticket>
     ```
1. Before merging `unretire-pkg` to `rawhide`, check if it builds successfully:
   ```bash
   $ fedpkg --release rawhide srpm
   $ fedpkg --release rawhide scratch-build --srpm <path-to-the-recently-created-source-rpm>
   ```
1. Merge `unretire-pkg` into `rawhide`, delete the `unretire-pkg` branch:
   ```bash
   $ git checkout rawhide
   $ git merge --ff-only unretire-pkg
   $ git branch -D unretire-pkg
   ```
1. Push your changes and build the package:
   ```bash
   $ git push -u origin rawhide
   $ git fetch
   $ fedpkg build
   ```
1. After the successful build lands in the Fedora rawhide, close the review
   request ticket with the `NEXTRELEASE` resolution.

## Tooling

### Build History

```bash
$ koji list-history --build <NVR>
```

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
