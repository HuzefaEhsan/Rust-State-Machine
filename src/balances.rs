use crate::support::DispatchResult;
use num::traits::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;
use std::marker::PhantomData;

/// Configuration trait for the Balances pallet.
/// Tightly coupled to the System pallet by inheriting its configuration.
pub trait Config: crate::system::Config {
	/// The type used to represent the balance of an account.
	type Balance: Zero + CheckedAdd + CheckedSub + Copy;
}

/// The Balances pallet, for managing account balances.
#[derive(Debug)]
pub struct Pallet<T: Config> {
	// A mapping from account IDs to their balances.
	balances: BTreeMap<T::AccountId, T::Balance>,
	_phantom: PhantomData<T>,
}

impl<T: Config> Pallet<T> {
	/// Constructs a new instance of this pallet.
	pub fn new() -> Self {
		Self { balances: BTreeMap::new(), _phantom: PhantomData }
	}

	/// Set the balance of an account.
	pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
		self.balances.insert(who.clone(), amount);
	}

	/// Get the balance of an account.
	/// Returns zero if the account has no stored balance.
	pub fn balance(&self, who: &T::AccountId) -> T::Balance {
		*self.balances.get(who).unwrap_or(&T::Balance::zero())
	}

	/// Transfer `amount` from one account to another.
	pub fn transfer(
		&mut self,
		caller: T::AccountId,
		to: T::AccountId,
		amount: T::Balance,
	) -> DispatchResult {
		let caller_balance = self.balance(&caller);
		let to_balance = self.balance(&to);

		let new_caller_balance = caller_balance.checked_sub(&amount).ok_or("Not enough funds.")?;
		let new_to_balance = to_balance.checked_add(&amount).ok_or("Overflow")?;

		self.balances.insert(caller, new_caller_balance);
		self.balances.insert(to, new_to_balance);

		Ok(())
	}
}

/// An enum representing the dispatchable calls in the Balances pallet.
pub enum Call<T: Config> {
	/// A call to transfer funds from the caller to another account.
	Transfer { to: T::AccountId, amount: T::Balance },
}

/// Implementation of the dispatch logic for the Balances pallet.
impl<T: Config> crate::support::Dispatch for Pallet<T> {
	type Caller = T::AccountId;
	type Call = Call<T>;

	fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult {
		// Match the call variant and route to the appropriate function.
		match call {
			Call::Transfer { to, amount } => {
				self.transfer(caller, to, amount)?;
			},
		}
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use crate::{balances, system};

	// Mock struct for testing purposes.
	struct TestConfig;

	// The System pallet's `Config` is a dependency for the Balances `Config`.
	impl system::Config for TestConfig {
		type AccountId = String;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	// Implement the Balances pallet's `Config` for the test struct.
	impl balances::Config for TestConfig {
		type Balance = u128;
	}

	#[test]
	fn init_balances() {
		let mut balances = balances::Pallet::<TestConfig>::new();

		assert_eq!(balances.balance(&"alice".to_string()), 0);
		balances.set_balance(&"alice".to_string(), 100);
		assert_eq!(balances.balance(&"alice".to_string()), 100);
		assert_eq!(balances.balance(&"bob".to_string()), 0);
	}

	#[test]
	fn transfer_balance() {
		let mut balances = balances::Pallet::<TestConfig>::new();

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
