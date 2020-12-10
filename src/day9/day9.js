const fs = require('fs');

const fileName = process.argv[2];
const preamble = process.argv[3];

function part2(order, n) {
  for (let i = 0; i < order.length; i++) {
    let sum = order[i];

    let j = i + 1;
    while (sum < n) {
      sum += order[j];
      j += 1;
    }

    if (sum === n) {
      const min = Math.min(...order.slice(i, j));
      const max = Math.max(...order.slice(i, j));
      return min + max;
    }
  }
  return false;
}

try {
  const solution = fs
    .readFileSync(fileName)
    .toString()
    .split('\n')
    .map((e) => parseInt(e, 10))
    .reduce((accArray, cur, i) => {
      const acc = accArray[0];
      const order = accArray[1];

      let exists = false;
      Object.keys(acc).forEach((key) => {
        if (acc[key].has(cur)) exists = true;
        const val = parseInt(key, 10) + cur;
        acc[key].add(val);
      });

      if (i >= preamble && !exists) {
        console.log(part2(order, cur));
        throw Error(`We found our index! ${cur}`);
      }
      if (i >= preamble) {
        delete acc[order[order.length - preamble]];
      }
      if (!Object.keys(acc).includes(cur)) {
        acc[cur] = new Set();
        order.push(cur);
      }
      return [acc, order];
    }, [{}, []]);
} catch (e) {
  console.log(e.message);
}
