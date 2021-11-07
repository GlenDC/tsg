// include scripts do not need to worry about different locales,
// this is defind by the page, and if desired the locale can be accessed using page.locale
fn generate(generator, page) {
    generator.text("<ul>");
    for page in generator.pages {
        if page.contains("parent") {
            continue;
        }
        generator.text(`<li><a href="${page.path}">${page.param("title")}</a></li>`);
    }
    generator.text("</ul>");
}
