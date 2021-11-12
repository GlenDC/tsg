---
layout: blog.html
---

const POSTS_PER_PAGE = 5;

fn generate(generator) {
    let sorted_pages = [];
    let index = 0;
    do {
        let next_index = min(index+POSTS_PER_PAGE, sorted_pages.len());
        let pages = #{};
        for page_index in range(0, next_index-index) {
            let page = sorted_pages[index+page_index];
            pages[page.path()] = #{
                "date": page.metadata["date"],
                "title": page.metadata["title"],
                "summary": page.metadata["summary"],
            };
        }
        // TODO: how to pass such extra metadata?
        // TODO: how to pass custom name, ...
        generator.html("blog", pages, index%POSTS_PER_PAGE);
        index = next_index;
    } until (index >= sorted_pages.len());
}