use num::traits::One;
use std::{collections::BTreeMap, marker::PhantomData};

/// Configuration trait for the System pallet.
pub trait Config {
	/// The type used to identify a user account.
	type AccountId: Ord + Clone;
	/// The type used to represent a block number.
	type BlockNumber: From<u8> + One + std::ops::AddAssign + Copy;
	/// The type used to represent a transaction number.
	type Nonce: From<u8> + One + std::ops::Add<Output = Self::Nonce> + Copy;
}

/// The System pallet, for managing core system state.
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
	/// Constructs a new instance of this pallet.
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

	/// Increments the block number by one.
	pub fn inc_block_number(&mut self) {
		self.block_number += T::BlockNumber::one();
	}

	/// Increments the nonce of an account.
	pub fn inc_nonce(&mut self, who: &T::AccountId) {
		let nonce = self.nonce(who);
		let new_nonce = nonce + T::Nonce::one();
		self.nonce.insert(who.clone(), new_nonce);
	}
}

#[cfg(test)]
mod test {
	use super::{Config, Pallet};

	// Mock struct for testing purposes.
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
