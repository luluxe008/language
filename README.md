# Language
A small compiler made by me.

I have not done the codegen but I think the compiler will trans-compiler an invented language to C, and then compile it with tinycc or gcc.

Currently, this repo has an Tokenizer, an Abstract Symbol Tree parser, Error handler and a small JIT to test it.

I wrote a lot of test, all of them should pass.

## Usage
```
git clone https://github.com/luluxe008/language.git
cd language
cargo run
```
### Test
```
git clone https://github.com/luluxe008/language.git
cd language
cargo test -- --nocapture
```

# Contributing
Make a PR and I will look at it ;)
Try to comment what you do. 