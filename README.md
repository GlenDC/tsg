<p align="center">
  <img src="docs/images/banner.png" alt="TSG banner image"/>
</p>

> **WARNING**: this tool is still unstable, under active development and incomplete.
> You advised to not use TSG yet in any of your own products for anything else besides experimentation!

## Overview

Tiny Site Generator (_TSG_) is a static site generator written in Rust.
It is optimized for speed, ease of use, and minimal learning curve.

_TSG_ can generate content from HTML files, [Rhai][rhai] scripts,
and Markdown files (optionally with _yaml_ front matter for metadata). Both the HTML and Markdown files
can also be templated using `<include>` tags to _include_ any one your other HTML files, [Rhai][rhai] scripts, Markdown files, _yaml_ files and even _bash_ scripts.
The website may also contain any kind of _assets_, which will be mirrored unmodified with respect of their underlying directory structure.
All files except _assets_ can also be localized simply by using suffixes of your own choices put between the filename and its file format extension.

A typical website of moderate size is rendered by _TSG_ in a fraction of a second.

_TSG_ is designed to work well with any kind of static website including blogs, tumbles and docs.

### Supported architectures

_TSG_ releases get bundled with pre-build binaries for Windows, Linux and macOS (Darwin).
However, you should be able to build to any platform and architecture supported by the Rust (LLVM) toolchain.

## Tiny Site Generator Documentation

### Directory Layout

| path | file formats | description |
|---|---|---|
| `/pages/**` | `html/md/rhai` | The files that map to the actual pages on your website, the HTML/MD files and their relative path map directly to an HTML page, while the Rhai script can generate any amount of pages. |
| `/layouts/**` | `html` | Layouts define the layout of a page, in its entirety or just a content section. Pages have a default layout assumed at `main.html`, any other content which is generated as HTML has no default layout. |
| `/includes/**` | `html/md/yml/rhai/sh` | Files that can be non-cyclic included as part of pages, layouts and other includes. |
| `/assets/**` | `*` | Files that are mirrored over to the publish directory as-is. These are the only files for which no out of the box localization support is provided. |

Feel free to also browse around in the [/examples](/examples) folder,
so you can see yourself how a source tree of a typical website made with TSG looks like. This is also a great way to introduce you to its various aspects and show you how to integrate the frameworks you know (e.g. bootstrap).

### TSG Templating

Layouts, pages and includes all can include other files in its entirety or simply a string value of it.

For HTML and Markdown files this is done as follows:

```html
<include>foo.bar.baz</include>
```

For [Rhai](rhai) scripts this is done as follows:

```rust
let baz = generator.include("foo.bar.baz");
```

For Bash (`*.sh`) scripts this is done as follows:

```bash
baz="$TSG_INCLUDE_FOO_BAR_BAZ"
```

Some examples of what `foo.bar.baz` can refer to:

| filepath | description |
| - | - |
|`includes/foo/bar/baz.*` | Include the output of the render using the `baz.*` file |
|`includes/foo/bar.yml` | Include the root `baz` string property from within the `bar.yml` file |
|`includes/foo.yml` | Include the `baz` string property, a child property of the root object `bar` property, data found within the `foo.yml` file |

The last two examples also work with:

- a [Rhai][rhai] script: this script is expected to have generated `yml` content exclusively;
- a Bash script: expected to have received an object that is `json`-encoded into a string over its STDOUT;
- a Markdown/HTML files: expected to be found within the metadata (Front matter) section of the file;

The first example can work with any valid `includes/*` file.

### Rhai scripting

Please consult "[the Rhai book - Rhai Language Reference](https://rhai.rs/book/language/index.html)" for any [Rhai][rhai] specific questions. In that section of the book you'll find all you need to know about the language and how to use it. Within this chapter we'll go over the API of the user-defined `Rhai` scripts.

TODO...

### Contributing to TSG

TODO...

## Dependencies

TSG stands on the shoulder of many great open source libraries.

If you run `tsg env -v` you will get a complete and up to date list.

In TSG 0.1.0 that list is, in lexical order:

```
TODO...
```

[rhai]: https://rhai.rs/
