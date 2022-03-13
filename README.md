oclif-hello-world
=================

oclif example Hello World CLI

[![oclif](https://img.shields.io/badge/cli-oclif-brightgreen.svg)](https://oclif.io)
[![Version](https://img.shields.io/npm/v/oclif-hello-world.svg)](https://npmjs.org/package/oclif-hello-world)
[![CircleCI](https://circleci.com/gh/oclif/hello-world/tree/main.svg?style=shield)](https://circleci.com/gh/oclif/hello-world/tree/main)
[![Downloads/week](https://img.shields.io/npm/dw/oclif-hello-world.svg)](https://npmjs.org/package/oclif-hello-world)
[![License](https://img.shields.io/npm/l/oclif-hello-world.svg)](https://github.com/oclif/hello-world/blob/main/package.json)

<!-- toc -->
* [Usage](#usage)
* [Commands](#commands)
<!-- tocstop -->
# Usage
<!-- usage -->
```sh-session
$ npm install -g tmpl
$ tmpl COMMAND
running command...
$ tmpl (--version)
tmpl/0.0.0 linux-x64 node-v16.13.2
$ tmpl --help [COMMAND]
USAGE
  $ tmpl COMMAND
...
```
<!-- usagestop -->
# Commands
<!-- commands -->
* [`tmpl hello PERSON`](#tmpl-hello-person)
* [`tmpl hello world`](#tmpl-hello-world)
* [`tmpl help [COMMAND]`](#tmpl-help-command)
* [`tmpl plugins`](#tmpl-plugins)
* [`tmpl plugins:inspect PLUGIN...`](#tmpl-pluginsinspect-plugin)
* [`tmpl plugins:install PLUGIN...`](#tmpl-pluginsinstall-plugin)
* [`tmpl plugins:link PLUGIN`](#tmpl-pluginslink-plugin)
* [`tmpl plugins:uninstall PLUGIN...`](#tmpl-pluginsuninstall-plugin)
* [`tmpl plugins update`](#tmpl-plugins-update)

## `tmpl hello PERSON`

Say hello

```
USAGE
  $ tmpl hello [PERSON] -f <value>

ARGUMENTS
  PERSON  Person to say hello to

FLAGS
  -f, --from=<value>  (required) Whom is saying hello

DESCRIPTION
  Say hello

EXAMPLES
  $ oex hello friend --from oclif
  hello friend from oclif! (./src/commands/hello/index.ts)
```

_See code: [dist/commands/hello/index.ts](https://github.com/jwpjrdev/tmpl/blob/v0.0.0/dist/commands/hello/index.ts)_

## `tmpl hello world`

Say hello world

```
USAGE
  $ tmpl hello world

DESCRIPTION
  Say hello world

EXAMPLES
  $ oex hello world
  hello world! (./src/commands/hello/world.ts)
```

## `tmpl help [COMMAND]`

Display help for tmpl.

```
USAGE
  $ tmpl help [COMMAND] [-n]

ARGUMENTS
  COMMAND  Command to show help for.

FLAGS
  -n, --nested-commands  Include all nested commands in the output.

DESCRIPTION
  Display help for tmpl.
```

_See code: [@oclif/plugin-help](https://github.com/oclif/plugin-help/blob/v5.1.12/src/commands/help.ts)_

## `tmpl plugins`

List installed plugins.

```
USAGE
  $ tmpl plugins [--core]

FLAGS
  --core  Show core plugins.

DESCRIPTION
  List installed plugins.

EXAMPLES
  $ tmpl plugins
```

_See code: [@oclif/plugin-plugins](https://github.com/oclif/plugin-plugins/blob/v2.0.11/src/commands/plugins/index.ts)_

## `tmpl plugins:inspect PLUGIN...`

Displays installation properties of a plugin.

```
USAGE
  $ tmpl plugins:inspect PLUGIN...

ARGUMENTS
  PLUGIN  [default: .] Plugin to inspect.

FLAGS
  -h, --help     Show CLI help.
  -v, --verbose

DESCRIPTION
  Displays installation properties of a plugin.

EXAMPLES
  $ tmpl plugins:inspect myplugin
```

## `tmpl plugins:install PLUGIN...`

Installs a plugin into the CLI.

```
USAGE
  $ tmpl plugins:install PLUGIN...

ARGUMENTS
  PLUGIN  Plugin to install.

FLAGS
  -f, --force    Run yarn install with force flag.
  -h, --help     Show CLI help.
  -v, --verbose

DESCRIPTION
  Installs a plugin into the CLI.

  Can be installed from npm or a git url.

  Installation of a user-installed plugin will override a core plugin.

  e.g. If you have a core plugin that has a 'hello' command, installing a user-installed plugin with a 'hello' command
  will override the core plugin implementation. This is useful if a user needs to update core plugin functionality in
  the CLI without the need to patch and update the whole CLI.

ALIASES
  $ tmpl plugins add

EXAMPLES
  $ tmpl plugins:install myplugin 

  $ tmpl plugins:install https://github.com/someuser/someplugin

  $ tmpl plugins:install someuser/someplugin
```

## `tmpl plugins:link PLUGIN`

Links a plugin into the CLI for development.

```
USAGE
  $ tmpl plugins:link PLUGIN

ARGUMENTS
  PATH  [default: .] path to plugin

FLAGS
  -h, --help     Show CLI help.
  -v, --verbose

DESCRIPTION
  Links a plugin into the CLI for development.

  Installation of a linked plugin will override a user-installed or core plugin.

  e.g. If you have a user-installed or core plugin that has a 'hello' command, installing a linked plugin with a 'hello'
  command will override the user-installed or core plugin implementation. This is useful for development work.

EXAMPLES
  $ tmpl plugins:link myplugin
```

## `tmpl plugins:uninstall PLUGIN...`

Removes a plugin from the CLI.

```
USAGE
  $ tmpl plugins:uninstall PLUGIN...

ARGUMENTS
  PLUGIN  plugin to uninstall

FLAGS
  -h, --help     Show CLI help.
  -v, --verbose

DESCRIPTION
  Removes a plugin from the CLI.

ALIASES
  $ tmpl plugins unlink
  $ tmpl plugins remove
```

## `tmpl plugins update`

Update installed plugins.

```
USAGE
  $ tmpl plugins update [-h] [-v]

FLAGS
  -h, --help     Show CLI help.
  -v, --verbose

DESCRIPTION
  Update installed plugins.
```
<!-- commandsstop -->
