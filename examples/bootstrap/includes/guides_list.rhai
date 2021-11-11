fn generate(generator) {
    guides = #{};
    for resource in generator.include("guides.*")) {
        guides[resource.param("order")] = resource;
    }
    for index in range(0, guides.len()) {
        let resource = guides[index];
        generator.text(`<li><a href="/guides/${resource.name()}>${resource.param("summary")}</a></li>`);
    }
}