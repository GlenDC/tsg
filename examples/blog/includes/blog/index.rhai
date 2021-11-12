fn generate(generator) {
    for post in generator.page.metadata["posts"] {
        generator.md(`### [${post["title"]} (${post["date"]})](${post["path"]})

${post["summary"]}
`);
}
