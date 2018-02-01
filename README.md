# xim

[![Build Status](https://travis-ci.org/DestructHub/xim.svg?branch=master)](https://travis-ci.org/DestructHub/xim)

`xim` is a minimal terminal frontend for [xi editor](https://github.com/google/xi-editor).

It is experimental and under development, so don't expect anything magical (yet!).

This is a fork which don't have any intention to merge on the original.

## Installation

Assuming that you have installed [xi editor core](https://github.com/google/xi-editor) and you have `~/.cargo/bin` into your `PATH`, the following should suffice:

```bash
# If you want syntax highlighting, download and install the syntect plugin
git clone https://github.com/google/xi-editor
cd xi-editor/rust/syntect-plugin
make install

# Clone this repository and navigate to the project root folder
git clone https://github.com/DestructHub/xim
cd xim

# If you want to install it on your machine
cargo install

# If you just want to run it without installing
cargo run --release -- <filename>
```

To run the editor after installing, use `xim <filename>`.

## Logging

For debugging, it can be useful to have logs. You can specify a
location for log files `xim` with `-l <logfile>`. Two files will
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
