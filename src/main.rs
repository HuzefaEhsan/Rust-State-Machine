mod balances;
mod support;
mod system;

use support::Dispatch;

/// Concrete types used throughout the runtime.
mod types {
	// The user-facing account identifier.
	pub type AccountId = String;
	// The balance of an account.
	pub type Balance = u128;
	// The block number.
	pub type BlockNumber = u32;
	// The transaction number of an account.
	pub type Nonce = u32;

	// A concrete `Extrinsic` type for this runtime.
	pub type Extrinsic = crate::support::Extrinsic<AccountId, crate::RuntimeCall>;
	// A concrete `Header` type for this runtime.
	pub type Header = crate::support::Header<BlockNumber>;
	// A concrete `Block` type for this runtime.
	pub type Block = crate::support::Block<Header, Extrinsic>;
}

/// An enum representing all possible external calls to the runtime.
/// Each variant holds the respective pallet's `Call` enum.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeCall {
	Balances(balances::Call<Runtime>),
}

/// The main runtime struct, which aggregates all pallets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Runtime {
	system: system::Pallet<Self>,
	balances: balances::Pallet<Self>,
}

/// Implements the `system::Config` trait for the `Runtime`.
impl system::Config for Runtime {
	type AccountId = types::AccountId;
	type BlockNumber = types::BlockNumber;
	type Nonce = types::Nonce;
}

/// Implements the `balances::Config` trait for the `Runtime`.
impl balances::Config for Runtime {
	// `AccountId` is inherited from `system::Config`.
	type Balance = types::Balance;
}

impl Runtime {
	/// Constructs a new instance of the runtime.
	fn new() -> Self {
		Self { system: system::Pallet::<Self>::new(), balances: balances::Pallet::<Self>::new() }
	}

	/// Executes a block of extrinsics.
	fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
		self.system.inc_block_number();
		if block.header.block_number != self.system.block_number() {
			return Err("block number does not match what is expected");
		}

		for (i, support::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
			self.system.inc_nonce(&caller);
			let _res = self.dispatch(caller, call).map_err(|e| {
				eprintln!(
					"Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Index: {}\n\tError: {}",
					block.header.block_number, i, e
				)
			});
		}
		Ok(())
	}
}

impl support::Dispatch for Runtime {
	type Caller = <Runtime as system::Config>::AccountId;
	type Call = RuntimeCall;

	/// Dispatches a call on behalf of a caller.
	fn dispatch(
		&mut self,
		caller: Self::Caller,
		runtime_call: Self::Call,
	) -> support::DispatchResult {
		// Match on the outer `RuntimeCall` enum and dispatch the inner `Call`
		// to the appropriate pallet.
		match runtime_call {
			RuntimeCall::Balances(call) => {
				self.balances.dispatch(caller, call)?;
			},
		}
		// Return `Ok` if the dispatch was successful.
		Ok(())
	}
}

fn main() {
	// Create a new instance of the Runtime.
	let mut runtime = Runtime::new();
	let alice = "alice".to_string();
	let bob = "bob".to_string();
	let charlie = "charlie".to_string();

	// Initialize the system with some initial balance.
	runtime.balances.set_balance(&alice, 100);

	// Construct the block to be executed.
	let block_1 = types::Block {
		header: support::Header { block_number: 1 },
		extrinsics: vec![
			support::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::Balances(balances::Call::Transfer { to: bob, amount: 30 }),
			},
			support::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::Balances(balances::Call::Transfer { to: charlie, amount: 20 }),
			},
		],
	};

	// Execute the block.
	runtime.execute_block(block_1).expect("invalid block");

	// Print the final runtime state.
	println!("{:#?}", runtime);

	// Verify the final state.
	assert_eq!(runtime.system.block_number(), 1);
	assert_eq!(runtime.system.nonce(&alice), 2);
	assert_eq!(runtime.balances.balance(&alice), 50);
}
