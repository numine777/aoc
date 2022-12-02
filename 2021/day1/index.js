import path from "path";
import fs from "fs";
import readLine from "readline";

function main() {
  const fileName = process.argv[2];
  const filePath = path.join(process.cwd(), fileName);
  let increases = 0;
  let cache = [];
  let rl = readLine.createInterface({
    input: fs.createReadStream(filePath),
    output: process.stdout,
    terminal: false,
  });
  rl.on("line", function (line) {
    let incoming = parseInt(line, 10);
    if (cache.length < 3) {
      cache.push(incoming);
    } else {
      const outgoing = cache[0];
      cache.shift();
      cache.push(incoming);
      if (outgoing < incoming) {
        increases++;
      }
    }
    console.log(increases);
  });
}

main();
