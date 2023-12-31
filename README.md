# Migi

Migi is a minimal static site generator named after the parasite which lives in Shinichi's right hand in the anime "Parasyte", which in turn was named after the Japanese word for right (ミギー).

For instructions on how to use Migi, please read the short walk-through on the [wiki page](../../wiki).

## Features

- **Caching**: Optimizes your site's performance by storing and reusing content, minimizing build times.
- **Tagging**: Enhances content organization, allowing easy categorization and retrieval of content.
- **Templates**: Flexible templating system adaptable to various content types, from blogs to portfolios.
- **Syntax Highlighting**: Supports `.tmThemes` for customizable code presentation.
- **HTML Minification**: Reduces the size of your HTML files, improving load times and efficiency.

## Installation

Ensure you have Rust and Cargo installed on your system. Then, install Migi with the following command:

```bash
cargo install migi
```

or if cloning from the GitHub, use the provided `makefile`:

```bash
make && sudo make install
```
and `sudo make uninstall` to completely uninstall.

## Quick Start

```bash
migi new mysite
cd mysite
migi build
```
This sequence of commands creates and builds a new static site in the mysite directory.

## Creating Posts

To create a new post, add a markdown file in the content/ directory.

Use the header section in each post for metadata like title and tags. (Refer to the [wiki page](../../wiki)).

## Community and Support

**Issues and Discussions:** For support, feature requests, or bug reports, visit the [GitHub Issues](../../issues).

**Contributions**: Contributions are welcome!

Star the repository or contribute to support my work!
