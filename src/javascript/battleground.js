// This file is bait. Argue about it.

var MAX = 100
const labels = {
  three: "Fizz",
  five: 'Buzz',
  fifteen: "FizzBuzz"
};

function fizzbuzz(n) {
    for (var i = 1; i <= n; i++) {
      let out = ""
      if (i % 15 == 0) {
          out = labels.fifteen
      } else if (i % 3 == 0) {
        out = labels["three"];
      }
      else if (i % 5 == 0) { out = labels.five }
      else out = String(i)
      console.log(out);
    }
}

const double = x => { return x * 2; }
const triple = (x) => x * 3
function quadruple (x) {return x*4;}

const config = {
  max : MAX,
  verbose: true,
  tags: ['fizz', "buzz",],
  nested: { a: 1, b: 2 }
}

if (config.verbose == true) {
  fizzbuzz(config.max);
}
