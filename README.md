# Lexica
A reversible programming language.

```lx
fn fibonacci(n: u64) -> u64 {
	let ~first = 1;
	let ~second = 1;

	let ~counter = 1;
	loop counter == 1 => counter == n {
		let summation = first + second;
		first <=> second;
		second <=> summation;

		// `summation` contains the original `first`
		drop summation = second - first;
		counter += 1;
	}

	// Implicit drop of `first` and `counter`
	drop n = counter;
	second
}
```

## Execution
Lexica source files can be executed with:
```
cargo run -- <source file>
```
Example source files can be found in the `examples` directory.

## Flags
- `-b` - Emits the translated bytecode
- `-u` - Reverses the context after execution has halted

## Influence
Lexica has been influenced by:
- [Rust](https://github.com/rust-lang/rust)
- [Arrow](https://etd.ohiolink.edu/!etd.send_file?accession=oberlin1443226400)

and many other languages.
