extern crate crt_eg;

use std::env;
use std::thread;
use std::sync::mpsc;
use std::time::Instant;
use num::bigint::{BigInt, ToBigInt};

fn main() {
	let args: Vec<String> = env::args().collect();

	assert!(
		args.len() == 3,
		"Must supply exactly two arguments! (You gave: {})",
		env::args().len() - 1
	);

	println!("Moduli:   \t{}.", args[1]);
	let n_i = crt_eg::parse(&args[1]);

	println!("Congruents:\t{}.", args[2]);
	let a_i = crt_eg::parse(&args[2]);

	assert!(
		a_i.len() == n_i.len(),
		"You must supply an equal number of moduli and congruents. ({} != {})",
		n_i.len(),
		a_i.len()
	);

	// compute the product of the big numbers
	let big_n: BigInt = n_i.iter().fold(1.to_bigint().unwrap(), |acc, x| acc * x);

	println!("Product of all n_i: {}", big_n);

	let mut threads = Vec::new();

	let (tx, rx) = mpsc::channel();

	let timer = Instant::now();

	for idx in 0..a_i.len() {
		let local_tx = tx.clone();
		let big_n = big_n.clone();
		let n_i = n_i.clone();
		let a_i = a_i.clone();

		threads.push(thread::spawn(move || {
			let start = Instant::now();
			local_tx.send(crt_eg::solution(&big_n, &n_i[idx], &a_i[idx])).unwrap();
			println!("Thread {} completed its work in {}ns", idx, start.elapsed().as_nanos());
		}));
	}

	for thread in threads {
		thread.join().unwrap();
	}

	let solution: Vec<BigInt> = rx.try_iter().collect();
	let solution: BigInt = solution.iter().sum();
	let solution = crt_eg::gate_in_mod(&solution, &big_n);

	println!("Found solution in {}ns: {} + {} n for n \u{2208} \u{2115}", timer.elapsed().as_nanos(), &solution % &big_n, big_n);

	crt_eg::prove_solution(&solution, &n_i, &a_i);
}
