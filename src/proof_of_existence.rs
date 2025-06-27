use crate::support::DispatchResult;
use core::fmt::Debug;
use std::{collections::BTreeMap, marker::PhantomData};

/// Configuration trait for the Proof of Existence pallet.
pub trait Config: crate::system::Config {
	/// The type that represents the content that can be claimed.
	type Content: Debug + Ord + Clone;
}

/// The Proof of Existence pallet.
/// Allows accounts to claim the existence of some data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pallet<T: Config> {
	/// A mapping from the content to the account that claimed it.
	claims: BTreeMap<T::Content, T::AccountId>,
	_phantom: PhantomData<T>,
}

impl<T: Config> Pallet<T> {
	/// Constructs a new instance of this pallet.
	pub fn new() -> Self {
		Self { claims: BTreeMap::new(), _phantom: PhantomData }
	}

	/// Get the owner of a claim, if it exists.
	pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
		self.claims.get(claim)
	}

	/// Create a new claim on behalf of the `caller`.
	///
	/// Returns an error if the claim has already been made.
	pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
		if self.claims.contains_key(&claim) {
			return Err("this content is already claimed");
		}
		self.claims.insert(claim, caller);
		Ok(())
	}

	/// Revoke an existing claim.
	///
	/// This function will return an error if the caller is not the owner of the
	/// claim, or if the claim does not exist.
	pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
		let owner = self.get_claim(&claim).ok_or("claim does not exist")?;
		if *owner != caller {
			return Err("not the owner of the claim");
		}
		self.claims.remove(&claim);
		Ok(())
	}
}

/// An enum representing the dispatchable calls in this pallet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Call<T: Config> {
	/// Creates a new claim.
	CreateClaim { claim: T::Content },
	/// Revokes an existing claim.
	RevokeClaim { claim: T::Content },
}

/// Implementation of the dispatch logic for this pallet.
impl<T: Config> crate::support::Dispatch for Pallet<T> {
	type Caller = T::AccountId;
	type Call = Call<T>;

	fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult {
		// Match the call variant and route to the appropriate function.
		match call {
			Call::CreateClaim { claim } => {
				self.create_claim(caller, claim)?;
			},
			Call::RevokeClaim { claim } => {
				self.revoke_claim(caller, claim)?;
			},
		}
		Ok(())
	}
}

#[cfg(test)]
mod test {
	use super::{Config, Pallet};
	use crate::system;

	// Mock struct for testing purposes.
	struct TestConfig;

	impl Config for TestConfig {
		type Content = &'static str;
	}

	impl system::Config for TestConfig {
		type AccountId = &'static str;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	#[test]
	fn basic_proof_of_existence() {
		let mut poe = Pallet::<TestConfig>::new();
		assert_eq!(poe.get_claim(&"Hello, world!"), None);
		assert_eq!(poe.create_claim("alice", "Hello, world!"), Ok(()));
		assert_eq!(poe.get_claim(&"Hello, world!"), Some(&"alice"));
		assert_eq!(
			poe.create_claim("bob", "Hello, world!"),
			Err("this content is already claimed")
		);
		assert_eq!(poe.revoke_claim("alice", "Hello, world!"), Ok(()));
		assert_eq!(poe.create_claim("bob", "Hello, world!"), Ok(()));
	}
}
