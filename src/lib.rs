extern crate num;

use num::bigint::{BigInt, ToBigInt};
use num::{Integer, Signed};

// Use an extended version of the Euclidean algorithm to get two coefficients
// in a tuple (s, t) such that a * s + b * t = gcd(a, b)
fn bezout_coeffs(a: &BigInt, b: &BigInt) -> (BigInt, BigInt) {
	let mut s: BigInt = 0.into();
	let mut old_s = 1.into();

	let mut r = b.clone();
	let mut old_r = a.clone();

	while r != 0.to_bigint().unwrap() {
		let quotient = &old_r / &r;

		let rt = r.clone();
		r = old_r - &quotient * r;
		old_r = rt;

		let st = s.clone();
		s = old_s - &quotient * s;
		old_s = st;
	}

	if b == &0.to_bigint().unwrap() {
		(old_s, 0.to_bigint().unwrap())
	} else {
		(old_s.clone(), (old_r - &old_s * a) / b)
	}
}

#[test]
fn bezout() {
	assert_eq!(bezout_coeffs(&2.into(), &3.into()), (BigInt::from(-1), BigInt::from(1)));
	assert_eq!(bezout_coeffs(&7.into(), &2.into()), (BigInt::from(1), BigInt::from(-3)));
}

pub fn solution(big_n: &BigInt, n_i: &BigInt, a_i: &BigInt) -> BigInt {
	// Verify that n_i \mid N
	assert!(big_n.is_multiple_of(n_i), "{} divides {}", n_i, big_n);

	// Verify that gcd(n_i, N / n_i) == 1.
	assert!(
		n_i.gcd(&(big_n / n_i)) == 1.to_bigint().unwrap(),
		"{} (n_i) and {} (N/n_i) must be coprime. (Check all moduli are pairwise coprime)",
		n_i,
		big_n / n_i
	);

	let bezout_coeffs = bezout_coeffs(&(big_n / n_i), n_i);

	a_i * bezout_coeffs.0 * &(big_n / n_i)
}

pub fn parse<Vu>(s: &str) -> Vec<Vu>
where
	Vu: std::str::FromStr,
	<Vu as std::str::FromStr>::Err: std::fmt::Debug,
{
	s.split(',').map(|x| x.parse::<Vu>().unwrap()).collect()
}

pub fn prove_solution(s: &BigInt, moduli: &[BigInt], congruents: &[BigInt]) {
	println!("For the skeptics, proving that {} is a solution:", s);

	for (modulus, congruent) in moduli.iter().zip(congruents) {
		if s % modulus == *congruent {
			println!("  {} % {} = {} == {} (expected)", s, modulus, s % modulus, congruent)
		} else {
			panic!("  Something's wrong.")
		}
	}
}

pub fn gate_in_mod(s: &BigInt, modulus: &BigInt) -> BigInt {
	if *s < 0.to_bigint().unwrap() {
		(s + modulus * (((0.to_bigint().unwrap() - s).abs() / modulus) + 1)) % modulus
	} else if s >= modulus {
		s % modulus
	} else {
		s.clone()
	}
}

#[test]
fn gate_works() {
	assert_eq!(gate_in_mod(&-6.to_bigint().unwrap(), &3.to_bigint().unwrap()), 0.to_bigint().unwrap());
	assert_eq!(gate_in_mod(&-5.to_bigint().unwrap(), &3.to_bigint().unwrap()), 1.to_bigint().unwrap());
	assert_eq!(gate_in_mod(&-4.to_bigint().unwrap(), &3.to_bigint().unwrap()), 2.to_bigint().unwrap());
	assert_eq!(gate_in_mod(&-3.to_bigint().unwrap(), &3.to_bigint().unwrap()), 0.to_bigint().unwrap());
	assert_eq!(gate_in_mod(&-2.to_bigint().unwrap(), &3.to_bigint().unwrap()), 1.to_bigint().unwrap());
	assert_eq!(gate_in_mod(&-1.to_bigint().unwrap(), &3.to_bigint().unwrap()), 2.to_bigint().unwrap());
	assert_eq!(gate_in_mod(&0.to_bigint().unwrap(), &3.to_bigint().unwrap()), 0.to_bigint().unwrap());
	assert_eq!(gate_in_mod(&1.to_bigint().unwrap(), &3.to_bigint().unwrap()), 1.to_bigint().unwrap());
	assert_eq!(gate_in_mod(&2.to_bigint().unwrap(), &3.to_bigint().unwrap()), 2.to_bigint().unwrap());
	assert_eq!(gate_in_mod(&3.to_bigint().unwrap(), &3.to_bigint().unwrap()), 0.to_bigint().unwrap());
	assert_eq!(gate_in_mod(&4.to_bigint().unwrap(), &3.to_bigint().unwrap()), 1.to_bigint().unwrap());
	assert_eq!(gate_in_mod(&5.to_bigint().unwrap(), &3.to_bigint().unwrap()), 2.to_bigint().unwrap());
	assert_eq!(gate_in_mod(&6.to_bigint().unwrap(), &3.to_bigint().unwrap()), 0.to_bigint().unwrap());
}
