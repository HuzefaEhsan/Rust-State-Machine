mod balances;
mod proof_of_existence;
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
	// The content of a claim, represented as a vector of bytes.
	pub type Content = Vec<u8>;

	pub type Extrinsic = crate::support::Extrinsic<AccountId, crate::RuntimeCall>;
	pub type Header = crate::support::Header<BlockNumber>;
	pub type Block = crate::support::Block<Header, Extrinsic>;
}

/// An enum representing all possible external calls to the runtime.
/// Each variant holds the respective pallet's `Call` enum.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeCall {
	Balances(balances::Call<Runtime>),
	ProofOfExistence(proof_of_existence::Call<Runtime>),
}

/// The main runtime struct, which aggregates all pallets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Runtime {
	system: system::Pallet<Self>,
	balances: balances::Pallet<Self>,
	proof_of_existence: proof_of_existence::Pallet<Self>,
}

/// Implements the `system::Config` trait for the `Runtime`.
impl system::Config for Runtime {
	type AccountId = types::AccountId;
	type BlockNumber = types::BlockNumber;
	type Nonce = types::Nonce;
}

/// Implements the `balances::Config` trait for the `Runtime`.
impl balances::Config for Runtime {
	type Balance = types::Balance;
}

/// Implements the `proof_of_existence::Config` trait for the `Runtime`.
impl proof_of_existence::Config for Runtime {
	type Content = types::Content;
}

impl Runtime {
	/// Constructs a new instance of the runtime.
	fn new() -> Self {
		Self {
			system: system::Pallet::<Self>::new(),
			balances: balances::Pallet::<Self>::new(),
			proof_of_existence: proof_of_existence::Pallet::<Self>::new(),
		}
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
			RuntimeCall::ProofOfExistence(call) => {
				self.proof_of_existence.dispatch(caller, call)?;
			},
		}
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

	// Construct block 1.
	let block_1 = types::Block {
		header: support::Header { block_number: 1 },
		extrinsics: vec![
			support::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::Balances(balances::Call::Transfer {
					to: bob.clone(),
					amount: 30,
				}),
			},
			support::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::Balances(balances::Call::Transfer {
					to: charlie.clone(),
					amount: 20,
				}),
			},
		],
	};

	// Construct block 2, which includes Proof of Existence extrinsics.
	let block_2 = types::Block {
		header: support::Header { block_number: 2 },
		extrinsics: vec![
			support::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::CreateClaim {
					claim: b"hello_world".to_vec(),
				}),
			},
			support::Extrinsic {
				caller: bob.clone(),
				call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::CreateClaim {
					claim: b"hello_world".to_vec(),
				}),
			},
		],
	};

	// Construct block 3 to test revocation.
	let block_3 = types::Block {
		header: support::Header { block_number: 3 },
		extrinsics: vec![
			support::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::RevokeClaim {
					claim: b"hello_world".to_vec(),
				}),
			},
			support::Extrinsic {
				caller: bob.clone(),
				call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::CreateClaim {
					claim: b"hello_world".to_vec(),
				}),
			},
		],
	};

	// Execute the blocks.
	runtime.execute_block(block_1).expect("invalid block");
	runtime.execute_block(block_2).expect("invalid block");
	runtime.execute_block(block_3).expect("invalid block");

	// Print the final runtime state.
	println!("{:#?}", runtime);
}
