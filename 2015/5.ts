const input = await Bun.file("5.txt").text();

function isNiceString(str: string) {
  let repeatCheck = false;
  let middleCheck = false;
  for (let i = 0; i < str.length; i++) {
    const lastIndex = str.lastIndexOf(str[i] + str[i + 1]);
    if (lastIndex !== i && lastIndex !== i + 1 && lastIndex !== -1) {
      repeatCheck = true;
    }
    if (str[i] === str[i + 2]) {
      middleCheck = true;
    }
  }

  return repeatCheck && middleCheck;
}

let niceCount = 0;

for (const str of input.split("\n")) {
  if (isNiceString(str)) {
    niceCount++;
  }
}

console.log(niceCount);
