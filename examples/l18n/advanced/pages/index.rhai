fn generate(generator) {
    // generate an index page for all possible locales
    // and generate one of all possible locales using an accessibility css
    for locale in generator.locales {
        // generate regular locale version
        generator.page.set_param("locales", ["main.css"]);
        // generate page as /<locale>/index.html
        // you can also omit the <locale> parameter and in that case
        // one html per found locale will be generated
        generator.html(generator.page, "index", locale);

        // generate accessibility version
        generator.page.set_param("main.css", "main_accessible.css"]);
        // generate page as /<locale>/accessible/index.html
        generator.html(generator.page, "index", `${locale}.accessible`);
    }
}
