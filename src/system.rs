use num::traits::One;
use std::{collections::BTreeMap, marker::PhantomData};

/// The configuration trait for the System pallet.
/// This trait defines the associated types that the pallet depends on.
pub trait Config {
	/// The account identifier type.
	type AccountId: Ord + Clone;
	/// The block number type.
	type BlockNumber: From<u8> + One + std::ops::AddAssign + Copy;
	/// The nonce type.
	type Nonce: From<u8> + One + std::ops::Add<Output = Self::Nonce> + Copy;
}

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet<T: Config> {
	/// The current block number.
	block_number: T::BlockNumber,
	/// A map from an account to their nonce.
	nonce: BTreeMap<T::AccountId, T::Nonce>,
	/// A marker for the generic type `T`.
	_phantom: PhantomData<T>,
}

impl<T: Config> Pallet<T> {
	/// Create a new instance of the System Pallet.
	pub fn new() -> Self {
		Self {
			block_number: T::BlockNumber::from(0),
			nonce: BTreeMap::new(),
			_phantom: PhantomData,
		}
	}

	/// Get the current block number.
	pub fn block_number(&self) -> T::BlockNumber {
		self.block_number
	}

	/// Get the nonce of an account.
	pub fn nonce(&self, who: &T::AccountId) -> T::Nonce {
		*self.nonce.get(who).unwrap_or(&T::Nonce::from(0))
	}

	/// This function can be used to increment the block number.
	/// Increases the block number by one.
	pub fn inc_block_number(&mut self) {
		self.block_number += T::BlockNumber::one();
	}

	/// Increment the nonce of an account. This helps us keep track of how many transactions each
	/// account has made.
	pub fn inc_nonce(&mut self, who: &T::AccountId) {
		let nonce = self.nonce(who);
		let new_nonce = nonce + T::Nonce::one();
		self.nonce.insert(who.clone(), new_nonce);
	}
}

#[cfg(test)]
mod test {
	use super::{Config, Pallet};

	// Define a mock struct for testing purposes.
	struct TestConfig;

	// Implement the Config trait for the mock struct.
	impl Config for TestConfig {
		type AccountId = String;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	#[test]
	fn init_system() {
		// When creating an instance of `Pallet`, we use our mock config.
		let mut system = Pallet::<TestConfig>::new();
		system.inc_block_number();
		system.inc_nonce(&"alice".to_string());

		assert_eq!(system.block_number(), 1);
		assert_eq!(system.nonce(&"alice".to_string()), 1);
		assert_eq!(system.nonce(&"bob".to_string()), 0);
	}
}
