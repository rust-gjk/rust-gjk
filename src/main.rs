extern crate rstd;
use rstd::prelude::*;

fn main() {
	let mut vysledek: i64 = 0;	

	loop {
		let vstup: String = read();
		let seznam = vstup.split_nonempty(' ');

		match seznam[0].as_str() {
			"+" => {
				vysledek += seznam[1].parse::<i64>().unwrap();
				println!("{}", vysledek);
			},
			"-" => {
				vysledek += seznam[1].parse::<i64>().unwrap();
				println!("{}", vysledek);
			},
			"*" => {
				vysledek += seznam[1].parse::<i64>().unwrap();
				println!("{}", vysledek);
			},
			"/" => {
				vysledek += seznam[1].parse::<i64>().unwrap();
				println!("{}", vysledek);
			},
			x => println!("neznamy"),
		}
	}
}
