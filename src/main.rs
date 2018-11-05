extern crate rstd;
use rstd::prelude::*;

fn main() {
	let jmeno: String = prompt("zadej svoje jméno");
	println!("tvoje jméno je {:?}", jmeno.split_nonempty('a'));
}
