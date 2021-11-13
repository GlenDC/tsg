<p align="center">
  <img src="docs/images/banner.png" alt="TSG banner image"/>
</p>

> **WARNING**: this tool is still unstable, under active development and incomplete.
> You advised to not use TSG yet in any of your own products for anything else besides experimentation!

## 1. Overview

Tiny Site Generator (_TSG_) is a static site generator written in Rust.
It is optimized for speed, ease of use, and minimal learning curve.

_TSG_ can generate content from HTML files, [Rhai][rhai] scripts,
and Markdown files (optionally with _yaml_ front matter for metadata). Both the HTML and Markdown files
can also be templated using `<include>` tags to _include_ any one your other HTML files, [Rhai][rhai] scripts, Markdown files, _yaml_ files and even _bash_ scripts.
The website may also contain any kind of _assets_, which will be mirrored unmodified with respect of their underlying directory structure.
All files except _assets_ can also be localized simply by using suffixes of your own choices put between the filename and its file format extension.

A typical website of moderate size is rendered by _TSG_ in a fraction of a second.

_TSG_ is designed to work well with any kind of static website including blogs, tumbles and docs.

### 1.A. Supported architectures

_TSG_ releases get bundled with pre-build binaries for Windows, Linux and macOS (Darwin).
However, you should be able to build to any platform and architecture supported by the Rust (LLVM) toolchain.

## 2. Tiny Site Generator Documentation

### 2.A. Directory Layout

| path | file formats | description |
|---|---|---|
| `/pages/**` | `html/md/rhai` | The files that map to the actual pages on your website, the HTML/Markdown files and their relative path map directly to an HTML page, while the Rhai script can generate any amount of pages. |
| `/layouts/**` | `html` | Layouts define the layout of a page, in its entirety or just a content section. Pages have a default layout assumed at `main.html`, any other content which is generated as HTML has no default layout. |
| `/includes/**` | `html/md/yml/rhai/sh` | Files that can be non-cyclic included as part of pages, layouts and other includes. |
| `/assets/**` | `*` | Files that are mirrored over to the publish directory as-is. These are the only files for which no out of the box localization support is provided. |

Feel free to also browse around in the [/examples](/examples) folder,
so you can see yourself how a source tree of a typical website made with TSG looks like. This is also a great way to introduce you to its various aspects and show you how to integrate the frameworks you know (e.g. bootstrap).

### 2.B. TSG Templating

#### 2.B.I. Includes

Layouts, pages and includes all can include other files in its entirety or simply a string value of it.

For HTML and Markdown files this is done as follows:

```html
<include>foo.bar.baz</include>
```

For [Rhai](rhai) scripts this is done as follows:

```rust
let baz = tsg.include("foo.bar.baz");
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

#### 2.B.II. Metadata

You can also include strings from within the current metadata. The most common metadata is the one defined as Front matter of a page (available for both HTML and Markdown).
Let's dive into some examples.

Including string values in HTML and Markdown files from the metadata is done almost
the same way was other includes, with the only distinctive defines that
the root property is `$`:

```html
<include>$.title</include>
```

For [Rhai][rhai] scripts it is done using:

```rust
let title = tsg.meta["title"];
```

And from within bash scripts this metadata data is accessed using:

```bash
title="$TSG_META_TITLE"
```

#### 2.B.III. Content

A Layout file can also `include` content. The special metadata
property `$` can be used here is well as part of your `include` statements.
Layouts also need to define where the content needs to be placed.
This content is either the rendered result of the page it lays our
or the include that configured it as a Layout.

In order to reference the output/content of the file you use the `$%` special variable. For example:

```html
<main>
  <include>$%</include>
</main>
```

#### 2.B.IV. Front Matter

Front matter data is the optional _yaml_ formatted data you can put at the start
of any HTML and/or Markdown file. As asset files aren't processed by _TSG_
you should probably not add Front Matter for these files. Here is an example
of a blog post file with Front Matter data:

```yaml
---
title: My Blog Post
intro: A blog post to showcase front matter data defined in a file.
date: 2021-11-10 18:30:00
draft: false
---

<include>$.intro</include>

Hello, glad that you're here!
Let's talk about front matter data.
```

> NOTE: that there is nothing special about the kind of metadata used in the above example.
> _TSG_ processes them as raw _yaml_ without giving any special meaning or value to any of
> these individual properties. It is how you use and interpret the metadata defined by yourself
> that give them the meaning and value you seek.

The above works for HTML files as well. This metadata can be accessed as follows:

- within the files itself using the special `$` root include property;
- by any of the layout files used for a page;
- by the [Rhai][rhai] script including a Markdown/Html file;

[Rhai][rhai] scripts can also define metadata when returning
a render object rather than a primitive.

Also note that metadata can be shadowed.
If there are metadata properties defined in multiple layers (e.g. layout, page and include),
the value will be used defined in the most inner layer. Best is to to keep your metadata
to a minimal and unique, and you will not have to worry about it at all. You'll be fine.

### 2.C. Rhai scripting

Please consult "[the Rhai book - Rhai Language Reference](https://rhai.rs/book/language/index.html)" for any [Rhai][rhai] specific questions. In that section of the book you'll find all you need to know about the language and how to use it. Within this chapter we'll go over the API of the user-defined `Rhai` scripts.

#### 2.C.I. API

All exposed _TSG_ functionality can be found as properties and
methods of the already in scope `tsg` object:

| property | description |
| - | - |
| `tsg.includes(path:str="", recursive:bool=False) -> [File]` | Return a list of `File` found on the given path, listed recursive if desired |
| `tsg.include(path:str) -> Dynamic` | Return an `File` in case the given path points to an include file, but it can also return a primitive value in case the path extends beyond the filepath to extract some of the metadata within. |
| `tsg.layouts(path:str="", recursive:bool=False) -> [File]` | Return a list of `File` found on the given path, listed recursive if desired |
| `tsg.layout(path:str) -> File` | Return a `File` found on the given path. |
| `tsg.pages(path:str="", recursive:bool=False) -> [File]` | Return a list of `File` found on the given path, listed recursive if desired. |
| `tsg.page(path:str="") -> File` | Return a `File` found on the given path or for the current `Page`. |
| `tsg.metadata(path:str) -> Dynamic` | Return a Primitive value of the metadata on the given path (any valid _yaml_ value). |

The `File` type is an _object mapping_ with the following properties:

| property | description |
| - | - |
| `file.meta` | _object mapping_ value containing the metadata of the File |
| `file.content` | _str_ value containing the raw content section of the File |
| `file.path` | _str_  value containing the relative path of the File (dot notation) |
| `file.locale` | _str_ value containing the Locale of the File |
| `file.type` | _str_ value containing the File extension |

A [Rhai][rhai] script is run as a function, and thus it is expected that the last line of the
script returns the value to be rendered. This can be done implicitly without the use of the `return` keyword. The return value can be one of the following:

- A _primitive_ value: value will be rendered as a _string_;
- A _File_ value: value will be rendered using the regular _TSG_ pipeline into an `html` string;
- A _list of primitive values_ and/or _Files_: for each the logic of the previous two lines is used;

#### 2.C.II. Rhai Scripts as Modules

[Rhai][rhai] scripts can also import other [Rhai][rhai] scripts within your codebase,
this allows you to define reusable logic. Our advice is to keep your generation logic
as simple as possible, so do please be careful in not going overboard in
your usage of [Rhai][rhai] scripts, for your own sanity and those around you.

Please refer to <https://rhai.rs/book/language/modules/index.html> to learn
how to import and export modules. They are not a requirement to get started with _TSG_,
but its a feature that is available for those that feel the need for it.

TODO: confirm how/if it works in the _TSG_ setup and document extra where needed!!!!

### 2.D. Bash scripting

Keep the scripting to a minimum. Use [Rhai][rhai] to write your scripts by default,
and only use Bash scripts in the context of _TSG_ when you really need to.

In case you need to interact with your host system or rely for whatever
reasons on the UNIX tools it is probably a good enough reason to use Bash scripts.
As always though, keep it simple and to a minimum.

> (!) Bash scripts can _only_ include primitive values, trying to include entire files
will result in a generator error.

Everything printed to the STDOUT will be used as the generated content.

### 2.E. TSG Cli Help

TODO ...

### 2.F. Contributing to TSG

TODO...

## 3. Dependencies

TSG stands on the shoulder of many great open source libraries.

If you run `tsg env -v` you will get a complete and up to date list.

In TSG 0.1.0 that list is, in lexical order:

```
TODO...
```

[rhai]: https://rhai.rs/
