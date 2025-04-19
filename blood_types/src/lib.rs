#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
	A,
	AB,
	B,
	O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
	Positive,
	Negative,
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
	pub antigen: Antigen,
	pub rh_factor: RhFactor,
}

use std::cmp::{Ord, Ordering};

use std::str::FromStr;

impl FromStr for Antigen {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"A" => Ok(Antigen::A),
			"B" => Ok(Antigen::B),
			"AB" => Ok(Antigen::AB),
			"O" => Ok(Antigen::O),
			_ => Err(()),
		}
	}
}

impl FromStr for RhFactor {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"+" => Ok(RhFactor::Positive),
			"-" => Ok(RhFactor::Negative),
			_ => Err(()),
		}
	}
}

impl FromStr for BloodType {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (antigen_part, rh_part) = if s.ends_with('+') {
			(&s[..s.len()-1], "+")
		} else if s.ends_with('-') {
			(&s[..s.len()-1], "-")
		} else {
			return Err(());
		};

		let antigen = antigen_part.parse()?;
		let rh_factor = rh_part.parse()?;

		Ok(BloodType { antigen, rh_factor })
	}
}

impl Ord for BloodType {
	fn cmp(&self, other: &Self) -> Ordering {
		self.antigen.cmp(&other.antigen).then(self.rh_factor.cmp(&other.rh_factor))
	}
}


use std::fmt::{self, Debug};

impl Debug for BloodType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let antigen_str = match &self.antigen {
			Antigen::A => "A",
			Antigen::B => "B",
			Antigen::AB => "AB",
			Antigen::O => "O",
		};

		let rh_str = match &self.rh_factor {
			RhFactor::Positive => "+",
			RhFactor::Negative => "-",
		};

		write!(f, "{}{}", antigen_str, rh_str)
	}
}


impl BloodType {
	pub fn can_receive_from(&self, other: &Self) -> bool {
		let antigen_compat = match (&self.antigen, &other.antigen) {
			(Antigen::AB, _) => true,
			(Antigen::A, Antigen::A) | (Antigen::A, Antigen::O) => true,
			(Antigen::B, Antigen::B) | (Antigen::B, Antigen::O) => true,
			(Antigen::O, Antigen::O) => true,
			_ => false,
		};

		let rh_compat = match (&self.rh_factor, &other.rh_factor) {
			(RhFactor::Positive, _) => true,
			(RhFactor::Negative, RhFactor::Negative) => true,
			_ => false,
		};

		antigen_compat && rh_compat
	}

	pub fn donors(&self) -> Vec<Self> {
		all_blood_types()
			.into_iter()
			.filter(|bt| self.can_receive_from(bt))
			.collect()
	}

	pub fn recipients(&self) -> Vec<Self> {
		all_blood_types()
			.into_iter()
			.filter(|bt| bt.can_receive_from(self))
			.collect()
	}
}
fn all_blood_types() -> Vec<BloodType> {
	use Antigen::*;
	use RhFactor::*;
	let mut types = vec![];

	for antigen in [O, A, B, AB] {
		for rh in [Negative, Positive] {
			types.push(BloodType {
				antigen: antigen.clone(),
				rh_factor: rh.clone(),
			});
		}
	}
	types
}
