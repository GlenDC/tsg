---
title: TSG
---

## Tiny Site Generator

TSG, short for Tiny Site Generator, is an opinionated
bare minimum site generator with localization support for all files.

What does TSG allow you to do:

- generate a static HTML website;
- use markdown files as includes or entire pages;
- localize any file simply by providing the same file with different dot-separated
  localization suffixes to the filename;
- use yaml/toml/json files similar to any other include, to define any kind of data, localized or not;
- use [Rhai scripts][rhai] as includes or entire pages;
- use any static asset directly as-is and within the directory structure you defined it in;

What does TSG not allow you to do:

- add configuration files;
- use scripting-like logic in html file templates;
- define your own root directory structure;
- extend it with any kind of plugins or modules;

So basically it is yet another static site generator. There are so many you might very easily lose count.
And while it might not even be a very unique one, here are the values that the design and implementation
of TSG adheres to:

- keep it simple stupid: it needs to be simple enough that
  the entire scope of the tool can be learned using just one or two pages of documentation; in a page or two;
- no external configuration files: instead configure based on extensions,
  file structure and minimal embedded configuration (metadata);
- work with raw HTML / CSS for layouts: once again, keep it simple;
- support include templating in HTML/Markdown files, but do so using a valid HTML tag;
- keep the TSG Cli tool API to a minimum;
