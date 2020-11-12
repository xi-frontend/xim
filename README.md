# xim

[![Build Status](https://travis-ci.org/xi-frontend/xim.svg?branch=master)](https://travis-ci.org/xi-frontend/xim)

![the xim source](.github/xim.png)

`xim` is a minimal terminal frontend for [xi editor](https://github.com/google/xi-editor).

It is experimental and under development, so don't expect anything magical (yet!).

This is a fork which don't have any intention to merge on the original.

## Installation

Assuming that you have installed [xi editor core](https://github.com/google/xi-editor) and you have `~/.cargo/bin` into your `PATH`, the following should suffice:

```bash
# syntax highlight plugin
git clone https://github.com/google/xi-editor                                                                                       
cd xi-editor/rust/syntect-plugin                                                                                                      
make install                                                                                                                          
cd $OLDPWD                                                                                                                            

# xim install
git clone https://github.com/xi-frontend/xim                                                                                          
cd xim                                                                          
cargo install  
```

To run the editor after installing, use `xim <filename>`.

## Logging

To enable logging, use the parameter `-l <logfile>`. Two files will be written:

- `<logfile>`: `xim` log
- `<logfile>.rpc`: `rpc` messages log


## Shortcuts

- `^s` Save
- `^q` Quit

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

- You must have true colors enabled. Otherwise, some portions of text
  won't be displayed.
- The default color scheme (for now) is `base16-eighties.dark`.
