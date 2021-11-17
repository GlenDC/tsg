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
let baz = tsg.includes("foo.bar.baz");
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
let title = tsg.meta("title");
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

```rust
// Include a File, List of Files or primitive metadata value depending on the path.
let file = tsg.includes("foo.bar.baz");         // a file for the found include
let title = tsg.includes("foo.bar.baz.title");  // metadata within the found include
let files = tsg.includes("foo.bar.*");          // all files in includes/foo/bar directory
let more_files = tsg.includes("foo.bar.**");    // all files in includes/foo/bar directory, recursive

// Return a File or List of Files for the given path, one File per page.
let file = tsg.pages();                      // file for current page
let file = tsg.pages("foo.bar.baz");         // a file for the found page
let title = tsg.pages("foo.bar.baz.title");  // metadata within the found page
let files = tsg.pages("foo.bar.*");          // all files in pages/foo/bar directory
let more_files = tsg.pages("foo.bar.**");    // all files in pages/foo/bar directory, recursive

// Return the most specific metadata property from the parent which included it.
let title = tsg.meta("title");  // return "title" metadata property of the foo
```

The `File` type is an _object mapping_ with the following properties:

| property | description |
| - | - |
| `file.meta(path: str) -> Dynamic` | getter function to access the metadata of the File |
| `file.set_meta(path: str, value: Dynamic)` | setter function to modify the metadata of the File (in-memory) copy, doesn't change the actual File |
| `file.content` | _str_ value containing the raw content section of the File |
| `file.path` | _str_  value containing the absolute path of the File |
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

## 4. FAQ

> Can I use _TSG_ to generate a website developed using React/Angular/Vue/Ember/...

Yes, but you shouldn't. _TSG_ is really meant for static websites, and while you can use Javascript
and thus also any Javascript framework you want, it is not recommended. These frameworks are more meant
for full-fledged Javascript-driven websites and web applications, a very different kind of website than
the ones _TSG_ is designed for.

> Can I use Bootstrap or some kind of similar framework for my Website generated using _TSG_?

Yes, definitely you can. Given you define your own HTML files with _TSG_ and the fact that Bootstrap is just a bunch of CSS
and optionally some JS, you can include it as any other _asset_. Check also out [the Bootstrap example](./examples/bootstrap) to see
how this plays out in practice.

That being said. There is a certain clarity and beauty in being able to create your website using a minimum of hand crafted HTML and CSS.
Try to do so if you can and want. This is also the road the authors of _TSG_ have gone for for their own websites. Learn tables or flexbox,
whatever works for your layouts. Keep it simple and clean. <https://web.dev/patterns/layout/> is an example of some layouts
using modern CSS. However their solutions don't explain the reasoning behind. On top of that their solutions are each just a possible solution of many.

> Can I use (CSS/Layout) templates for my _TSG_ websites?

No, at least not if you talk about templates that you can share with others or use yourself.
It is also not something that is planned to be supported. You can definitely look at templates made for
other site generators as inspiration, but in the end it is expected of you to make your own designs.

> Can I use Javascript for my website built using _TSG_?

Yes, most definitely. Your asset files can be anything you want and are directly included in your (generated) HTML files.
Personally we avoid Javascript if we can, and only use it for the interactive features where we really need it.
A comment section could be a good use of Javascript, while for example a scrolling feature most likely isn't that great of a feature.
Opinions differ however and so you are free to use where and how much Javascript you use.

> Can you add X to make Y more easier?

It depends. If it extends the API or takes the design in a different direction than the answer is most likely no.
If it is about adding a feature than the answer is probably still no. The goal of _TSG_ is to keep the feature
and its API to a bare minimum as to make it as easy as possible to learn. While at the same time empower you to build
whatever you want. If you believe it fits within _TSG_'s design philosophy feel free to open a Feature Request
and we can take it from there.

> Why can I not use my favorite Template Language for my HTML/Markdown files?

_TSG_ is opinionated with the feature surface kept to a bare minimum. Use [Rhai][rhai] scripts
for the purposes where you would use Template Languages for any kind of special generation.
And include primitive values just the same as you would using your Template Language, except
in a slightly different syntax. With _TSG_ your HTML files remain HTML files, and Markdown files
remain Markdown files.

> Why should I use _TSG_ instead of Jekyll, Hugo, or any other amazing generator project out there?

A good question, please do tell me your take on it. For us the reasons are that their feature set has
grown to such a size that it takes more time than we wish to spend on it to fully learn it. At the same time,
for anything more advanced than the normal use you are either limited to the features they offer or you
have to find a very weird to work around its limitations. These frameworks also really push you almost
in using an existing theme or develop an entire theme yourself, which is yet another thing you'll have to learn.
For non web developers like ourselves this is a lot to ask, as what we really just want is a bare minimum website
for which we fully understood how it is build at the HTML, CSS and if required JS level.

[rhai]: https://rhai.rs/
