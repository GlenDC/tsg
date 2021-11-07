fn generate(generator, page) {
    // generate an index page for all possible locales
    // and generate one of all possible locales using an accessibility css
    for locale in generator.locales {
        // generate regular locale version
        page.set_param("locales", ["main.css"]);
        // generate page as /<locale>/index.html
        // you can also omit the <locale> parameter and in that case
        // one html per found locale will be generated
        generator.html(page, "index", locale);

        // generate accessibility version
        page.set_param("main.css", "main_accessible.css"]);
        // generate page as /<locale>/accessible/index.html
        generator.html(page, "index", `${locale}.accessible`);
    }
}
