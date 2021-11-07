fn generate(generator, page) {
    for stylsheet in page.param("stylesheets") {
        generator.text(`<link rel="stylesheet" href="/assets/${stylesheet}">`);
    }
}
