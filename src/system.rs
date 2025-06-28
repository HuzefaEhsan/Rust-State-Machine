use core::ops::AddAssign;
use num::traits::{One, Zero};
use std::{collections::BTreeMap, marker::PhantomData};

/// The configuration trait for the System pallet.
/// Defines the common types used throughout the state machine.
pub trait Config {
	/// The type used to identify a user account.
	type AccountId: Ord + Clone;
	/// The type used to represent the current block number.
	type BlockNumber: Zero + One + AddAssign + Copy;
	/// The type used to represent the number of transactions from an account.
	type Nonce: Zero + One + AddAssign + Copy;
}

/// The System pallet, for managing low-level state of the blockchain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pallet<T: Config> {
	/// The current block number.
	block_number: T::BlockNumber,
	/// A map from an account to their nonce.
	nonce: BTreeMap<T::AccountId, T::Nonce>,
	/// A marker for the generic type `T`.
	_phantom: PhantomData<T>,
}

impl<T: Config> Pallet<T> {
	/// Constructs a new instance of the System pallet.
	pub fn new() -> Self {
		Self { block_number: T::BlockNumber::zero(), nonce: BTreeMap::new(), _phantom: PhantomData }
	}

	/// Get the current block number.
	pub fn block_number(&self) -> T::BlockNumber {
		self.block_number
	}

	/// Get the nonce of an account.
	pub fn nonce(&self, who: &T::AccountId) -> T::Nonce {
		*self.nonce.get(who).unwrap_or(&T::Nonce::zero())
	}

	/// Increments the block number by one.
	pub fn inc_block_number(&mut self) {
		self.block_number += T::BlockNumber::one();
	}

	/// Increments the nonce of an account.
	pub fn inc_nonce(&mut self, who: &T::AccountId) {
		let nonce = self.nonce.entry(who.clone()).or_insert(T::Nonce::zero());
		*nonce += T::Nonce::one();
	}
}

#[cfg(test)]
mod test {
	use super::{Config, Pallet};

	// Mock struct for testing purposes.
	struct TestConfig;

	impl Config for TestConfig {
		type AccountId = String;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	#[test]
	fn init_system() {
		let mut system = Pallet::<TestConfig>::new();
		system.inc_block_number();
		system.inc_nonce(&"alice".to_string());

		assert_eq!(system.block_number(), 1);
		assert_eq!(system.nonce(&"alice".to_string()), 1);
		assert_eq!(system.nonce(&"bob".to_string()), 0);
	}
}
