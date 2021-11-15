let guides = tsg.includes("guides.**").sort(|f| f.meta("order"));
let output = "";
for guide in guides {
    output += `<li><a href="/guides/${guide.meta("name")}>${guide.meta("summary")}</a></li>`;
}
output
