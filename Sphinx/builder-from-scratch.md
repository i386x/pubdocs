# How to Write Sphinx Builder from Scratch

In this document, we take a look on how to build our first HTML [builder](http://www.sphinx-doc.org/en/master/extdev/builderapi.html)
for [Sphinx](http://www.sphinx-doc.org/en/master/) from scratch.

In what follows, we refer to Sphinx 1.7.4 (changeset [f7b3292](https://github.com/sphinx-doc/sphinx/tree/f7b3292d87e9a2b7eae0b4ef72e87779beefc699)).

## Table of Contents

## What Happens When You Type `sphinx-build`?

When you type

```
sphinx-build -b html -d build/doctrees . build/html
```

Sphinx runs

```python
sphinx.cmdline.main(['-b', 'html', '-d', 'build/doctrees', '.', 'build/html'])
```

which performs the following steps:

1. gather and verify arguments
1. patch docutils
1. create instance of [Sphinx](http://www.sphinx-doc.org/en/master/extdev/appapi.html)
   application class
1. invoke the `build` method
1. return the exit code

The most important steps for us are creating a `Sphinx` instance, in which the
initialization is done, and the `build` method invokation.

We first look on Sphinx initialization.

### What happens during Sphinx initialization?

When `Sphinx` constructor is reached, the following members are created (we
take a look at the important ones):

**TODO:** Add links to data types.

| Member                  | Type                               | Description                                                                         |
| ----------------------- | -----------------------------------| ----------------------------------------------------------------------------------- |
| `extensions`            | `dict(str, Extension)`             | maps extension name to `Extension` object                                           |
| `_setting_up_extension` | `list(str)`                        | stack of currently processed (loaded) extensions; `?` means no extension is loading |
| `builder`               | `Builder`                          | this is where the all fun starts                                                    |
| `env`                   | `BuildEnvironment`                 | keeps all necessary information while building                                      |
| `registry`              | `SphinxComponentRegistry`          | keeps all registered components                                                     |
| `enumerable_nodes`      | `dict(Node, tuple(str, callable))` | TODO                                                                                |
| `html_themes`           | `dict(str, str)`                   | maps name to theme code                                                             |
| `srcdir`                | `str`                              | directory containing sources                                                        |
| `confdir`               | `str`                              | directory containing `conf.py`                                                      |
| `outdir`                | `str`                              | directory to write output                                                           |
| `doctreedir`            | `str`                              | cache directory for parsed source files                                             |
| `parallel`              | `int`                              | number of processes for parallel builds                                             |
| `events`                | `EventManager`                     | responsible for the proper events delivery                                          |
| `tags`                  | `Tags`                             | maps a tag name to `True`                                                           |
| `config`                | `Config`                           | loaded `conf.py`                                                                    |
| `translator`            | `object`                           | translations provider                                                               |

All of these members can be accessed via `Sphinx` instance, which is propagated
to every part of Sphinx.

The initialization steps inside `Sphinx` constructor are performed as follows:

1. `registry` is created. `registry` works as a container for Sphinx 
   components; these components must be available before the `build` is
   invoked.
1. Event manager (`events`) is initialized. [TODO: list of/link to events]
1. `tags` and `config` are loaded. `config` is loaded using `execfile_`. [TODO: list of/link to config values; note SOURCE_DATE_EPOCH]
1. Initialize `needs_sphinx`, `suppress_warnings`, `language`, and
   `locale_dirs` `config` values.
1. Initialize translations; load Sphinx locale dirs followed by user locale
   dirs. [TODO: link to i18n in Sphinx doc]
1. Load build-in and user extension modules. [TODO: link to extensions]
1. Preload builder. [TODO: link to builder]
1. Create output directory.
1. Run `config.setup`.
1. Initialize the rest of `config` values. [TODO: list of/link to config values]
1. Create builder. [TODO: link to builder]
1. Set up source parsers. [TODO: link to config/Parser]
1. Set up build environment. [TODO: link to more info]
1. Initialize builder:
   * set builder environment
   * call `builder.init`
   * emit `builder-inited`
1. Initialize enumerable nodes.

#### `Builder`

#### `BuildEnvironment`

#### `Config`

#### `EventManager`

#### `Extension`

#### `SphinxComponentRegistry`

**TODO:** Add links to data dypes.

| Member                | Type                                                    | Description                                                       |
| --------------------- | ------------------------------------------------------- | ----------------------------------------------------------------- |
| `autodoc_attrgettrs`  | `dict(type, callable(object * str * object -> object))` | TODO                                                              |
| `builders`            | `dict(str, type(Builder))`                              | maps builder name to its corresponding class                      |
| `documenters`         | `dict(str, type(Documenter))`                           | maps documenter name to its corresponding class                   |
| `domains`             | `dict(str, type(Domain))`                               | maps domain name to its corresponding class                       |
| `domain_directives`   | `dict(str, dict(str, object))`                          | maps domain name to the set of domain directives (TODO: describe) |
| `domain_indices`      | `dict(str, list(type(Index)))`                          | maps domain name to the list of indices classes                   |
| `domain_object_types` | `dict(str, dict(str, ObjType))`                         | maps domain name to the set of object types                       |
| `domain_roles`        | `dict(str, dict(str, RoleFunction + XRefRole))`         | maps domain name to the set of roles                              |
| `post_transforms`     | `list(type(Transform))`                                 | TODO                                                              |
| `source_parsers`      | `dict(str, Parser)`                                     | TODO                                                              |
| `source_inputs`       | `dict(str, Input)`                                      | TODO                                                              |
| `translators`         | `dict(str, NodeVisitor)`                                | TODO                                                              |
| `transforms`          | `list(type(Transform))`                                 | TODO                                                              |

#### `Tags`
