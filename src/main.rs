mod compiler;
mod interpreter;
mod node;
mod display;

//static PROGRAM: &'static str = r"
//fn fibonacci(n: u32) -> u32 {
//  let ~first = 1;
//  let ~second = 1;
//
//  let ~counter = 1;
//  loop counter == 1 => counter == n {
//    let summation = first + second;
//    first <=> second;
//    second <=> summation;
//
//    // `summation` contains the original `first`
//    drop summation = second - first;
//    counter += 1;
//  }
//
//  # Implicit drop of `first` and `counter`
//  drop n = counter;
//  second
//}
//";

fn main() {
	let _function = interpreter::construct();
}
