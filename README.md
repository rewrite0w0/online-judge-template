# online-judge-template

- [Haskell](#haskell)
- [Javascript](#javascript)
- [Python3](#python3)
- [Ruby](#ruby)
- [Rust](#rust)

### JavaScript

```js
process.stdin.resume();
process.stdin.setEncoding('utf8');

var lines = [];
var reader = require('readline').createInterface({
  input: process.stdin,
  output: process.stdout,
});
reader.on('line', (line) => {
  lines.push(line);
});
reader.on('close', () => {
  console.log(lines[0]);
});

// https://paiza.jp/guide/samplecode.html
```

```js
Main(require('fs').readFileSync('/dev/stdin', 'utf8'));

function Main(input) {
  console.log(input);
}

// atcoder.jp
```

```js
var fs = require('fs');
var input = fs.readFileSync('/dev/stdin').toString().split(' ');
var a = parseInt(input[0]);
var b = parseInt(input[1]);
console.log(a + b);

// https://www.acmicpc.net/
```

### Haskell

```hs
main :: IO ()
main = do
    input_line <- getLine
    print(input_line)

-- https://github.com/dowdiness/atcoder-haskell/blob/master/src/Template.hs

-- https://www.haskell.org/tutorial/io.html
-- https://wiki.haskell.org/Introduction_to_Haskell_IO/Actions
```

### Python3

```py
input_line = input()
print(input_line)

# https://paiza.jp/guide/samplecode.html
```

### Ruby

```rb
input_line = gets
puts input_line

# https://paiza.jp/guide/samplecode.html

```

### Rust

```rs
use std::io;
fn main(){
    let mut lines = String::new();
    io::stdin().read_line(&mut lines).ok();
    println!("{}", lines);
}

// https://github.com/Shota-Kudo/rust_learning_by_paiza
```

```rs
use std::io;

fn main() {
let mut s = String::new();

io::stdin().read_line(&mut s).unwrap();

let values:Vec<i32> = s
    .as_mut_str()
    .split_whitespace()
    .map(|s| s.parse().unwrap())
    .collect();

println!("{}", values[0] + values[1]);

}

// https://www.acmicpc.net/
```
