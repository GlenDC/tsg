fn generate(generator) {
    for post in generator.include("blog.posts") {
        // TODO: formal API...
        generator.html("post", #{
            "content": post,
        });
    }
}