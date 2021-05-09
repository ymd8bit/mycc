# Prerequisite

## Docker container install 
```shell
docker build -t mycc:latest .
```

## Docker container run
```shell
docker run -it --rm -v `pwd`:/workspace -t mycc:latest /bin/bash
```

# Usage

## Run with source file
```shell
cargo run -- resource/expr.txt
```

The source file may contain simple expression like below.
```
>> cat resource/expr.txt
-5 + (4 - 20) * 4
```

you might be able to see
```shell
...
    Finished dev [unoptimized + debuginfo] target(s) in 6.07s
     Running `target/debug/mycc resource/expr.txt`
source file path: resource/expr.txt
Some(BinaryOp { op: Add, lhs: UnaryOp { op: Minus, rhs: Number(5) }, rhs: BinaryOp { op: Mul, lhs: BinaryOp { op: Sub, lhs: Number(4), rhs: Number(20) }, rhs: Number(4) } })
```

## Run tests
```shell
cargo test
```

you might be able to see

```shell
...
running 2 tests
test lexer::test_lexer ... ok
test parser::test_parser ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```