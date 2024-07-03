const input = await Bun.file("6.txt").text();

const lights: number[][] = Array.from({ length: 1000 }, () =>
  new Array(1000).fill(0),
);

for (const instruction of input.split("\n")) {
  const tokens = instruction.split(" ");
  const [xCoord1, yCoord1] = tokens[tokens[0] === "toggle" ? 1 : 2]
    .split(",")
    .map((n) => +n);
  const [xCoord2, yCoord2] = tokens[tokens[0] === "toggle" ? 3 : 4]
    .split(",")
    .map((n) => +n);

  if (tokens[0] === "toggle") {
    for (let row = yCoord1; row <= yCoord2; row++) {
      for (let col = xCoord1; col <= xCoord2; col++) {
        lights[row][col] += 2;
      }
    }
  } else {
    if (tokens[1] === "on") {
      for (let row = yCoord1; row <= yCoord2; row++) {
        for (let col = xCoord1; col <= xCoord2; col++) {
          lights[row][col] += 1;
        }
      }
    } else {
      for (let row = yCoord1; row <= yCoord2; row++) {
        for (let col = xCoord1; col <= xCoord2; col++) {
          lights[row][col] -= lights[row][col] > 0 ? 1 : 0;
        }
      }
    }
  }
}

let onLights = 0;
for (const row of lights) {
  for (const light of row) {
    onLights += light;
  }
}

console.log(onLights);
