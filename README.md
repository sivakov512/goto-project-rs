goto-project
============
Easy and fast project switching in your shell!

This is a like `workon` for python, but more powerfull and not only for python.

[![Build Status](https://github.com/sivakov512/goto-project-rs/workflows/test/badge.svg)](https://github.com/sivakov512/goto-project-rs/actions?query=workflow%3Atest)
[![Crates.io](https://img.shields.io/crates/v/goto_project.svg)](https://crates.io/crates/goto_project/)


Configuration and usage
---
Specify your projects in `~/.goto-project.yaml` file.

``` yaml
goto-project:  # project name
  path: ~/Devel/Projects/goto-project/  # where to cd to open project
  instructions:  # any instructions to run on project opening
    - source ~/Devel/Envs/py3_goto-project/bin/activate
    - export PATH="$HOME/Devel/Projects/goto-project/src/target/debug:$PATH"
```

* List all available projects

``` shell
gt
```

* Open project `goto-project`

``` shell
gt goto-project
```

* List subdirs of `goto-projects`

``` shell
gt goto-project --list-subdirs
```

* List subdirs of project's subdir

``` shell
gt goto-project src --list-subdirs
```

* Open project within subdir

``` shell
gt goto-project src
```

To close project press `C-D`, this will roll back all environment changes. In the example above, virtual environment will be "deactivated" and `PATH` will be restored.


Screencast
---
[![asciicast](https://asciinema.org/a/eWzv0cl5P2FhafqkjNETQ5ZoT.svg)](https://asciinema.org/a/eWzv0cl5P2FhafqkjNETQ5ZoT)
