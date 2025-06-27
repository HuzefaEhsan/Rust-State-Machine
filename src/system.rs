use num::traits::One;
use std::collections::BTreeMap;

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet<AccountId, BlockNumber, Nonce> {
	/// The current block number.
	block_number: BlockNumber,
	/// A map from an account to their nonce.
	nonce: BTreeMap<AccountId, Nonce>,
}

impl<AccountId, BlockNumber, Nonce> Pallet<AccountId, BlockNumber, Nonce>
where
	AccountId: Ord + Clone,
	// CORRECTED TRAIT BOUNDS: Added `One` and `AddAssign` for robust incrementing.
	BlockNumber: From<u8> + One + std::ops::AddAssign + Copy,
	Nonce: From<u8> + One + std::ops::Add<Output = Nonce> + Copy,
{
	/// Create a new instance of the System Pallet.
	pub fn new() -> Self {
		Self { block_number: BlockNumber::from(0), nonce: BTreeMap::new() }
	}

	/// Get the current block number.
	pub fn block_number(&self) -> BlockNumber {
		self.block_number
	}

	/// Get the nonce of an account.
	pub fn nonce(&self, who: &AccountId) -> Nonce {
		*self.nonce.get(who).unwrap_or(&Nonce::from(0))
	}

	/// This function can be used to increment the block number.
	/// Increases the block number by one.
	pub fn inc_block_number(&mut self) {
		// CORRECTED IMPLEMENTATION: Use the `One` trait for clarity and correctness.
		self.block_number += BlockNumber::one();
	}

	/// Increment the nonce of an account. This helps us keep track of how many transactions each
	/// account has made.
	pub fn inc_nonce(&mut self, who: &AccountId) {
		let nonce = self.nonce(who);
		let new_nonce = nonce + Nonce::one();
		self.nonce.insert(who.clone(), new_nonce);
	}
}

#[cfg(test)]
mod test {
	#[test]
	fn init_system() {
		// When creating an instance of `Pallet`, we explicitly define the types we use.
		let mut system = super::Pallet::<String, u32, u32>::new();
		system.inc_block_number();
		system.inc_nonce(&"alice".to_string());

		assert_eq!(system.block_number(), 1);
		assert_eq!(system.nonce(&"alice".to_string()), 1);
		assert_eq!(system.nonce(&"bob".to_string()), 0);
	}
}
