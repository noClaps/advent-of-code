const input = "ckczppom";
let index = 1;
let hasher = new Bun.CryptoHasher("md5").update(`${input}${index}`);

while (!hasher.digest("hex").toString().startsWith("000000")) {
  hasher = new Bun.CryptoHasher("md5").update(`${input}${index}`);
  index++;
}

console.log(index - 1);
