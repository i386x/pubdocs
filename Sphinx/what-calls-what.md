# What Calls What in Sphinx

Notes from the reading [Sphinx 1.7.4 source](https://github.com/sphinx-doc/sphinx/tree/f7b3292d87e9a2b7eae0b4ef72e87779beefc699).

## Contents

* [`sphinx.application`](#sphinxapplication)
* [`sphinx.builders`](#sphinxbuilders)
* [`sphinx.cmd.build`](#sphinxcmd-build)
* [`sphinx.cmdline`](#sphinxcmdline)
* [`sphinx.config`](#sphinxconfig)
* [`sphinx.domains`](#sphinxdomains)
* [`sphinx.environment`](#sphinxenvironment)
* [`sphinx.environment.adapters.toctree`](#sphinxenvironmentadapterstoctree)
* [`sphinx.errors`](#sphinxerrors)
* [`sphinx.extension`](#sphinxextension)
* [`sphinx.io`](#sphinxio)
* [`sphinx.make_mode`](#sphinxmake_mode)
* [`sphinx.registry`](#sphinxregistry)
* [`sphinx.transforms`](#sphinxtransforms)
* [`sphinx.util`](#sphinxutil)
* [`sphinx.util.console`](#sphinxutilconsole)
* [`sphinx.util.docutils`](#sphinxutildocutils)
* [`sphinx.util.i18n`](#sphinxutili18n)
* [`sphinx.util.logging`](#sphinxutillogging)
* [`sphinx.util.matching`](#sphinxutilmatching)
* [`sphinx.util.nodes`](#sphinxutilnodes)
* [`sphinx.util.osutil`](#sphinxutilosutil)
* [`sphinx.util.parallel`](#sphinxutilparallel)
* [`sphinx.util.pycompat`](#sphinxutilpycompat)
* [`sphinx.util.tags`](#sphinxutiltags)
* [`sphinx.versioning`](#sphinxversioning)

### `sphinx.application`

* [`sphinx.application.Sphinx.__init__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/application.py#L112)
  * [`sphinx.registry.SphinxComponentRegistry.__init__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/registry.py#L54)
  * [`sphinx.util.logging.setup`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/logging.py#L489)
  * [`sphinx.events.EventManager.__init__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/events.py#L48)
  * [`sphinx.util.console.bold`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/console.py#L102)
  * [`sphinx.util.tags.Tags.__init__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/tags.py#L50)
  * [`sphinx.config.Config.__init__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/config.py#L148)
  * [`sphinx.config.Config.check_unicode`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/config.py#L232)
  * [`sphinx.config.Config.pre_init_values`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/config.py#L268)
  * [`sphinx.application.Sphinx._init_i18n`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/application.py#L237)
  * [`sphinx.errors.VersionRequirementError`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/errors.py#L63)
  * [`sphinx.locale.__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/locale/__init__.py#L251)
  * [`sphinx.application.Sphinx.setup_extension`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/application.py#L407)
  * [`sphinx.application.Sphinx.preload_builder`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/application.py#L295)
  * [`sphinx.config.Config.__getattr__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/config.py#L308)
  * [`sphinx.util.osutil.ensuredir`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/osutil.py#L83)
  * [`sphinx.errors.ConfigError`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/errors.py#L55)
  * [`sphinx.config.Config.init_values`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/config.py#L283)
  * [`sphinx.extension.verify_required_extensions`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/extension.py#L44)
  * [`sphinx.registry.SphinxComponentRegistry.has_domain`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/registry.py#L108)
  * [`sphinx.application.Sphinx.create_builder`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/application.py#L299)
  * [`sphinx.config.Config.check_types`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/config.py#L190)
  * [`sphinx.application.Sphinx._init_source_parsers`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/application.py#L263)
  * [`sphinx.application.Sphinx._init_env`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/application.py#L271)
  * [`sphinx.application.Sphinx._init_builder`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/application.py#L307)
  * [`sphinx.application.Sphinx._init_enumerable_nodes`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/application.py#L313)

* [`sphinx.application.Sphinx._init_builder`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/application.py#L307)
  * [`sphinx.builders.Builder.set_environment`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L109)
  * [`sphinx.builders.Builder.init`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L149)
  * [`sphinx.application.Sphinx.emit`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/application.py#L436)
    - emmits `builder-inited`

* [`sphinx.application.Sphinx._init_enumerable_nodes`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/application.py#L313)
  * [`sphinx.environment.BuildEnvironment.get_domain`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L765)

* [`sphinx.application.Sphinx._init_env`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/application.py#L271)
  * [`sphinx.environment.BuildEnvironment.__init__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L168)
  * [`sphinx.environment.BuildEnvironment.find_files`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L405)
  * [`sphinx.registry.SphinxComponentRegistry.create_domains`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/registry.py#L112)
  * [`sphinx.locale.__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/locale/__init__.py#L251)
  * [`sphinx.util.console.bold`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/console.py#L102)
  * [`sphinx.environment.BuildEnvironment.frompickle`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L126)
  * [`sphinx.application.Sphinx._init_env`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/application.py#L271)

* [`sphinx.application.Sphinx._init_i18n`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/application.py#L237)
  * [`sphinx.config.Config.__getattr__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/config.py#L308)
  * [`sphinx.util.console.bold`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/console.py#L102)
  * [`sphinx.util.i18n.find_catalog_source_files`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/i18n.py#L103)
  * [`sphinx.util.i18n.CatalogInfo.write_mo`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/i18n.py#L65)
  * [`sphinx.locale.init`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/locale/__init__.py#L261)
  * [`sphinx.locale.__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/locale/__init__.py#L251)

* [`sphinx.application.Sphinx._init_source_parsers`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/application.py#L263)
  * [`sphinx.config.Config.__getattr__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/config.py#L308)
  * [`sphinx.application.Sphinx.add_source_parser`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/application.py#L668)
  * [`sphinx.registry.SphinxComponentRegistry.get_source_parsers`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/registry.py#L223)

* [`sphinx.application.Sphinx.add_source_parser`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/application.py#L668)
  * [`sphinx.registry.SphinxComponentRegistry.add_source_parser`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/registry.py#L200)

* [`sphinx.application.Sphinx.build`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/application.py#L320)
  * [`sphinx.builders.Builder.compile_all_catalogs`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L248)
  * [`sphinx.builders.Builder.build_all`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L292)
  * [`sphinx.builders.Builder.compile_specific_catalogs`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L259)
  * [`sphinx.builders.Builder.build_specific`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L297)
  * [`sphinx.builders.Builder.compile_update_catalogs`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L280)
  * [`sphinx.builders.Builder.build_update`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L328)
  * [`sphinx.locale.__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/locale/__init__.py#L251)
  * [`sphinx.util.console.bold`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/console.py#L102)
  * [`sphinx.util.osutil.relpath`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/osutil.py#L208)
  * [`sphinx.config.Config.__getattr__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/config.py#L308)
  * [`sphinx.application.Sphinx.emit`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/application.py#L436)
    - emits `build-finished`
  * [`sphinx.builders.Builder.cleanup`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L501)

* [`sphinx.application.Sphinx.create_builder`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/application.py#L299)
  * [`sphinx.locale.__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/locale/__init__.py#L251)
  * [`sphinx.registry.SphinxComponentRegistry.create_builder`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/registry.py#L94)

* [`sphinx.application.Sphinx.emit`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/application.py#L436)
  * [`sphinx.events.EventManager.emit`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/events.py#L75)

* [`sphinx.application.Sphinx.is_parallel_allowed`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/application.py#L683)
  * [`sphinx.locale.__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/locale/__init__.py#L251)

* [`sphinx.application.Sphinx.preload_builder`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/application.py#L295)
  * [`sphinx.registry.SphinxComponentRegistry.preload_builder`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/registry.py#L79)

* [`sphinx.application.Sphinx.setup_extension`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/application.py#L407)
  * [`sphinx.registry.SphinxComponentRegistry.load_extension`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/registry.py#L300)

### `sphinx.builders`

* [`sphinx.builders.Builder.__init__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L79)
  * [`sphinx.util.osutil.ensuredir`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/osutil.py#L83)

* [`sphinx.builders.Builder._write_parallel`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L447)
  * [`sphinx.builders.Builder.write_doc`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L481)
  * [`sphinx.environment.BuildEnvironment.get_and_resolve_doctree`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L803)
  * [`sphinx.builders.Builder.write_doc_serialized`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L486)
  * [`sphinx.util.parallel.ParallelTasks.__init__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/parallel.py#L62)
  * [`sphinx.util.parallel.make_chunks`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/parallel.py#L138)
  * [`sphinx.util.status_iterator`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/__init__.py#L589)
  * [`sphinx.util.parallel.ParallelTasks.add_task`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/parallel.py#L97)
  * [`sphinx.util.console.bold`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/console.py#L102)
  * [`sphinx.util.parallel.ParallelTasks.join`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/parallel.py#L110)

* [`sphinx.builders.Builder._write_serial`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L438)
  * [`sphinx.util.logging.pending_warnings`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/logging.py#L218)
  * [`sphinx.util.status_iterator`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/__init__.py#L589)
  * [`sphinx.environment.BuildEnvironment.get_and_resolve_doctree`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L803)
  * [`sphinx.builders.Builder.write_doc_serialized`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L486)
  * [`sphinx.builders.Builder.write_doc`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L481)

* [`sphinx.builders.Builder.build`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L340)
  - overriden by [`sphinx.builders.gettext.MessageCatalogBuilder.build`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/gettext.py#L254)
  * [`sphinx.util.console.bold`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/console.py#L102)
  * [`sphinx.util.logging.pending_warnings`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/logging.py#L218)
  * [`sphinx.environment.BuildEnvironment.update`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L493)
  * [`sphinx.environment.BuildEnvironment.check_dependents`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L617)
  * [`sphinx.environment.BuildEnvironment.topickle`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L161)
  * [`sphinx.environment.BuildEnvironment.check_consistency`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L911)
  * [`sphinx.application.Sphinx.is_parallel_allowed`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/application.py#L683)
  * [`sphinx.util.parallel.SerialTasks.__init__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/parallel.py#L41)
  * [`sphinx.builders.Builder.write`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L407)
  * [`sphinx.builders.Builder.finish`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L493)
  * [`sphinx.util.parallel.SerialTasks.join`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/parallel.py#L54)

* [`sphinx.builders.Builder.build_all`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L292)
  * [`sphinx.builders.Builder.build`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L340)

* [`sphinx.builders.Builder.build_specific`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L297)
  * [`sphinx.config.Config.__getattr__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/config.py#L308)
  * [`sphinx.builders.Builder.build`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L340)

* [`sphinx.builders.Builder.build_update`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L328)
  * [`sphinx.builders.Builder.get_outdated_docs`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L184)
  * [`sphinx.builders.Builder.build`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L340)

* [`sphinx.builders.Builder.cleanup`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L501)
  - overriden by [`sphinx.builders.html.StandaloneHTMLBuilder.cleanup`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/html.py#L831)

* [`sphinx.builders.Builder.compile_all_catalogs`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L248)
  * [`sphinx.config.Config.__getattr__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/config.py#L308)
  * [`sphinx.util.i18n.find_catalog_source_files`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/i18n.py#L103)
  * [`sphinx.builders.Builder.compile_catalogs`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L233)

* [`sphinx.builders.Builder.compile_catalogs`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L233)
  - overriden by [`sphinx.builders.gettext.I18nBuilder.compile_catalogs`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/gettext.py#L139)
  * [`sphinx.config.Config.__getattr__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/config.py#L308)
  * [`sphinx.util.osutil.relpath`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/osutil.py#L208)
  * [`sphinx.util.console.bold`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/console.py#L102)
  * [`sphinx.util.status_iterator`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/__init__.py#L589)
  * [`sphinx.util.i18n.CatalogInfo.write_mo`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/i18n.py#L65)

* [`sphinx.builders.Builder.compile_specific_catalogs`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L259)
  * [`sphinx.environment.BuildEnvironment.path2doc`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L342)
  * [`sphinx.util.i18n.find_catalog`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/i18n.py#L81)
  * [`sphinx.config.Config.__getattr__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/config.py#L308)
  * [`sphinx.util.i18n.find_catalog_source_files`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/i18n.py#L103)
  * [`sphinx.builders.Builder.compile_catalogs`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L233)

* [`sphinx.builders.Builder.compile_update_catalogs`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L280)
  * [`sphinx.util.i18n.find_catalog_source_files`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/i18n.py#L103)
  * [`sphinx.config.Config.__getattr__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/config.py#L308)
  * [`sphinx.builders.Builder.compile_catalogs`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L233)

* [`sphinx.builders.Builder.default_translator_class`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L64)
  - overriden by [`sphinx.builders.html.StandaloneHTMLBuilder.default_translator_class`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/html.py#L323)

* [`sphinx.builders.Builder.finish`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L493)
  - overriden by [`sphinx.builders.changes.ChangesBuilder.finish`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/changes.py#L165)
  - overriden by [`sphinx.builders.dummy.DummyBuilder.finish`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/dummy.py#L48)
  - overriden by [`sphinx.builders.gettext.MessageCatalogBuilder.finish`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/gettext.py#L259)
  - overriden by [`sphinx.builders.html.SingleFileHTMLBuilder.finish`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/html.py#L1258)
  - overriden by [`sphinx.builders.html.StandaloneHTMLBuilder.finish`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/html.py#L618)
  - overriden by [`sphinx.builders.latex.LaTeXBuilder.finish`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/latex/__init__.py#L223)
  - overriden by [`sphinx.builders.linkcheck.CheckExternalLinksBuilder.finish`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/linkcheck.py#L310)
  - overriden by [`sphinx.builders.manpage.ManualPageBuilder.finish`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/manpage.py#L106)
  - overriden by [`sphinx.builders.texinfo.TexinfoBuilder.finish`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/texinfo.py#L233)
  - overriden by [`sphinx.builders.text.TextBuilder.finish`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/text.py#L89)
  - overriden by [`sphinx.builders.xml.XMLBuilder.finish`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/xml.py#L102)

* [`sphinx.builders.Builder.get_asset_paths`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L195)
  - overriden by [`sphinx.builders.html.StandaloneHTMLBuilder.get_asset_paths`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/html.py#L371)

* [`sphinx.builders.Builder.get_outdated_docs`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L184)
  - overriden by [`sphinx.builders.changes.ChangesBuilder.get_outdated_docs`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/changes.py#L50)
  - overriden by [`sphinx.builders.dummy.DummyBuilder.get_outdated_docs`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/dummy.py#L32)
  - overriden by [`sphinx.builders.gettext.I18nBuilder.get_outdated_docs`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/gettext.py#L131)
  - overriden by [`sphinx.builders.html.SingleFileHTMLBuilder.get_outdated_docs`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/html.py#L1118)
  - overriden by [`sphinx.builders.html.StandaloneHTMLBuilder.get_outdated_docs`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/html.py#L333)
  - overriden by [`sphinx.builders.latex.LaTeXBuilder.get_outdated_docs`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/latex/__init__.py#L70)
  - overriden by [`sphinx.builders.linkcheck.CheckExternalLinksBuilder.get_outdated_docs`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/linkcheck.py#L272)
  - overriden by [`sphinx.builders.manpage.ManualPageBuilder.get_outdated_docs`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/manpage.py#L53)
  - overriden by [`sphinx.builders.texinfo.TexinfoBuilder.get_outdated_docs`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/texinfo.py#L116)
  - overriden by [`sphinx.builders.text.TextBuilder.get_outdated_docs`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/text.py#L47)
  - overriden by [`sphinx.builders.xml.XMLBuilder.get_outdated_docs`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/xml.py#L50)

* [`sphinx.builders.Builder.get_relative_uri`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L175)
  - overriden by [`sphinx.builders.html.SingleFileHTMLBuilder.get_relative_uri`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/html.py#L1132)
  - overriden by [`sphinx.builders.texinfo.TexinfoBuilder.get_relative_uri`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/texinfo.py#L127)
  - overriden by [`sphinx.builders.latex.LaTeXBuilder.get_relative_uri`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/latex/__init__.py#L81)
  * [`sphinx.util.osutil.relative_uri`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/osutil.py#L59)
  * [`sphinx.builders.Builder.get_target_uri`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L166)

* [`sphinx.builders.Builder.get_target_uri`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L166)
  - overriden by [`sphinx.builders.dummy.DummyBuilder.get_target_uri`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/dummy.py#L36)
  - overriden by [`sphinx.builders.gettext.I18nBuilder.get_target_uri`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/gettext.py#L127)
  - overriden by [`sphinx.builders.html.DirectoryHTMLBuilder.get_target_uri`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/html.py#L1083)
  - overriden by [`sphinx.builders.html.SerializingHTMLBuilder.get_target_uri`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/html.py#L1312)
  - overriden by [`sphinx.builders.html.SingleFileHTMLBuilder.get_target_uri`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/html.py#L1122)
  - overriden by [`sphinx.builders.html.StandaloneHTMLBuilder.get_target_uri`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/html.py#L956)
  - overriden by [`sphinx.builders.latex.LaTeXBuilder.get_target_uri`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/latex/__init__.py#L74)
  - overriden by [`sphinx.builders.linkcheck.CheckExternalLinksBuilder.get_target_uri`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/linkcheck.py#L268)
  - overriden by [`sphinx.builders.manpage.ManualPageBuilder.get_target_uri`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/manpage.py#L57)
  - overriden by [`sphinx.builders.texinfo.TexinfoBuilder.get_target_uri`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/texinfo.py#L120)
  - overriden by [`sphinx.builders.text.TextBuilder.get_target_uri`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/text.py#L67)
  - overriden by [`sphinx.builders.xml.XMLBuilder.get_target_uri`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/xml.py#L70)

* [`sphinx.builders.Builder.init`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L149)
  - overriden by [`sphinx.builders._epub_base.EpubBuilder.init`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/_epub_base.py#L151)
  - overriden by [`sphinx.builders.applehelp.AppleHelpBuilder.init`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/applehelp.py#L95)
  - overriden by [`sphinx.builders.changes.ChangesBuilder.init`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/changes.py#L43)
  - overriden by [`sphinx.builders.devhelp.DevhelpBuilder.init`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/devhelp.py#L60)
  - overriden by [`sphinx.builders.dummy.DummyBuilder.init`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/dummy.py#L28)
  - overriden by [`sphinx.builders.gettext.I18nBuilder.init`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/gettext.py#L119)
  - overriden by [`sphinx.builders.gettext.MessageCatalogBuilder.init`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/gettext.py#L219)
  - overriden by [`sphinx.builders.html.JSONHTMLBuilder.init`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/html.py#L1409)
  - overriden by [`sphinx.builders.html.SerializingHTMLBuilder.init`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/html.py#L1301)
  - overriden by [`sphinx.builders.html.StandaloneHTMLBuilder.init`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/html.py#L248)
  - overriden by [`sphinx.builders.htmlhelp.HTMLHelpBuilder.init`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/htmlhelp.py#L195)
  - overriden by [`sphinx.builders.latex.LaTeXBuilder.init`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/latex/__init__.py#L63)
  - overriden by [`sphinx.builders.linkcheck.CheckExternalLinksBuilder.init`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/linkcheck.py#L96)
  - overriden by [`sphinx.builders.manpage.ManualPageBuilder.init`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/manpage.py#L47)
  - overriden by [`sphinx.builders.qthelp.QtHelpBuilder.init`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/qthelp.py#L80)
  - overriden by [`sphinx.builders.texinfo.TexinfoBuilder.init`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/texinfo.py#L111)
  - overriden by [`sphinx.builders.text.TextBuilder.init`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/text.py#L42)
  - overriden by [`sphinx.builders.xml.XMLBuilder.init`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/xml.py#L46)

* [`sphinx.builders.Builder.post_process_images`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L200)
  - overriden by [`sphinx.builders.html.StandaloneHTMLBuilder.post_process_images`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/html.py#L837)

* [`sphinx.builders.Builder.prepare_writing`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L476)
  - overriden by [`sphinx.builders.dummy.DummyBuilder.prepare_writing`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/dummy.py#L40)
  - overriden by [`sphinx.builders.epub3.Epub3Builder.prepare_writing`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/epub3.py#L139)
  - overriden by [`sphinx.builders.gettext.I18nBuilder.prepare_writing`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/gettext.py#L135)
  - overriden by [`sphinx.builders.html.DirectoryHTMLBuilder.prepare_writing`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/html.py#L1102)
  - overriden by [`sphinx.builders.html.StandaloneHTMLBuilder.prepare_writing`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/html.py#L401)
  - overriden by [`sphinx.builders.linkcheck.CheckExternalLinksBuilder.prepare_writing`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/linkcheck.py#L276)
  - overriden by [`sphinx.builders.text.TextBuilder.prepare_writing`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/text.py#L71)
  - overriden by [`sphinx.builders.xml.XMLBuilder.prepare_writing`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/xml.py#L74)

* [`sphinx.builders.Builder.set_environment`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L109)
  * [`sphinx.environment.BuildEnvironment.set_versioning_method`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L273)

* [`sphinx.builders.Builder.write`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L407)
  - overriden by [`sphinx.builders.changes.ChangesBuilder.write`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/changes.py#L60)
  - overriden by [`sphinx.builders.html.SingleFileHTMLBuilder.write`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/html.py#L1240)
  - overriden by [`sphinx.builders.latex.LaTeXBuilder.write`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/latex/__init__.py#L117)
  - overriden by [`sphinx.builders.manpage.ManualPageBuilder.write`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/manpage.py#L63)
  - overriden by [`sphinx.builders.texinfo.TexinfoBuilder.write`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/texinfo.py#L152)
  * [`sphinx.config.Config.__getattr__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/config.py#L308)
  * [`sphinx.util.console.bold`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/console.py#L102)
  * [`sphinx.builders.Builder.prepare_writing`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L476)
  * [`sphinx.builders.Builder._write_parallel`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L447)
  * [`sphinx.builders.Builder._write_serial`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L438)

* [`sphinx.builders.Builder.write_doc`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L481)
  - overriden by [`sphinx.builders._epub_base.EpubBuilder.write_doc`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/_epub_base.py#L354)
  - overriden by [`sphinx.builders.dummy.DummyBuilder.write_doc`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/dummy.py#L44)
  - overriden by [`sphinx.builders.gettext.I18nBuilder.write_doc`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/gettext.py#L143)
  - overriden by [`sphinx.builders.html.StandaloneHTMLBuilder.write_doc`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/html.py#L592)
  - overriden by [`sphinx.builders.htmlhelp.HTMLHelpBuilder.write_doc`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/htmlhelp.py#L220)
  - overriden by [`sphinx.builders.linkcheck.CheckExternalLinksBuilder.write_doc`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/linkcheck.py#L280)
  - overriden by [`sphinx.builders.text.TextBuilder.write_doc`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/text.py#L75)
  - overriden by [`sphinx.builders.xml.XMLBuilder.write_doc`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/xml.py#L78)

* [`sphinx.builders.Builder.write_doc_serialized`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L486)
  - overriden by [`sphinx.builders.html.StandaloneHTMLBuilder.write_doc_serialized`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/html.py#L610)

### `sphinx.cmd.build`

* [`sphinx.cmd.build.build_main`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/cmd/build.py#L19)
  * [`sphinx.cmdline.main`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/cmdline.py#L198)

* [`sphinx.cmd.build.main`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/cmd/build.py#L33)
  * [`sphinx.cmd.build.make_main`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/cmd/build.py#L19)
  * [`sphinx.cmd.build.build_main`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/cmd/build.py#L19)

* [`sphinx.cmd.build.make_main`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/cmd/build.py#L19)
  * [`sphinx.make_mode.run_make_mode`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/make_mode.py#L157)

### `sphinx.cmdline`

* [`sphinx.cmdline.get_parser`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/cmdline.py#L104)
  * [`sphinx.cmdline.jobs_argument`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/cmdline.py#L87)

* [`sphinx.cmdline.handle_exception`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/cmdline.py#L36)
  * [`sphinx.util.console.red`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/console.py#L102)
  * [`sphinx.util.pycompat.terminal_safe`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/pycompat.py#L53)
  * [`sphinx.util.save_traceback`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/__init__.py#L207)
  * [`sphinx.util.format_exception_cut_frames`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/__init__.py#L484)

* [`sphinx.cmdline.main`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/cmdline.py#L198)
  * [`sphinx.cmdline.get_parser`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/cmdline.py#L104)
  * [`sphinx.util.osutil.abspath`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/osutil.py#L226)
  * [`sphinx.util.console.color_terminal`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/console.py#L61)
  * [`sphinx.util.console.nocolor`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/console.py#L78)
  * [`sphinx.util.Tee.__init__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/__init__.py#L368)
  * [`sphinx.util.docutils.patch_docutils`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/docutils.py#L69)
  * [`sphinx.util.docutils.docutils_namespace`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/docutils.py#L45)
  * [`sphinx.application.Sphinx.__init__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/application.py#L112)
  * [`sphinx.application.Sphinx.build`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/application.py#L320)
  * [`sphinx.cmdline.handle_exception`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/cmdline.py#L36)

### `sphinx.config`

* [`sphinx.config.Config.__getattr__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/config.py#L308)
  * [`sphinx.locale.__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/locale/__init__.py#L251)

* [`sphinx.config.Config.__getitem__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/config.py#L319)
  * [`sphinx.config.Config.__getattr__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/config.py#L308)

* [`sphinx.config.Config.__init__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/config.py#L148)
  - tests `SOURCE_DATE_EPOCH` environment variable
  * [`sphinx.util.osutil.cd`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/osutil.py#L242)
  * [`sphinx.util.pycompat.execfile_`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/pycompat.py#L129)
  * [`sphinx.errors.ConfigError`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/errors.py#L55)
  * [`sphinx.util.i18n.format_date`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/i18n.py#L216)

* [`sphinx.config.Config.__iter__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/config.py#L335)
  * [`sphinx.config.ConfigValue`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/config.py#L52)
  * [`sphinx.config.Config.__getattr__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/config.py#L308)

* [`sphinx.config.Config.check_types`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/config.py#L190)
  * [`sphinx.config.ENUM.match`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/config.py#L67)

* [`sphinx.config.Config.convert_overrides`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/config.py#L242)
  * [`sphinx.locale.__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/locale/__init__.py#L251)

* [`sphinx.config.Config.filter`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/config.py#L344)
  * [`sphinx.config.Config.__iter__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/config.py#L335)

* [`sphinx.config.Config.init_values`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/config.py#L283)
  * [`sphinx.locale.__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/locale/__init__.py#L251)
  * [`sphinx.config.Config.convert_overrides`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/config.py#L242)
  * [`sphinx.config.Config.__getattr__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/config.py#L308)

* [`sphinx.config.Config.pre_init_values`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/config.py#L268)
  * [`sphinx.config.Config.convert_overrides`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/config.py#L242)

### `sphinx.domains`

* [`sphinx.domains.Domain.check_consistency`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/__init__.py#L262)
  - overriden by [`sphinx.domains.std.StandardDomain.check_consistency`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/std.py#L637)

* [`sphinx.domains.Domain.clear_doc`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/__init__.py#L243)
  - overriden by [`sphinx.domains.c.CDomain.clear_doc`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/c.py#L282)
  - overriden by [`sphinx.domains.cpp.CPPDomain.clear_doc`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/cpp.py#L6028)
  - overriden by [`sphinx.domains.javascript.JavaScriptDomain.clear_doc`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/javascript.py#L327)
  - overriden by [`sphinx.domains.python.PythonDomain.clear_doc`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/python.py#L750)
  - overriden by [`sphinx.domains.rst.ReSTDomain.clear_doc`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/rst.py#L134)
  - overriden by [`sphinx.domains.std.StandardDomain.clear_doc`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/std.py#L525)

* [`sphinx.domains.Domain.get_full_qualified_name`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/__init__.py#L336)
  - overriden by [`sphinx.domains.cpp.CPPDomain.get_full_qualified_name`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/cpp.py#L6202)
  - overriden by [`sphinx.domains.javascript.JavaScriptDomain.get_full_qualified_name`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/javascript.py#L401)
  - overriden by [`sphinx.domains.python.PythonDomain.get_full_qualified_name`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/python.py#L898)
  - overriden by [`sphinx.domains.std.StandardDomain.get_full_qualified_name`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/std.py#L960)

* [`sphinx.domains.Domain.get_objects`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/__init__.py#L309)
  - overriden by [`sphinx.domains.c.CDomain.get_objects`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/c.py#L321)
  - overriden by [`sphinx.domains.cpp.CPPDomain.get_objects`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/cpp.py#L6189)
  - overriden by [`sphinx.domains.javascript.JavaScriptDomain.get_objects`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/javascript.py#L395)
  - overriden by [`sphinx.domains.python.PythonDomain.get_objects`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/python.py#L890)
  - overriden by [`sphinx.domains.rst.ReSTDomain.get_objects`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/rst.py#L173)
  - overriden by [`sphinx.domains.std.StandardDomain.get_objects`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/std.py#L874)

* [`sphinx.domains.Domain.get_type_name`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/__init__.py#L329)
  - overriden by [`sphinx.domains.std.StandardDomain.get_type_name`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/std.py#L896)

* [`sphinx.domains.Domain.merge_domaindata`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/__init__.py#L248)
  - overriden by [`sphinx.domains.c.CDomain.merge_domaindata`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/c.py#L288)
  - overriden by [`sphinx.domains.cpp.CPPDomain.merge_domaindata`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/cpp.py#L6047)
  - overriden by [`sphinx.domains.javascript.JavaScriptDomain.merge_domaindata`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/javascript.py#L336)
  - overriden by [`sphinx.domains.python.PythonDomain.merge_domaindata`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/python.py#L759)
  - overriden by [`sphinx.domains.rst.ReSTDomain.merge_domaindata`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/rst.py#L140)
  - overriden by [`sphinx.domains.std.StandardDomain.merge_domaindata`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/std.py#L548)

* [`sphinx.domains.Domain.process_field_xref`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/__init__.py#L267)
  - overriden by [`sphinx.domains.cpp.CPPDomain.process_field_xref`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/cpp.py#L6043)

* [`sphinx.domains.Domain.process_doc`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/__init__.py#L257)
  - overriden by [`sphinx.domains.cpp.CPPDomain.process_doc`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/cpp.py#L6036)
  - overriden by [`sphinx.domains.std.StandardDomain.process_doc`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/std.py#L572)

* [`sphinx.domains.Domain.resolve_any_xref`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/__init__.py#L292)
  - overriden by [`sphinx.domains.c.CDomain.resolve_any_xref`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/c.py#L309)
  - overriden by [`sphinx.domains.cpp.CPPDomain.resolve_any_xref`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/cpp.py#L6176)
  - overriden by [`sphinx.domains.javascript.JavaScriptDomain.resolve_any_xref`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/javascript.py#L383)
  - overriden by [`sphinx.domains.python.PythonDomain.resolve_any_xref`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/python.py#L856)
  - overriden by [`sphinx.domains.rst.ReSTDomain.resolve_any_xref`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/rst.py#L159)
  - overriden by [`sphinx.domains.std.StandardDomain.resolve_any_xref`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/std.py#L852)

* [`sphinx.domains.Domain.resolve_xref`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/__init__.py#L274)
  - overriden by [`sphinx.domains.c.CDomain.resolve_xref`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/c.py#L295)
  - overriden by [`sphinx.domains.cpp.CPPDomain.resolve_xref`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/cpp.py#L6170)
  - overriden by [`sphinx.domains.javascript.JavaScriptDomain.resolve_xref`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/javascript.py#L371)
  - overriden by [`sphinx.domains.python.PythonDomain.resolve_xref`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/python.py#L833)
  - overriden by [`sphinx.domains.rst.ReSTDomain.resolve_xref`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/rst.py#L147)
  - overriden by [`sphinx.domains.std.StandardDomain.resolve_xref`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/std.py#L670)

### `sphinx.environment`

* [`sphinx.environment.BuildEnvironment.__init__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L168)
  * [`sphinx.util.FilenameUniqDict.__init__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/__init__.py#L126)

* [`sphinx.environment.BuildEnvironment._read_parallel`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L586)
  * [`sphinx.application.Sphinx.emit`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/application.py#L436)
    - emits `env-purge-doc`
  * [`sphinx.environment.BuildEnvironment.clear_doc`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L307)
  * [`sphinx.environment.BuildEnvironment.read_doc`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L647)
  * [`sphinx.environment.BuildEnvironment.dumps`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L155)
  * [`sphinx.environment.BuildEnvironment.loads`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L120)
  * [`sphinx.environment.BuildEnvironment.merge_info_from`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L321)
  * [`sphinx.util.parallel.ParallelTasks.__init__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/parallel.py#L62)
  * [`sphinx.util.parallel.make_chunks`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/parallel.py#L138)
  * [`sphinx.util.status_iterator`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/__init__.py#L589)
  * [`sphinx.util.parallel.ParallelTasks.add_task`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/parallel.py#L97)
  * [`sphinx.util.console.bold`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/console.py#L102)
  * [`sphinx.util.parallel.ParallelTasks.join`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/parallel.py#L110)

* [`sphinx.environment.BuildEnvironment._read_serial`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L577)
  * [`sphinx.util.status_iterator`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/__init__.py#L589)
  * [`sphinx.application.Sphinx.emit`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/application.py#L436)
    - emits `env-purge-doc`
  * [`sphinx.environment.BuildEnvironment.clear_doc`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L307)
  * [`sphinx.environment.BuildEnvironment.read_doc`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L647)

* [`sphinx.environment.BuildEnvironment.apply_post_transforms`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L849)
  * [`sphinx.transforms.SphinxTransformer`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/transforms/__init__.py#L78)
  * [`sphinx.transforms.SphinxTransformer.set_environment`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/transforms/__init__.py#L86)
  * [`sphinx.registry.SphinxComponentRegistry.get_post_transforms`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/registry.py#L288)
  * [`sphinx.transforms.SphinxTransformer.apply_transforms`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/transforms/__init__.py#L90)
  * [`sphinx.application.Sphinx.emit`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/application.py#L436)
    - emits `doctree-resolved`

* [`sphinx.environment.BuildEnvironment.check_consistency`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L911)
  * [`sphinx.config.Config.__getattr__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/config.py#L308)
  * [`sphinx.domains.Domain.check_consistency`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/__init__.py#L262)
  * [`sphinx.application.Sphinx.emit`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/application.py#L436)
    - emits `env-check-consistency`

* [`sphinx.environment.BuildEnvironment.check_dependents`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L617)
  * [`sphinx.application.Sphinx.emit`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/application.py#L436)
    - emits `env-get-updated`

* [`sphinx.environment.BuildEnvironment.clear_doc`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L307)
  * [`sphinx.domains.Domain.clear_doc`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/__init__.py#L243)

* [`sphinx.environment.BuildEnvironment.doc2path`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L355)
  * [`sphinx.config.Config.__getattr__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/config.py#L308)

* [`sphinx.environment.BuildEnvironment.dump`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L132)
  * [`sphinx.config.Config.__delitem__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/config.py#L327)

* [`sphinx.environment.BuildEnvironment.dumps`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L155)
  * [`sphinx.environment.BuildEnvironment.dump`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L132)

* [`sphinx.environment.BuildEnvironment.find_files`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L405)
  * [`sphinx.config.Config.__getattr__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/config.py#L308)
  * [`sphinx.util.matching.compile_matchers`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/matching.py#L66)
  * [`sphinx.builders.Builder.get_asset_paths`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L195)
  * [`sphinx.util.get_matching_docs`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/__init__.py#L105)
  * [`sphinx.environment.BuildEnvironment.doc2path`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L355)
  * [`sphinx.util.i18n.find_catalog_files`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/i18n.py#L91)

* [`sphinx.environment.BuildEnvironment.frompickle`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L126)
  * [`sphinx.environment.BuildEnvironment.load`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L102)

* [`sphinx.environment.BuildEnvironment.get_and_resolve_doctree`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L803)
  * [`sphinx.environment.BuildEnvironment.get_doctree`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L778)
  * [`sphinx.environment.BuildEnvironment.apply_post_transforms`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L849)
  * [`sphinx.environment.adapters.toctree.TocTree.__init__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/adapters/toctree.py#L29)
  * [`sphinx.environment.adapters.toctree.TocTree.resolve`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/adapters/toctree.py#L49)
  * [`sphinx.addnodes.toctree`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/addnodes.py#L57)

* [`sphinx.environment.BuildEnvironment.get_doctree`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L778)
  * [`sphinx.environment.BuildEnvironment.doc2path`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L355)
  * [`sphinx.util.docutils.WarningStream`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/docutils.py#L160)

* [`sphinx.environment.BuildEnvironment.get_domain`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L765)
  * [`sphinx.errors.ExtensionError.__init__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/errors.py#L35)

* [`sphinx.environment.BuildEnvironment.get_outdated_files`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L442)
  * [`sphinx.environment.BuildEnvironment.doc2path`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L355)

* [`sphinx.environment.BuildEnvironment.loads`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L120)
  * [`sphinx.environment.BuildEnvironment.load`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L102)

* [`sphinx.environment.BuildEnvironment.merge_info_from`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L321)
  * [`sphinx.domains.Domain.merge_domaindata`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/__init__.py#L248)
  * [`sphinx.application.Sphinx.emit`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/application.py#L436)
    - emits `env-merge-info`

* [`sphinx.environment.BuildEnvironment.note_dependency`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L705)
  * [`sphinx.environment.BuildEnvironment.docname`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L689)

* [`sphinx.environment.BuildEnvironment.path2doc`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L342)
  * [`sphinx.util.osutil.relpath`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/osutil.py#L208)
  * [`sphinx.config.Config.__getattr__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/config.py#L308)

* [`sphinx.environment.BuildEnvironment.prepare_settings`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L628)
  * [`sphinx.config.Config.__getattr__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/config.py#L308)

* [`sphinx.environment.BuildEnvironment.read_doc`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L647)
  * [`sphinx.environment.BuildEnvironment.prepare_settings`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L628)
  * [`sphinx.environment.BuildEnvironment.note_dependency`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L705)
  * [`sphinx.util.docutils.sphinx_domains.__init__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/docutils.py#L89)
  * [`sphinx.util.docutils.sphinx_domains.__enter__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/docutils.py#L95)
  * [`sphinx.util.docutils.sphinx_domains.__exit__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/docutils.py#L99)
  * [`sphinx.config.Config.__getattr__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/config.py#L308)
  * [`sphinx.io.read_doc`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/io.py#L277)
  * [`sphinx.environment.BuildEnvironment.doc2path`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L355)
  * [`sphinx.domains.Domain.process_doc`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/__init__.py#L257)
  * [`sphinx.application.Sphinx.emit`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/application.py#L436)
    - emits `doctree-read`
  * [`sphinx.versioning.prepare`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/versioning.py#L179)
  * [`sphinx.environment.BuildEnvironment.write_doctree`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L788)

* [`sphinx.environment.BuildEnvironment.set_versioning_method`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L273)
  * [`sphinx.errors.SphinxError`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/errors.py#L18)

* [`sphinx.environment.BuildEnvironment.topickle`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L161)
  * [`sphinx.environment.BuildEnvironment.dump`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L132)

* [`sphinx.environment.BuildEnvironment.update`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L493)
  * [`sphinx.config.Config.filter`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/config.py#L344)
  * [`sphinx.config.Config.__getitem__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/config.py#L319)
  * [`sphinx.config.Config.__getattr__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/config.py#L308)
  * [`sphinx.environment.BuildEnvironment.find_files`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L405)
  * [`sphinx.util.console.bold`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/console.py#L102)
  * [`sphinx.environment.BuildEnvironment.get_outdated_files`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L442)
  * [`sphinx.application.Sphinx.emit`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/application.py#L436)
    - emits `env-get-outdated`
    - emits `env-purge-doc`
    - emits `env-before-read-docs`
    - emits `env-updated`
  * [`sphinx.environment.BuildEnvironment.clear_doc`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L307)
  * [`sphinx.application.Sphinx.is_parallel_allowed`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/application.py#L683)
  * [`sphinx.environment.BuildEnvironment._read_parallel`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L586)
  * [`sphinx.environment.BuildEnvironment._read_serial`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L577)
  * [`sphinx.errors.SphinxError`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/errors.py#L18)
  * [`sphinx.environment.BuildEnvironment.doc2path`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L355)

* [`sphinx.environment.BuildEnvironment.write_doctree`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L788)
  * [`sphinx.environment.BuildEnvironment.doc2path`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L355)
  * [`sphinx.util.osutil.ensuredir`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/osutil.py#L83)

### `sphinx.environment.adapters.toctree`

* [`sphinx.environment.adapters.toctree.TocTree._toctree_prune`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/adapters/toctree.py#L267)
  * [`sphinx.addnodes.compact_paragraph`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/addnodes.py#L221)
  * [`sphinx.environment.adapters.toctree.TocTree._toctree_prune`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/adapters/toctree.py#L267)

* [`sphinx.environment.adapters.toctree.TocTree.resolve`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/adapters/toctree.py#L49)
  * [`sphinx.environment.adapters.toctree.TocTree.get_toctree_ancestors`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/adapters/toctree.py#L254)
  * [`sphinx.addnodes.compact_paragraph`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/addnodes.py#L221)
  * [`sphinx.util.nodes.clean_astext`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/nodes.py#L216)
  * [`sphinx.environment.adapters.toctree.TocTree._toctree_prune`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/adapters/toctree.py#L267)
  * [`sphinx.util.nodes.process_only_nodes`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/nodes.py#L364)
  * [`sphinx.addnodes.toctree`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/addnodes.py#L57)
  * [`sphinx.builders.Builder.get_relative_uri`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/__init__.py#L175)

### `sphinx.errors`

* [`sphinx.errors.ConfigError`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/errors.py#L55)
  * [`sphinx.errors.SphinxError`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/errors.py#L18)

* [`sphinx.errors.ExtensionError.__init__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/errors.py#L35)
  * [`sphinx.errors.SphinxError`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/errors.py#L18)

* [`sphinx.errors.SphinxParallelError.__init__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/errors.py#L80)
  * [`sphinx.errors.SphinxError`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/errors.py#L18)

* [`sphinx.errors.VersionRequirementError`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/errors.py#L63)
  * [`sphinx.errors.SphinxError`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/errors.py#L18)

### `sphinx.extension`

* [`sphinx.extension.verify_required_extensions`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/extension.py#L44)
  * [`sphinx.locale.__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/locale/__init__.py#L251)
  * [`sphinx.errors.VersionRequirementError`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/errors.py#L63)

### `sphinx.io`

* [`sphinx.io.read_doc`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/io.py#L277)
  * [`sphinx.registry.SphinxComponentRegistry.get_source_input`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/registry.py#L243)
  * [`sphinx.io.SphinxStandaloneReader.__init__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/io.py#L98)
  * [`sphinx.config.Config.__getattr__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/config.py#L308)
  * [`sphinx.registry.SphinxComponentRegistry.create_source_parser`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/registry.py#L227)
  * [`sphinx.io.SphinxDummyWriter`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/io.py#L141)
  * [`sphinx.io.SphinxDummySourceClass`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/io.py#L151)

* [`sphinx.io.SphinxStandaloneReader.__init__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/io.py#L98)
  * [`sphinx.registry.SphinxComponentRegistry.get_transforms`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/registry.py#L279)
  * [`sphinx.io.SphinxBaseReader.__init__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/io.py#L60)

### `sphinx.make_mode`

* reads `SPHINXPROJ` environmet variable (default: `<project>`)

* [`sphinx.make_mode.Make.__init__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/make_mode.py#L66)
  - reads `MAKE` environment variable (default: `make`)

* [`sphinx.make_mode.Make.build_clean`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/make_mode.py#L77)
  * [`sphinx.make_mode.Make.builddir_join`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/make_mode.py#L73)
  * [`sphinx.util.osutil.rmtree`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/osutil.py#L325)

* [`sphinx.make_mode.Make.build_gettext`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/make_mode.py#L133)
  * [`sphinx.make_mode.Make.builddir_join`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/make_mode.py#L73)
  * [`sphinx.make_mode.Make.run_generic_build`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/make_mode.py#L140)

* [`sphinx.make_mode.Make.build_help`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/make_mode.py#L89)
  * [`sphinx.util.console.color_terminal`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/console.py#L61)
  * [`sphinx.util.console.nocolor`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/console.py#L78)
  * [`sphinx.util.console.bold`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/console.py#L102)
  * [`sphinx.util.console.blue`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/console.py#L102)

* [`sphinx.make_mode.Make.build_info`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/make_mode.py#L122)
  * [`sphinx.make_mode.Make.run_generic_build`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/make_mode.py#L140)
  * [`sphinx.util.osutil.cd`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/osutil.py#L242)
  * [`sphinx.make_mode.Make.builddir_join`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/make_mode.py#L73)

* [`sphinx.make_mode.Make.build_latexpdf`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/make_mode.py#L100)
  * [`sphinx.make_mode.Make.run_generic_build`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/make_mode.py#L140)
  * [`sphinx.util.osutil.cd`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/osutil.py#L242)
  * [`sphinx.make_mode.Make.builddir_join`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/make_mode.py#L73)

* [`sphinx.make_mode.Make.build_latexpdfja`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/make_mode.py#L111)
  * [`sphinx.make_mode.Make.run_generic_build`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/make_mode.py#L140)
  * [`sphinx.util.osutil.cd`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/osutil.py#L242)
  * [`sphinx.make_mode.Make.builddir_join`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/make_mode.py#L73)

* [`sphinx.make_mode.Make.run_generic_build`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/make_mode.py#L140)
  - reads `PAPER` environment variable
  * [`sphinx.make_mode.Make.builddir_join`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/make_mode.py#L73)
  * [`sphinx.cmdline.main`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/cmdline.py#L198)

* [`sphinx.make_mode.run_make_mode`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/make_mode.py#L157)
  * [`sphinx.make_mode.Make.__init__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/make_mode.py#L66)
  * [`sphinx.make_mode.Make.build_clean`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/make_mode.py#L77)
  * [`sphinx.make_mode.Make.build_help`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/make_mode.py#L89)
  * [`sphinx.make_mode.Make.build_latexpdf`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/make_mode.py#L100)
  * [`sphinx.make_mode.Make.build_latexpdfja`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/make_mode.py#L111)
  * [`sphinx.make_mode.Make.build_info`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/make_mode.py#L122)
  * [`sphinx.make_mode.Make.build_gettext`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/make_mode.py#L133)
  * [`sphinx.make_mode.Make.run_generic_build`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/make_mode.py#L140)

### `sphinx.registry`

* [`sphinx.registry.SphinxComponentRegistry.add_source_parser`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/registry.py#L200)
  * [`sphinx.locale.__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/locale/__init__.py#L251)
  * [`sphinx.errors.ExtensionError.__init__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/errors.py#L35)

* [`sphinx.registry.SphinxComponentRegistry.create_builder`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/registry.py#L94)
  * [`sphinx.locale.__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/locale/__init__.py#L251)
  * [`sphinx.errors.SphinxError`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/errors.py#L18)

* [`sphinx.registry.SphinxComponentRegistry.create_domains`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/registry.py#L112)
  * [`sphinx.domains.Domain.__init__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/__init__.py#L161)
  * [`sphinx.domains.Domain.add_object_type`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/__init__.py#L191)

* [`sphinx.registry.SphinxComponentRegistry.create_source_parser`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/registry.py#L227)
  * [`sphinx.registry.SphinxComponentRegistry.get_source_parser`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/registry.py#L207)
  * [`sphinx.parsers.Parser`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/parsers.py#L26)
  * [`sphinx.parsers.Parser.set_application`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/parsers.py#L46)

* [`sphinx.registry.SphinxComponentRegistry.get_source_input`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/registry.py#L243)
  * [`sphinx.registry.SphinxComponentRegistry.get_source_parser`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/registry.py#L207)
  * [`sphinx.errors.SphinxError`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/errors.py#L18)
  * [`sphinx.locale.__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/locale/__init__.py#L251)

* [`sphinx.registry.SphinxComponentRegistry.get_source_parser`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/registry.py#L207)
  * [`sphinx.errors.SphinxError`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/errors.py#L18)
  * [`sphinx.locale.__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/locale/__init__.py#L251)
  * [`sphinx.util.import_object`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/__init__.py#L534)

* [`sphinx.registry.SphinxComponentRegistry.load_extension`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/registry.py#L300)
  * [`sphinx.locale.__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/locale/__init__.py#L251)
  * [`sphinx.util.logging.SphinxLoggerAdapter.verbose`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/logging.py#L117)
  * [`sphinx.errors.ExtensionError.__init__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/errors.py#L35)
  * [`sphinx.errors.VersionRequirementError`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/errors.py#L63)
  * [`sphinx.extension.Extension.__init__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/extension.py#L27)

* [`sphinx.registry.SphinxComponentRegistry.preload_builder`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/registry.py#L79)
  * [`sphinx.locale.__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/locale/__init__.py#L251)
  * [`sphinx.errors.SphinxError`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/errors.py#L18)
  * [`sphinx.registry.SphinxComponentRegistry.load_extension`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/registry.py#L300)

### `sphinx.transforms`

* [`sphinx.transforms.SphinxTransformer.apply_transforms`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/transforms/__init__.py#L90)
  * [`sphinx.util.docutils.new_document`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/docutils.py#L239)

### `sphinx.util`

* [`sphinx.util.get_matching_files`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/__init__.py#L78)
  * [`sphinx.util.osutil.walk`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/osutil.py#L97)
  * [`sphinx.util.path_stabilize`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/__init__.py#L69)

* [`sphinx.util.get_matching_docs`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/__init__.py#L105)
  * [`sphinx.util.get_matching_files`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/__init__.py#L78)

* [`sphinx.util.import_object`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/__init__.py#L534)
  * [`sphinx.errors.ExtensionError.__init__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/errors.py#L35)

* [`sphinx.util.old_status_iterator`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/__init__.py#L574)
  * [`sphinx.util.display_chunk`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/__init__.py#L565)
  * [`sphinx.util.console.bold`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/console.py#L102)

* [`sphinx.util.save_traceback`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/__init__.py#L207)
  * [`sphinx.util.console.strip_colors`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/console.py#L95)
  * [`sphinx.util.force_decode`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/__init__.py#L414)

* [`sphinx.util.status_iterator`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/__init__.py#L589)
  * [`sphinx.util.display_chunk`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/__init__.py#L565)
  * [`sphinx.util.old_status_iterator`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/__init__.py#L574)
  * [`sphinx.util.console.bold`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/console.py#L102)
  * [`sphinx.util.console.colorize`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/console.py#L90)
  * [`sphinx.util.console.term_width_line`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/console.py#L51)

### `sphinx.util.console`

* [`sphinx.util.console.blue`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/console.py#L102)
  * [`sphinx.util.console.colorize`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/console.py#L90)

* [`sphinx.util.console.bold`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/console.py#L102)
  * [`sphinx.util.console.colorize`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/console.py#L90)

* [`sphinx.util.console.color_terminal`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/console.py#L61)
  - tests `COLORTERM` environment variable
  - reads `TERM` environment variable (default: `dumb`)

* [`sphinx.util.console.red`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/console.py#L102)
  * [`sphinx.util.console.colorize`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/console.py#L90)

### `sphinx.util.docutils`

* [`sphinx.util.docutils.patch_docutils`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/docutils.py#L69)
  * [`sphinx.util.docutils.patched_get_language`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/docutils.py#L58)

* [`sphinx.util.docutils.sphinx_domains.__enter__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/docutils.py#L95)
  * [`sphinx.util.docutils.sphinx_domains.enable`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/docutils.py#L103)

* [`sphinx.util.docutils.sphinx_domains.__exit__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/docutils.py#L99)
  * [`sphinx.util.docutils.sphinx_domains.disable`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/docutils.py#L111)

* [`sphinx.util.docutils.sphinx_domains.enable`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/docutils.py#L103)
  * [`sphinx.util.docutils.sphinx_domains.lookup_directive`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/docutils.py#L145)
  * [`sphinx.util.docutils.sphinx_domains.lookup_role`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/docutils.py#L152)

* [`sphinx.util.docutils.sphinx_domains.lookup_directive`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/docutils.py#L145)
  * [`sphinx.util.docutils.sphinx_domains.lookup_domain_element`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/docutils.py#L116)

* [`sphinx.util.docutils.sphinx_domains.lookup_domain_element`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/docutils.py#L116)
  * [`sphinx.environment.BuildEnvironment.get_domain`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L765)
  * [`sphinx.domains.Domain.role`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/__init__.py#L203)
  * [`sphinx.domains.Domain.directive`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/domains/__init__.py#L221)

* [`sphinx.util.docutils.sphinx_domains.lookup_role`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/docutils.py#L152)
  * [`sphinx.util.docutils.sphinx_domains.lookup_domain_element`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/docutils.py#L116)

### `sphinx.util.i18n`

* [`sphinx.util.i18n.CatalogInfo`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/i18n.py#L37)
  * [`sphinx.util.i18n.LocaleFileInfoBase`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/i18n.py#L34)

* [`sphinx.util.i18n.find_catalog_files`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/i18n.py#L91)
  * [`sphinx.util.i18n.find_catalog`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/i18n.py#L81)
  * [`sphinx.util.osutil.relpath`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/osutil.py#L208)

* [`sphinx.util.i18n.find_catalog_source_files`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/i18n.py#L103)
  * [`sphinx.util.osutil.walk`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/osutil.py#L97)
  * [`sphinx.util.osutil.relpath`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/osutil.py#L208)
  * [`sphinx.util.i18n.CatalogInfo`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/i18n.py#L37)
  * [`sphinx.util.i18n.CatalogInfo.is_outdated`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/i18n.py#L59)

* [`sphinx.util.i18n.format_date`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/i18n.py#L216)
  - reads `SOURCE_DATE_EPOCH` environment variable
  * [`sphinx.util.i18n.babel_format_date`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/i18n.py#L195)

### `sphinx.util.logging`

* [`sphinx.util.logging.MemoryHandler.flushTo`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/logging.py#L201)
  * [`sphinx.util.logging.SphinxLoggerAdapter.handle`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/logging.py#L137)

* [`sphinx.util.logging.pending_warnings`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/logging.py#L218)
  * [`sphinx.util.logging.MemoryHandler.__init__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/logging.py#L193)
  * [`sphinx.util.logging.MemoryHandler.flushTo`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/logging.py#L201)

* [`sphinx.util.logging.setup`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/logging.py#L489)
  * [`sphinx.util.logging.SafeEncodingWriter.__init__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/logging.py#L458)
  * [`sphinx.util.logging.NewLineStreamHandler`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/logging.py#L168)
  * [`sphinx.util.logging.InfoFilter`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/logging.py#L302)
  * [`sphinx.util.logging.ColorizeFormatter`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/logging.py#L442)
  * [`sphinx.util.logging.WarningStreamHandler`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/logging.py#L142)
  * [`sphinx.util.logging.WarningSuppressor.__init__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/logging.py#L336)
  * [`sphinx.util.logging.WarningLogRecordTranslator.__init__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/logging.py#L402)
  * [`sphinx.util.logging.WarningIsErrorFilter.__init__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/logging.py#L362)
  * [`sphinx.util.logging.LastMessagesWriter.__init__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/logging.py#L480)

* [`sphinx.util.logging.SphinxLoggerAdapter.verbose`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/logging.py#L117)
  * [`sphinx.util.logging.SphinxLoggerAdapter.log`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/logging.py#L109)

### `sphinx.util.matching`

* [`sphinx.util.matching.compile_matchers`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/matching.py#L66)
  * [`sphinx.util.matching._translate_pattern`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/matching.py#L19)

### `sphinx.util.nodes`

* [`sphinx.util.nodes.process_only_nodes`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/nodes.py#L364)
  * [`sphinx.addnodes.only`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/addnodes.py#L229)
  * [`sphinx.util.tags.Tags.eval_condition`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/tags.py#L72)

### `sphinx.util.osutil`

* [`sphinx.util.osutil.walk`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/osutil.py#L97)
  * [`sphinx.util.osutil.walk`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/osutil.py#L97)

### `sphinx.util.parallel`

* [`sphinx.util.parallel.ParallelTasks._join_one`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/parallel.py#L115)
  * [`sphinx.errors.SphinxParallelError.__init__`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/errors.py#L80)
  * [`sphinx.util.logging.SphinxLoggerAdapter.handle`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/logging.py#L137)

* [`sphinx.util.parallel.ParallelTasks.add_task`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/parallel.py#L97)
  * [`sphinx.util.parallel.ParallelTasks._join_one`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/parallel.py#L115)

* [`sphinx.util.parallel.ParallelTasks.join`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/parallel.py#L110)
  * [`sphinx.util.parallel.ParallelTasks._join_one`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/parallel.py#L115)

### `sphinx.util.pycompat`

* [`sphinx.util.pycompat.execfile_`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/pycompat.py#L129)
  * [`sphinx.util.pycompat.convert_with_2to3`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/pycompat.py#L65)

### `sphinx.util.tags`

* [`sphinx.util.tags.Tags.eval_condition`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/tags.py#L72)
  - overriden by [`sphinx.builders.gettext.I18nTags.eval_condition`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/builders/gettext.py#L104)
  * [`sphinx.util.tags.BooleanParser`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/util/tags.py#L22)

### `sphinx.versioning`

* [`sphinx.versioning.get_ratio`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/versioning.py#L121)
  * [`sphinx.versioning.levenshtein_distance`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/versioning.py#L135)

* [`sphinx.versioning.merge_doctrees`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/versioning.py#L53)
  * [`sphinx.versioning.get_ratio`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/versioning.py#L121)

* [`sphinx.versioning.prepare`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/versioning.py#L179)
  * [`sphinx.versioning.UIDTransform`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/versioning.py#L156)
  * [`sphinx.versioning.UIDTransform.apply`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/versioning.py#L160)

* [`sphinx.versioning.UIDTransform`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/versioning.py#L156)
  * [`sphinx.transforms.SphinxTransform`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/transforms/__init__.py#L45)

* [`sphinx.versioning.UIDTransform.apply`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/versioning.py#L160)
  * [`sphinx.environment.BuildEnvironment.doc2path`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/environment/__init__.py#L355)
  * [`sphinx.versioning.add_uids`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/versioning.py#L37)
  * [`sphinx.versioning.merge_doctrees`](https://github.com/sphinx-doc/sphinx/blob/f7b3292d87e9a2b7eae0b4ef72e87779beefc699/sphinx/versioning.py#L53)
