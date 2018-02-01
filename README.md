# xim

[![Build Status](https://travis-ci.org/DestructHub/xim.svg?branch=master)](https://travis-ci.org/DestructHub/xim)

`xim` is a terminal frontend for [xi editor](https://github.com/google/xi-editor/).

It is experimental and under development, so don't expect anything magical (yet!).

This is a fork which don't have any intention to merge on the original.

## Installation

The frontend assumes that you have installed the [core editor](https://github.com/google/xi-editor) and is available in your PATH. The following should suffice:

```bash
# If you want syntax highlighting, you need to download and install the syntect plugin
git clone https://github.com/google/xi-editor
cd xi-editor/rust/syntect-plugin
cargo install

# You need to have ~/.cargo/bin into your PATH in order to run any Cargo installation binary
# In case you don´t have it, add `export PATH=$PATH:~/.cargo/bin` to your .bashrc (or equivalent)

# To install xim, just clone the repository and install it by Cargo
git clone https://github.com/DestructHub/xim
cd xim
cargo install
```

You can then run the editor using `xim <filename>` on terminal, or, if you want to run directly from sourcecode, use `cargo run --release -- <filename>`.

## Logging

For debugging, it can be useful to have logs.  You can specify a
location for log files `xim` with `-l <logfile>`.  Two files will
be written:

- `<logfile>`: all the `xim` logs
- `<logfile>.rpc`: the RPC messages exchanged between the core and the
  frontend

## Screenshots

![a python file](.github/python.png)

![the README file](.github/README.png)

## Shortcuts

For now, there are only two shortcuts:

- `^w` saves the current view
- `^c` exits

## Caveats

### Tabs

We assume tabs (`\t`) are 4 columns large. It that is not the case in
your terminal, the cursor position will be inaccurate. On linux, to
set the `\t` width to four spaces, do:

```
tabs -4
```

### Colors

If you have the `syntect` plugin installed, colors will be enabled by
default, with two caveats:

- you must have true colors enabled. Otherwise, some portions of text
  won't be displayed
- the default theme is for dark backgrounds
