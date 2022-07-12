# Shell Hacks

A set of useful shell hacks gathered over the Internet.

## Git

A set of git hacks and aliases.

### Commit the contents of directory to any branch

by *Fraser Tweedale* ([github](https://github.com/frasertweedale/dotfiles/blob/494e7056d888ec2cd9ae1dd04ad52521c06d05fb/.gitconfig#L91),
[youtube](https://youtu.be/3MDsu6iFAD0?t=2370))

Copy this to your `.gitconfig`:
```ini
[alias]
snapshot = "!f() { \n\
  # decide index file before overriding work tree \n\
  export GIT_INDEX_FILE=$(git rev-parse --absolute-git-dir)/index-tmp \n\
  export GIT_WORK_TREE=$1 \n\
  REF=refs/heads/$2 \n\
  git read-tree \"$REF\" \n\
  git add --all --intent-to-add \n\
  git diff --quiet && exit \n\
  git add --all \n\
  TREE=$(git write-tree) \n\
  COMMIT=$(git commit-tree \"$TREE\" -p \"$REF\" -m \"snapshot $(date '+%y-%m-%d %H:%M')\") \n\
  git update-ref \"$REF\" \"$COMMIT\" \n\
}; f"
```

then use it as (no need to checkout or copy the repository)
```sh
$ git snapshot DIRECTORY TARGET-BRANCH
```

For example, to commit your static html documetation to github pages, type
```sh
$ git snapshot html gh-pages
```

## Hardware

Exploring hardware capabilities.

### CPU

* Print CPU HW capabilities:
  ```
  $ ld.so --help
  ```

### GPU

* Get details about graphics card:
  ```
  $ lspci -d ::0300 -v -nn
  ```
