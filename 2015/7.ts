const input = await Bun.file("7.txt").text();
// const input = `
// 123 -> x
// 456 -> y
// x AND y -> d
// x OR y -> e
// x LSHIFT 2 -> f
// y RSHIFT 2 -> g
// NOT x -> h
// NOT y -> i
// `.trim();

let lines = input.split("\n");

function replaceLines(lines: string[], output: string, input: number) {
  if (Number.isNaN(input)) return lines;

  const regex = new RegExp(`(^|\ )${output}\ `);
  return lines
    .map((line) =>
      line.replace(regex, `${line.startsWith(output) ? "" : " "}${input} `),
    )
    .filter((line) => line !== `${input} -> ${output}`);
}

function wireValue(wire: string): number {
  const instruction = lines.find((line) => line.endsWith(`-> ${wire}`));
  if (!instruction) throw new Error(`Instruction not found: ${wire}`);

  const [operation, output] = instruction.split(" -> ");
  const ops = operation.split(" ");

  if (ops.length === 1) {
    lines = replaceLines(lines, output, +ops[0]);
    return !Number.isNaN(+ops[0]) ? +ops[0] : wireValue(ops[0]);
  }

  if (ops[0] === "NOT") {
    lines = replaceLines(lines, output, ~+ops[1]);
    return ~(!Number.isNaN(+ops[1]) ? +ops[1] : wireValue(ops[1]));
  }

  const val1 = !Number.isNaN(+ops[0]) ? +ops[0] : wireValue(ops[0]);
  const val2 = !Number.isNaN(+ops[2]) ? +ops[2] : wireValue(ops[2]);

  switch (ops[1]) {
    case "AND":
      lines = replaceLines(lines, output, val1 & val2);
      return val1 & val2;
    case "OR":
      lines = replaceLines(lines, output, val1 | val2);
      return val1 | val2;
    case "LSHIFT":
      lines = replaceLines(lines, output, val1 << val2);
      return val1 << val2;
    case "RSHIFT":
      lines = replaceLines(lines, output, val1 >> val2);
      return val1 >> val2;
    default:
      return -1;
  }
}

const wire = "a";
const outputA =
  wireValue(wire) < 0 ? wireValue(wire) + 2 ** 16 : wireValue(wire);

lines = input
  .split("\n")
  .map((line) => (line.endsWith("-> b") ? `${outputA} -> b` : line));

const output =
  wireValue(wire) < 0 ? wireValue(wire) + 2 ** 16 : wireValue(wire);
console.log(output);
