# online-judge-template

- [Haskell](#haskell)
- [Javascript](#javascript)
- [Python](#python)
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

// 출처 : https://paiza.jp/guide/samplecode.html
```

### Haskell

```hs
main :: IO ()
main = do
    input_line <- getLine
    print(input_line)

-- 출처 : https://github.com/dowdiness/atcoder-haskell/blob/master/src/Template.hs

-- https://www.haskell.org/tutorial/io.html
-- https://wiki.haskell.org/Introduction_to_Haskell_IO/Actions
```

### Python

```py
input_line = input()
print(input_line)

# 출처 : https://paiza.jp/guide/samplecode.html
```

### Ruby

```rb
input_line = gets
puts input_line

# 출처 : https://paiza.jp/guide/samplecode.html

```

### Rust

```rs
use std::io;
fn main(){
    let mut lines = String::new();
    io::stdin().read_line(&mut lines).ok();
    println!("{}", lines);
}

// 출처 : https://github.com/Shota-Kudo/rust_learning_by_paiza
```
