// include scripts do not need to worry about different locales,
// this is defind by the page, and if desired the locale can be accessed using page.locale

let output = "<ul>";
for page in tsg.pages("*") {
    output += `<li><a href="${page.path}">${page.meta("title")}</a></li>`;
}
output + "</ul>"

