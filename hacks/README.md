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

### Get details about graphics card

```sh
$ lspci -d ::0300 -v -nn
```

### Print CPU HW capabilities

```sh
$ ld.so --help
```

## Network

### Probe Network Interfaces

* `ip link`
* `/sys/class/net`

```sh
$ nmcli --fields device,type d status
$ nmcli --fields device,type d status | sed -n '/ethernet/{s/\s.*//;p;q}'
```

## Video

A set of X/Wayland hacks.

### Prefer X11 over Wayland

```sh
$ XDG_SESSION_TYPE=X11 command
```

### Tips of XWayland with Sway

`.bashrc` from [Fedora Discussion](https://discussion.fedoraproject.org/t/tips-of-xwayland-with-sway/74757/11):
```sh
# Wayland
# https://wiki.archlinux.org/title/Wayland
# https://discussion.fedoraproject.org/t/tips-of-xwayland-with-sway/74757
if [ "${XDG_SESSION_TYPE}" = wayland ]; then
    export MOZ_ENABLE_WAYLAND=1
    # For Gimp
    export GDK_BACKEND=x11
    # For Java application
    export _JAVA_AWT_WM_NONREPARENTING=1
else
    unset MOZ_ENABLE_WAYLAND
    unset GDK_BACKEND
    unset _JAVA_AWT_WM_NONREPARENTING
fi
```
