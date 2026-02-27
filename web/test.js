import fs from 'fs';
const content = fs.readFileSync('src/pages/index.astro', 'utf8');
if (content.includes("\\`")) {
  console.log("BACKSLASH BACKTICK FOUND!");
}
