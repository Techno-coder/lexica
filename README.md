# Lexica
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

	drop n = second
	second
```

## Influence
Lexica has been influenced by:
- [Rust](https://github.com/rust-lang/rust)
- [Arrow](https://etd.ohiolink.edu/!etd.send_file?accession=oberlin1443226400)
- [Nim](https://nim-lang.org)

and many other languages.
