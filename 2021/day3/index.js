import fs from "fs";
import path from "path";

function getMostLeast(arr, i) {
  let newArr = [];
  arr.forEach(function (el) {
    if (el !== "") newArr.push(el[i]);
  });
  let ones = newArr.filter(function (el) {
    return el === "1";
  });
  let zeros = newArr.filter(function (el) {
    return el === "0";
  });
  return ones.length >= zeros.length ? ["1", "0"] : ["0", "1"];
}

function calculateRating(first, second) {
  return parseInt(first, 2) * parseInt(second, 2);
}

function part1(arr) {
  if (arr.length <= 0) return;
  let gamma = "";
  let epsilon = "";
  for (let i = 0; i < arr[0].length; i++) {
    const [g, e] = getMostLeast(arr, i);
    gamma += g;
    epsilon += e;
  }
  const power = calculateRating(gamma, epsilon);
  console.log(`Part 1: ${power}`);
}

function recursiveFilter(arr, i, option) {
  if (arr.length <= 1 || i === arr[0].length) {
    return arr;
  }
  return recursiveFilter(
    arr.filter(function (el) {
      if (option === "most") {
        return el[i] === getMostLeast(arr, i)[0];
      } else {
        return el[i] === getMostLeast(arr, i)[1];
      }
    }),
    ++i,
    option
  );
}

function part2(arr) {
  if (arr.length <= 0) return;
  let i = 0;
  const oxygen = recursiveFilter(arr, i, "most");
  const co2 = recursiveFilter(arr, i, "least");
  const rating = calculateRating(oxygen, co2);
  console.log(`Part 2: ${rating}`);
}

function parseFile(file) {
  fs.readFile(file, "utf8", function (err, data) {
    if (err) throw err;
    let arr = data.toString().split("\n");
    part1(arr);
    part2(arr);
  });
}

function main() {
  const fileName = process.argv[2];
  const filePath = path.join(process.cwd(), fileName);
  parseFile(filePath);
}

main();
