use num::traits::{CheckedAdd, CheckedSub, Zero};
use std::{collections::BTreeMap, marker::PhantomData};

/// Configuration trait for the Balances pallet.
/// This pallet is tightly coupled to the System pallet by inheriting its configuration.
pub trait Config: crate::system::Config {
	/// The type used to represent the balance of an account.
	type Balance: Zero + CheckedSub + CheckedAdd + Copy;
}

/// The Balances pallet, for managing account balances.
#[derive(Debug)]
pub struct Pallet<T: Config> {
	// A mapping from account IDs to their balances.
	balances: BTreeMap<T::AccountId, T::Balance>,
	_phantom: PhantomData<T>,
}

impl<T: Config> Pallet<T> {
	/// Constructs a new instance of the balances module.
	pub fn new() -> Self {
		Self { balances: BTreeMap::new(), _phantom: PhantomData }
	}

	/// Set the balance of an account `who` to some `amount`.
	pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
		self.balances.insert(who.clone(), amount);
	}

	/// Get the balance of an account `who`.
	/// If the account has no stored balance, returns zero.
	pub fn balance(&self, who: &T::AccountId) -> T::Balance {
		*self.balances.get(who).unwrap_or(&T::Balance::zero())
	}

	/// Transfer `amount` from one account to another.
	/// This function verifies that `from` has at least `amount` balance to transfer,
	/// and that no mathematical overflows occur.
	pub fn transfer(
		&mut self,
		caller: T::AccountId,
		to: T::AccountId,
		amount: T::Balance,
	) -> Result<(), &'static str> {
		let caller_balance = self.balance(&caller);
		let to_balance = self.balance(&to);

		let new_caller_balance = caller_balance.checked_sub(&amount).ok_or("Not enough funds.")?;
		let new_to_balance = to_balance.checked_add(&amount).ok_or("Overflow")?;

		self.balances.insert(caller, new_caller_balance);
		self.balances.insert(to, new_to_balance);

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use crate::system;

	// Mock struct for testing purposes.
	struct TestConfig;

	// The System pallet's `Config` is a dependency for the Balances `Config`.
	impl system::Config for TestConfig {
		type AccountId = String;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	// Implement the Balances pallet's `Config` for the test struct.
	impl crate::balances::Config for TestConfig {
		type Balance = u128;
	}

	#[test]
	fn init_balances() {
		let mut balances = crate::balances::Pallet::<TestConfig>::new();

		assert_eq!(balances.balance(&"alice".to_string()), 0);
		balances.set_balance(&"alice".to_string(), 100);
		assert_eq!(balances.balance(&"alice".to_string()), 100);
		assert_eq!(balances.balance(&"bob".to_string()), 0);
	}

	#[test]
	fn transfer_balance() {
		let mut balances = crate::balances::Pallet::<TestConfig>::new();

		assert_eq!(
			balances.transfer("alice".to_string(), "bob".to_string(), 51),
			Err("Not enough funds.")
		);

		balances.set_balance(&"alice".to_string(), 100);
		assert_eq!(balances.transfer("alice".to_string(), "bob".to_string(), 51), Ok(()));
		assert_eq!(balances.balance(&"alice".to_string()), 49);
		assert_eq!(balances.balance(&"bob".to_string()), 51);

		assert_eq!(
			balances.transfer("alice".to_string(), "bob".to_string(), 51),
			Err("Not enough funds.")
		);
	}
}
