const input = await Bun.file("8.txt").text();

let codeLen = 0;
let memoryLen = 0;
for (const line of input.split("\n")) {
  const parsed = JSON.stringify(line);
  console.log(parsed);
  codeLen += line.length;
  memoryLen += parsed.length;
}

console.log(memoryLen - codeLen);
