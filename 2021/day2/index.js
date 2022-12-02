import path from "path";
import fs from "fs";
import readLine from "readline";

var x = 0;
var y = 0;
var aim = 0;

function lineParser(line) {
  const commands = line.split(" ");
  const commandReader = {
    forward: function (number) {
      x += number;
      y += aim * number;
    },
    down: function (number) {
      aim += number;
    },
    up: function (number) {
      aim -= number;
    },
  };
  commandReader[commands[0]](parseInt(commands[1], 10));
  console.log(x * y);
}

function parseFile(file, callback) {
  let rl = readLine.createInterface({
    input: fs.createReadStream(file),
    output: process.stdout,
    terminal: false,
  });
  rl.on("line", callback);
}

function main() {
  const fileName = process.argv[2];
  const filePath = path.join(process.cwd(), fileName);
  parseFile(filePath, lineParser);
}

main();
