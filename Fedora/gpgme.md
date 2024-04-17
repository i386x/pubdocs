# `gpgme` Maintainer's Notes

## Dependencies on `qgpgme`

`qgpgme` is the `gpgme`'s Qt binding.

Packages depending on it:
```
qgpgme (gpgme):
    isoimagewriter
    kf5-libkleo
    kget
    trojita

kf5-libkleo:
    kf5-messagelib
    kleopatra
    kmail-account-wizard

kf5-messagelib:
    kf5-mailcommon

kf5-mailcommon:
    kdepim-addons
    kmail
```
