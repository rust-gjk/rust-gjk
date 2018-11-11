extern crate rstd;
use rstd::prelude::*;

fn main() {
	let mut i: i64 = 0;
	let delitel: i64 = prompt("zadejte dělitele");

	loop {
		if i == 100 { break; }
		if i % delitel == 0 {
			println!("{}", i);
		}
		i += 1;
	}
}
