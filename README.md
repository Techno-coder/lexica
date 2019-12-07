# [Lexica](https://technocoder.gitbook.io/lexica/)
A reversible programming language.

```
fn fibonacci(n: u64) -> u64:
	let ~first = 1
	let ~second = 1

	let ~counter = 1
	loop counter == 1 => counter == n:
		let summation = first + second
		first <=> second
		second <=> summation

		drop summation = second - first
		counter += 1

	drop n = counter
	second
```

## Installation
```
cargo +nightly install --locked --git https://github.com/Techno-coder/lexica
```

## Usage
```
lexica <path>/main.lx
```

## Commands
* `context` - Displays the compiler context state
* `basic <reversible|entropic> <function>` - Displays the basic node lowering of a function
* `evaluate <function>` - Evaluates and returns the result of a zero arity function
* `cycle <function>` - Evaluates and reverses and returns the parameter values of a zero arity function

## Influence
Lexica has been influenced by:
- [Rust](https://github.com/rust-lang/rust)
- [Arrow](https://etd.ohiolink.edu/!etd.send_file?accession=oberlin1443226400)
- [Nim](https://nim-lang.org)

and many other languages.
