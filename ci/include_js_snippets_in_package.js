const fs = require("fs");

const snippetFolderContents = fs.readdirSync("./pkg/snippets");

const packageHash = snippetFolderContents.pop();

let packageJson = JSON.parse(fs.readFileSync("./pkg/package.json", {encoding: "utf-8"}));

packageJson.files.push(`/snippets/${packageHash}/js/create_directory_mapping.js`);
packageJson.files.push(`/snippets/${packageHash}/js/get_system_time.js`);

fs.writeFileSync("./pkg/package.json", JSON.stringify(packageJson));