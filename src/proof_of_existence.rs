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
	/// A marker for the generic type `T`.
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
}

/// The dispatchable functions of the Proof of Existence pallet.
#[macros::call]
impl<T: Config> Pallet<T>
where
	T: Config,
{
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

#[cfg(test)]
mod test {
	use crate::{proof_of_existence as poe, system};

	// Mock struct for testing purposes.
	struct TestConfig;

	impl poe::Config for TestConfig {
		type Content = &'static str;
	}

	impl system::Config for TestConfig {
		type AccountId = &'static str;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	#[test]
	fn basic_proof_of_existence() {
		let mut poe_pallet = poe::Pallet::<TestConfig>::new();
		assert_eq!(poe_pallet.get_claim(&"Hello, world!"), None);
		assert_eq!(poe_pallet.create_claim("alice", "Hello, world!"), Ok(()));
		assert_eq!(poe_pallet.get_claim(&"Hello, world!"), Some(&"alice"));
		assert_eq!(
			poe_pallet.create_claim("bob", "Hello, world!"),
			Err("this content is already claimed")
		);
		assert_eq!(poe_pallet.revoke_claim("alice", "Hello, world!"), Ok(()));
		assert_eq!(poe_pallet.create_claim("bob", "Hello, world!"), Ok(()));
	}
}
