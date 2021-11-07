// generate an index page for all possible locales
// and generate one of all possible locales using an accessibility css
for locale in page.locales {
    // generate regular locale version
    page.stylesheets = ["main.css"]
    // generate page as /<locale>/index.html
    generate(page, locale)

    // generate accessibility version
    page.stylesheets = ["main.css", "main_accessible.css"]
    // generate page as /<locale>/accessible/index.html
    generate(page, `${locale}.accessible`)
}
