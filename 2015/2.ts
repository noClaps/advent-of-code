const input = await Bun.file("2.txt").text();
let total = 0;

function surfaceArea(l: number, w: number, h: number) {
  if (!l || !w || !h) return 0;
  const side1 = l * w;
  const side2 = w * h;
  const side3 = h * l;
  return 2 * side1 + 2 * side2 + 2 * side3 + Math.min(side1, side2, side3);
}

function ribbonLength(l: number, w: number, h: number) {
  if (!l || !w || !h) return 0;
  return l * w * h + 2 * (l + w + h - Math.max(l, w, h));
}

input.split("\n").forEach((line, index) => {
  const [l, w, h] = line.split("x").map((char) => +char);
  total += ribbonLength(l, w, h);
});

console.log(total);
