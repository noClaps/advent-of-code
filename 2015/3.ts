const input = await Bun.file("3.txt").text();

const coordinatesSanta = {
  x: 0,
  y: 0,
};

const coordinatesRobot = {
  x: 0,
  y: 0,
};

const visitedCoords = [
  { house: [coordinatesSanta.x, coordinatesSanta.y], count: 1 },
];

input.split("").forEach((char, i) => {
  switch (char) {
    case "^":
      if (i % 2 === 0) {
        coordinatesSanta.y++;
      } else {
        coordinatesRobot.y++;
      }
      break;
    case ">":
      if (i % 2 === 0) {
        coordinatesSanta.x++;
      } else {
        coordinatesRobot.x++;
      }
      break;
    case "<":
      if (i % 2 === 0) {
        coordinatesSanta.x--;
      } else {
        coordinatesRobot.x--;
      }
      break;
    case "v":
      if (i % 2 === 0) {
        coordinatesSanta.y--;
      } else {
        coordinatesRobot.y--;
      }
      break;
    default:
      break;
  }

  if (i % 2 === 0) {
    const visitedIndexSanta = visitedCoords.findIndex((coord) =>
      Bun.deepEquals(coord.house, [coordinatesSanta.x, coordinatesSanta.y]),
    );

    if (visitedIndexSanta === -1) {
      visitedCoords.push({
        house: [coordinatesSanta.x, coordinatesSanta.y],
        count: 1,
      });
    } else {
      visitedCoords[visitedIndexSanta].count++;
    }
  } else {
    const visitedIndexRobot = visitedCoords.findIndex((coord) =>
      Bun.deepEquals(coord.house, [coordinatesRobot.x, coordinatesRobot.y]),
    );

    if (visitedIndexRobot === -1) {
      visitedCoords.push({
        house: [coordinatesRobot.x, coordinatesRobot.y],
        count: 1,
      });
    } else {
      visitedCoords[visitedIndexRobot].count++;
    }
  }
});

let totalHouses = 0;

for (const coords of visitedCoords) {
  if (coords.count >= 1) {
    totalHouses++;
  }
}

console.log(totalHouses);
