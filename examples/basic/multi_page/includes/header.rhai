// include scripts do not need to worry about different locales,
// this is defind by the page, and if desired the locale can be accessed using page.locale
fn generate(generator) {
    generator.text("<ul>");
    for page in generator.pages {
        if generator.page.contains("parent") {
            continue;
        }
        generator.text(`<li><a href="${generator.page.path}">${generator.page.param("title")}</a></li>`);
    }
    generator.text("</ul>");
}
