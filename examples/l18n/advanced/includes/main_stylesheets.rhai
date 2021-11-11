fn generate(generator) {
    for stylsheet in generator.page.param("stylesheets") {
        generator.text(`<link rel="stylesheet" href="/assets/${stylesheet}">`);
    }
}
