let floor = 0;
let pos = 0;
const input = await Bun.file("1.txt").text();

for (const i in input.split("")) {
  switch (input[i]) {
    case "(":
      floor++;
      break;
    case ")":
      floor--;
      break;
    default:
      break;
  }

  if (floor === -1) {
    pos = +i;
    break;
  }
}

console.log(pos + 1);
