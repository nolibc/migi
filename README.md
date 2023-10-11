# Migi

A very minimal static site generator.

Migi is named after the parasite which lives in Shinichi's right hand in the anime parasyte, which in turn was named after the Japanese word for right (ミギー).

For instructions on how to use Migi, please read the short walk-through on the [wiki page](../../wiki).


## Features

- Caching System
- Tagging System
- Templates
- Syntax Highlighting
- HTML Minification

## Build

Migi is currently not available via package managers.

Instead, clone this repo and use the provided `Makefile`:

```bash
$ make build && sudo make install
```

To completely uninstall Migi:

```bash
$ sudo make uninstall
```
