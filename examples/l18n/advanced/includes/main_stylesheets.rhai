let output = "";
for stylesheet in tsg.meta("stylesheets") {
    output += `<link rel="stylesheet" href="/assets/${stylesheet}">`;
}
output
