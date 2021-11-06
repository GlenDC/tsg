# Basic Examples

Within this directory you find basic examples demonstrating you the more
basic concepts of tools provided by TSG.

- [/examples/basic/one_page_md](/examples/basic/one_page_md):
  the most basic example, with the source consisting of a single markdown file
- [/examples/basic/one_page_custom_layout](/examples/basic/one_page_custom_layout):
  still a basic example, but using a custom layout, some includes and our first static asset
- [/examples/basic/one_page_custom_layout_l18n](/examples/basic/one_page_custom_layout_l18n):
  a localized version of the previous example, showcasing how you can provide a site in a default
  and alternative version:
    - the default will be available under `/` of your domain, use a redirect in your server config
      should you want for example `/en` to redirect/mirror `/`;
    - note that this feature:
        - is not restricted to locales or languages, the suffixes used can be anything;
        - multiple layers of suffixes can be used to define fallbacks, e.g. `en.us` will first try `en.us` for files,
          `en` if it couldn't be found and the default one otherwise. For parameter files the fallback is also used
          in case the file does exist but a specific value cannot be found within it;
